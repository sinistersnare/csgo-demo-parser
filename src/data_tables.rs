//! Wow this is an ugly file. Just wanted to put that up here at the top.
//! I should split it into data_tables.rs and string_tables.rs

use anyhow::Context;
use serde::Serialize;

use crate::cursor::Cursor;
use crate::message::Message;
use crate::packet;
use crate::protos::csvc_msg_send_table::SendpropT;
use crate::protos::CsvcMsgSendTable;

#[derive(Debug, Serialize)]
pub struct DataTable {
    data_tables: Vec<CsvcMsgSendTable>,
    pub server_classes: Vec<ServerClass>,
    service_class_bits: u8,
}

impl DataTable {
    fn gather_excludes(
        send_table: &CsvcMsgSendTable,
        all_tables: &[CsvcMsgSendTable],
    ) -> anyhow::Result<Vec<SendpropT>> {
        let mut excludes = vec![];
        for prop in &send_table.props {
            let flags = PropFlags::from_bits(prop.flags() as u32).context("Bad prop flags")?;
            if flags.contains(PropFlags::EXCLUDE) {
                // TODO: make this a ref.
                excludes.push(prop.clone())
            }
            if prop.r#type() == (PropTypes::DataTable as i32) {
                let sub_table = Self::find_by_name(all_tables, prop.dt_name())
                    .context("No Table Found with name.")?;
                let inner_excludes = Self::gather_excludes(sub_table, all_tables)?;
                excludes.extend(inner_excludes);
            }
        }
        Ok(excludes)
    }

    fn find_by_name<'a>(
        all_tables: &'a [CsvcMsgSendTable],
        name: &str,
    ) -> Option<&'a CsvcMsgSendTable> {
        all_tables.iter().find(|t| t.net_table_name() == name)
    }

    pub fn parse(cursor: &Cursor) -> anyhow::Result<DataTable> {
        let mut data_tables = vec![];
        // Start by parsing SendTable messages until one has is_end.
        loop {
            let msg = packet::parse_message(cursor)?;
            match msg {
                Message::SendTable(st) => {
                    if st.is_end() {
                        break;
                    } else {
                        data_tables.push(st);
                    }
                }
                _ => anyhow::bail!("Only SendTable messages allowed."),
            }
        }

        let server_class_count = cursor.read_i16()?;
        let mut server_classes = Vec::with_capacity(server_class_count as usize);
        for _ in 0..server_class_count {
            let sc = ServerClass::parse(cursor, server_class_count)?;
            server_classes.push(sc);
        }

        for sc in &mut server_classes {
            let send_table =
                Self::find_by_name(&data_tables, &sc.owning_name).context("No owning table.")?;
            let excludes = DataTable::gather_excludes(send_table, &data_tables)?;
            sc.fill_props(&excludes, &data_tables)?;
            // let props = Self::gather_props(send_table, &data_tables, sc,
            // &excludes, String::new())?; let flat =
            // flatten(send_table, &data_tables, sc)?;
        }

        let mut iter = server_class_count;
        let mut service_class_bits = 1;
        loop {
            iter >>= 1;
            if iter > 0 {
                service_class_bits += 1;
            } else {
                break;
            }
        }

        // flatten_data_table(server_class_count, &mut current_excludes)?;

        Ok(DataTable {
            data_tables,
            server_classes,
            service_class_bits,
        })
    }
}

fn is_prop_excluded(
    send_table: &CsvcMsgSendTable,
    prop: &SendpropT,
    excludes: &[SendpropT],
) -> bool {
    excludes
        .iter()
        .any(|e| e.dt_name() == send_table.net_table_name() && e.var_name() == prop.var_name())
}

// TODO: should this be named ServiceClass?
// TODO: take &'a str.
#[derive(Debug, Serialize)]
pub struct ServerClass {
    /// TODO: what is this exactly.
    class_id: i16,
    /// The name of this ServerClass
    name: String,
    /// The name of the owning SendTable.
    owning_name: String,
    pub props: Vec<Prop>,
    array_props: Vec<Prop>,
}

#[derive(Debug, Serialize)]
pub struct Prop {
    inner: SendpropT,
    path: String,
}

impl Prop {
    pub fn new(inner: SendpropT, path: String) -> Prop {
        Prop { inner, path }
    }
}

impl ServerClass {
    fn gather_props(
        send_table: &CsvcMsgSendTable,
        all_tables: &[CsvcMsgSendTable],
        excludes: &[SendpropT],
        path: String,
    ) -> anyhow::Result<(Vec<Prop>, Vec<Prop>)> {
        let mut store = Vec::with_capacity(send_table.props.len());
        let mut arr_store = vec![];

        for i in 0..send_table.props.len() {
            let prop = &send_table.props[i];
            let flags = PropFlags::from_bits(prop.flags() as u32).context("Bad Flag bits.")?;
            if flags.contains(PropFlags::INSIDE_ARRAY)
                || flags.contains(PropFlags::EXCLUDE)
                || is_prop_excluded(send_table, prop, excludes)
            {
                continue;
            }

            let prop_path = {
                let mut p = String::new();
                if prop.var_name() != "baseclass" {
                    p += prop.var_name();
                }
                if !p.is_empty() && !path.is_empty() {
                    p = path.clone() + "." + &p;
                }
                p
            };

            match PropTypes::from_i32(prop.r#type.unwrap_or(-1))? {
                PropTypes::Array => {
                    // TODO: C++ code pushes the path with it... Necessary?
                    // Also how does the i-1 work here... everyone does it I guess...
                    arr_store.push(Prop::new(
                        send_table.props[i - 1].clone(),
                        prop_path.clone(),
                    ));
                    store.push(Prop::new(prop.clone(), prop_path));
                }
                PropTypes::DataTable => {
                    let table = DataTable::find_by_name(all_tables, prop.dt_name())
                        .context("No table with name.")?;
                    // TODO:
                    // Other impls branch if this is collapsible... and then do basically the same
                    // thing (AIUI) in both cases! WHY!?!!?
                    let (new_store, new_arr_store) =
                        Self::gather_props(table, all_tables, excludes, prop_path)?;
                    store.extend(new_store);
                    arr_store.extend(new_arr_store);
                }
                _ => store.push(Prop::new(prop.clone(), prop_path)),
            }
        }
        Ok((store, arr_store))
    }

    pub fn fill_props(
        &mut self,
        excludes: &[SendpropT],
        tables: &[CsvcMsgSendTable],
    ) -> anyhow::Result<()> {
        let owning_table =
            DataTable::find_by_name(tables, &self.owning_name).context("No DataTable found.")?;
        let (props, array_props) =
            Self::gather_props(owning_table, tables, excludes, String::new())?;
        self.props = props;
        self.array_props = array_props;
        Ok(())
    }

    pub fn parse(cursor: &Cursor, class_count: i16) -> anyhow::Result<ServerClass> {
        let class_id = cursor.read_i16()?;
        if class_id > class_count {
            anyhow::bail!("Invalid class index... {class_id} > {class_count}");
        }

        let name = cursor.read_cstr_until()?.to_string();
        let owning_name = cursor.read_cstr_until()?.to_string();

        let props = vec![];
        let array_props = vec![];
        Ok(ServerClass {
            class_id,
            name,
            owning_name,
            props,
            array_props,
        })
    }
}

/// An enumeration that is used to detect the type.
/// Just cast this to i32 to get the value as its used in the protobuf.
enum PropTypes {
    Int = 0,
    Float,
    Vector,
    VectorXY, // Only encodes the XY of a vector, ignores Z
    String,
    Array, // An array of the base types (can't be of datatables).
    DataTable,
    Int64,
    NUMSendPropTypes,
}

impl PropTypes {
    pub fn from_i32(i: i32) -> anyhow::Result<PropTypes> {
        Ok(match i {
            0 => PropTypes::Int,
            1 => PropTypes::Float,
            2 => PropTypes::Vector,
            3 => PropTypes::VectorXY,
            4 => PropTypes::String,
            5 => PropTypes::Array,
            6 => PropTypes::DataTable,
            7 => PropTypes::Int64,
            8 => PropTypes::NUMSendPropTypes,
            _ => anyhow::bail!("Bad Prop Type value `{i}`."),
        })
    }
}

bitflags::bitflags! {
    struct PropFlags: u32 {
        /// // Unsigned integer data.
        const UNSIGNED = 0b00000000000000000001;
        /// If this is set, the float/vector is treated like a world coordinate. Note that the bit count is ignored in this case.
        const COORD = 0b00000000000000000010;
        /// For floating point, don't scale into range, just take value as is.
        const NO_SCALE = 0b00000000000000000100;
        /// For floating point, limit high value to range minus one bit unit
        const ROUND_DOWN = 0b00000000000000001000;
        /// For floating point, limit low value to range minus one bit unit
        const ROUND_UP = 0b00000000000000010000;
        /// If this is set, the vector is treated like a normal (only valid for vectors)
        const NORMAL = 0b00000000000000100000;
        /// This is an exclude prop (not excludED, but it points at another prop to be excluded).
        const EXCLUDE = 0b00000000000001000000;
        /// Use XYZ/Exponent encoding for vectors.
        const XYZE = 0b00000000000010000000;
        /// This tells us that the property is inside an array, so it shouldn't be put into the flattened property list. Its array will point at it when it needs to.
        const INSIDE_ARRAY = 0b00000000000100000000;
        /// Set for datatable props using one of the default datatable proxies like SendProxy_DataTableToDataTable that always send the data to all clients.
        const PROXY_ALWAYS_YES = 0b00000000001000000000;
        /// Set automatically if SPROP_VECTORELEM is used.
        const IS_A_VECTOR_ELEM = 0b00000000010000000000;
        /// Set automatically if it's a datatable with an offset of 0 that doesn't change the pointer (ie: for all automatically-chained base classes).
        const COLLAPSIBLE = 0b00000000100000000000;
        /// Like SPROP_COORD, but special handling for multiplayer games
        const COORD_MP = 0b00000001000000000000;
        /// Like SPROP_COORD, but special handling for multiplayer games where the fractional component only gets a 3 bits instead of 5
        const COORD_MP_LOW_PRECISION = 0b00000010000000000000;
        /// SPROP_COORD_MP, but coordinates are rounded to integral boundaries
        const COORD_MP_INTEGRAL = 0b00000100000000000000;
        /// Like SPROP_COORD, but special encoding for cell coordinates that can't be negative, bit count indicate maximum value
        const CELL_COORD = 0b00001000000000000000;
        /// Like SPROP_CELL_COORD, but special handling where the fractional component only gets a 3 bits instead of 5
        const CELL_COORD_LOWPRECISION = 0b00010000000000000000;
        /// SPROP_CELL_COORD, but coordinates are rounded to integral boundaries
        const CELL_COORD_INTEGRAL = 0b00100000000000000000;
        /// this is an often changed field, moved to head of sendtable so it gets a small index
        const CHANGES_OFTEN = 0b01000000000000000000;
        /// use var int encoded (google protobuf style), note you want to include SPROP_UNSIGNED if needed, its more efficient
        const VARINT = 0b10000000000000000000;
    }
}

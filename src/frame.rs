use crate::cursor::Cursor;
use crate::packet::Packet;

struct StringTables<'a> {
    d: &'a [u8],
}

impl<'a> StringTables<'a> {
    pub fn new(raw: &'a [u8]) -> anyhow::Result<StringTables<'a>> {
        // let cursor = Cursor::new(raw);
        // let num_tables = cursor.read_u8()?;
        // let tables = Vec::with_capacity(num_tables as usize);
        // for n in 0..num_tables {}
        Ok(StringTables { d: raw })
    }
}

#[derive(Debug)]
pub enum Command<'a> {
    SignOn(Packet<'a>),
    Packet(Packet<'a>),
    SyncTick,
    ConsoleCmd(&'a [u8]),
    UserCmd(&'a [u8]),
    DataTables(&'a [u8]),
    Stop,
    CustomData,
    StringTables(&'a [u8]),
}

impl<'a> Command<'a> {
    pub fn new(which: u8, data: &Cursor) -> anyhow::Result<Command<'a>> {
        Ok(match which {
            1 => {
                let packet = Packet::new(data);
                Command::SignOn(packet)
            }
            2 => {
                let packet = Packet::new(data);
                Command::Packet(packet)
            }
            3 => Command::SyncTick,
            4 => {
                let length = data.read_i32()?;
                assert!(length > 0);
                let chunk = data.read_bytes(length as usize)?;
                Command::ConsoleCmd(chunk)
            }
            5 => {
                // wtf is this for
                let x = data.read_u32()?;
                let length = data.read_i32()?;
                let chunk = data.read_bytes(length as usize)?;
                Command::UserCmd(chunk)
            }
            6 => {
                // BitStream.BeginChunk(BitStream.ReadSignedInt(32) * 8);
                // SendTableParser.ParsePacket(BitStream);
                // BitStream.EndChunk();

                // //And now we have the entities, we can bind events on them.
                // BindEntites();
                let length = data.read_i32()?;
                let chunk = data.read_bytes(length as usize)?;
                // let table = DataTable::new(chunk)?;
                Command::DataTables(chunk)
            }
            7 => Command::Stop,
            8 => {
                // CustomData
                todo!()
            }
            9 => {
                // StringTables
                let length = data.read_i32()?;
                let chunk = data.read_bytes(length as usize)?;
                let table = StringTables::new(chunk)?;
                Command::StringTables(table)
            }
            n => anyhow::bail!("Bad command enumeration: {n}"),
        })
    }
}

#[derive(Debug)]
pub struct Frame<'a> {
    command: Command<'a>,
    tick_number: u32,
    playerslot: i8,
}

impl<'a> Frame<'a> {
    pub fn new(data: &'a Cursor<'a>) -> anyhow::Result<Frame<'a>> {
        let which_command = data.read_u8()?;
        let tick_number = data.read_u32()?;
        let playerslot = data.read_i8()?;
        let mut last_frame = false;

        let command = Command::new(which_command, data)?;

        Ok(Frame {
            command,
            tick_number,
            playerslot,
        })
    }
}

// fn parse_packet<'a>(data: &'a Cursor) -> Packet<'a> {}

use serde::Serialize;

use crate::cursor::Cursor;
use crate::data_tables::DataTable;
use crate::packet::Packet;
use crate::string_tables::StringTables;

#[derive(Debug, Serialize)]
pub enum Command {
    SignOn(Packet),
    Packet(Packet),
    SyncTick,
    ConsoleCmd(Vec<u8>),
    UserCmd(Vec<u8>),
    DataTables(DataTable),
    Stop,
    CustomData,
    StringTables(StringTables),
}

impl Command {
    pub fn new(which: u8, data: &Cursor) -> anyhow::Result<Command> {
        Ok(match which {
            1 => {
                let packet = Packet::new(data)?;
                Command::SignOn(packet)
            }
            2 => {
                let packet = Packet::new(data)?;
                Command::Packet(packet)
            }
            3 => Command::SyncTick,
            4 => {
                let length = data.read_i32()?;
                assert!(length > 0);
                let chunk = data.read_bytes(length as usize)?.into_owned();
                Command::ConsoleCmd(chunk)
            }
            5 => {
                // Unused by any parser.
                let _outgoing_sequence = data.read_i32()?;
                let length = data.read_i32()?;
                let chunk = data.read_bytes(length as usize)?.into_owned();
                Command::UserCmd(chunk)
            }
            6 => {
                // BitStream.BeginChunk(BitStream.ReadSignedInt(32) * 8);
                // SendTableParser.ParsePacket(BitStream);
                // BitStream.EndChunk();

                // //And now we have the entities, we can bind events on them.
                // BindEntites();
                let length = data.read_i32()?;
                let chunk = data.chunk_bytes(length as usize)?;
                let table = DataTable::parse(&chunk)?;
                Command::DataTables(table)
            }
            7 => Command::Stop,
            8 => Command::CustomData,
            9 => {
                // StringTables
                let length = data.read_i32()?;
                let chunk = data.chunk_bytes(length as usize)?;
                let table = StringTables::parse(chunk)?;
                Command::StringTables(table)
            }
            n => anyhow::bail!("Not sure how to suport command: {n}"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Frame {
    command: Command,
    tick_number: u32,
    playerslot: i8,
}

impl Frame {
    pub fn new(data: &Cursor) -> anyhow::Result<Frame> {
        let which_command = data.read_u8()?;
        let tick_number = data.read_u32()?;
        let playerslot = data.read_i8()?;

        let command = Command::new(which_command, data)?;

        Ok(Frame {
            command,
            tick_number,
            playerslot,
        })
    }

    pub(crate) fn is_last(&self) -> bool {
        matches!(self.command, Command::Stop)
    }
}

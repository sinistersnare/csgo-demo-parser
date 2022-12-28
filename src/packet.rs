use crate::cursor::Cursor;
use crate::message::{Message, MessageType};

#[derive(Debug)]
pub struct Split {
    flags: i32,
    /// A Vector3
    view_origin: (f32, f32, f32),
    /// A QAngle
    view_angles: (f32, f32, f32),
    /// A QAngle
    local_view_angles: (f32, f32, f32),

    /// A Vector3
    view_origin_2: (f32, f32, f32),
    /// A QAngle
    view_angles_2: (f32, f32, f32),
    /// A QAngle
    local_view_angles_2: (f32, f32, f32),
}

impl Split {
    pub fn new(data: &Cursor) -> anyhow::Result<Split> {
        fn read_triple_f32(data: &Cursor) -> anyhow::Result<(f32, f32, f32)> {
            let a = data.read_f32()?;
            let b = data.read_f32()?;
            let c = data.read_f32()?;
            Ok((a, b, c))
        }
        let flags = data.read_i32()?;
        let view_origin = read_triple_f32(data)?;
        let view_angles = read_triple_f32(data)?;
        let local_view_angles = read_triple_f32(data)?;
        let view_origin_2 = read_triple_f32(data)?;
        let view_angles_2 = read_triple_f32(data)?;
        let local_view_angles_2 = read_triple_f32(data)?;
        Ok(Split {
            flags, view_origin, view_angles, local_view_angles,
            view_origin_2, view_angles_2, local_view_angles_2
        })


    }
}

#[derive(Debug)]
pub struct CommandInfo {
    u: (Split, Split),
}

impl CommandInfo {
    pub fn new<'a>(data: &'a Cursor) -> anyhow::Result<CommandInfo> {
        let a = Split::new(data)?;
        let b = Split::new(data)?;
        Ok(CommandInfo {
            u: (a, b)
        })
    }
}



#[derive(Debug)]
pub struct Packet<'a> {
    command_info: CommandInfo,
    seq_nr_in: u32,
    seq_nr_out: u32,
    messages: Vec<Message<'a>>,
}

impl<'a> Packet<'a> {
    pub fn new(cursor: &'a Cursor) -> anyhow::Result<Packet<'a>> {
        let command_info = CommandInfo::new(cursor)?;
        let seq_nr_in = cursor.read_u32()?;
        let seq_nr_out = cursor.read_u32()?;
        let chunk_size = cursor.read_i32()?;
        let chunk = cursor.read_bytes(chunk_size as usize)?;
        let messages = parse_messages(&Cursor::new(chunk))?;
        Ok(Packet {
            command_info, seq_nr_in, seq_nr_out, messages,
        })
    }
}

fn parse_messages<'a>(outer_chunk: &'a Cursor) -> anyhow::Result<Vec<Message<'a>>> {
    let mut messages = vec![];
    // While we have data left, read!
    while outer_chunk.available() > 0 {
        let cmd = outer_chunk.read_protobuf_var_int()?;
        let length = outer_chunk.read_protobuf_var_int()?;
        let inner_chunk = Cursor::new(outer_chunk.read_bytes(length as usize)?);
        let msg = Message::new(&inner_chunk, MessageType::from_ordinal(cmd)?, length as u32)?;
        messages.push(msg);
    }
    Ok(messages)
}

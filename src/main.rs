//! A parser for CSGO .dem files
//! You can find demos from your own matches,
//! Or download them from some online source
//! Try https://www.hltv.org/matches/2359846/outsiders-vs-heroic-iem-rio-major-2022
//! in the 'rewatch' tab there is a GOTV demo link.

pub mod cursor;
mod frame;
pub mod message;
mod packet;
mod data_tables;
mod string_tables;
mod protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use cursor::Cursor;
use frame::Frame;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DemoHeader<'a> {
    /// Demo protocol version (stored in little endian)
    demo_protocol: i32,
    /// Network protocol version number (stored in little endian)
    network_protocol: u32,
    /// Max 259 characters (source is a 260 byte C-string)
    server_name: Cow<'a, str>,
    /// Max 259 characters (source is a 260 byte C-string)
    client_name: Cow<'a, str>,
    /// Max 259 characters (source is a 260 byte C-string)
    map_name: Cow<'a, str>,
    /// Max 259 characters (source is a 260 byte C-string)
    game_directory: Cow<'a, str>,
    /// The length of the demo, in seconds
    playback_time: f32,
    /// The number of ticks in the demo
    ticks: i32,
    /// The number of frames in the demo
    frames: i32,
    /// Length of the signon data (Init for first frame)
    sign_on_length: i32,
}

impl<'a> DemoHeader<'a> {
    pub fn new<'b: 'a>(data: &'b Cursor) -> anyhow::Result<DemoHeader<'a>> {
        assert_eq!(b"HL2DEMO\x00", data.read_bytes(8)?.as_ref());
        let demo_protocol = data.read_i32()?;
        let network_protocol = data.read_u32()?;
        let server_name = data.read_cstr(260)?;
        let client_name = data.read_cstr(260)?;
        let map_name = data.read_cstr(260)?;
        let game_directory = data.read_cstr(260)?;
        let playback_time = data.read_f32()?;
        let ticks = data.read_i32()?;
        let frames = data.read_i32()?;
        let sign_on_length = data.read_i32()?;
        Ok(DemoHeader {
            demo_protocol,
            network_protocol,
            server_name,
            client_name,
            map_name,
            game_directory,
            playback_time,
            ticks,
            frames,
            sign_on_length,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Demo<'a> {
    pub header: DemoHeader<'a>,
    pub frames: Vec<Frame>,
}

impl<'a> Demo<'a> {
    pub fn new(cursor: &'a Cursor) -> anyhow::Result<Demo<'a>> {
        let header = DemoHeader::new(cursor)?;
        let mut frames = Vec::new();
        for i in 0..header.frames {
            let frame = Frame::new(cursor)?;
            let is_last = frame.is_last();
            frames.push(frame);
            if is_last {
                println!("Got last frame {i}/{}", header.frames);
                break;
            }
        }
        Ok(Demo { header, frames })
    }
}

fn main() -> anyhow::Result<()> {
    let f = File::open("testdata/outsiders-vs-heroic-m1-mirage.dem")?;
    // TODO: how to do this streaming so it does not need to read the whole
    //       file in before processing?
    let mut buf = BufReader::new(f);
    let mut raw = Vec::new();
    buf.read_to_end(&mut raw)?;

    let cursor = Cursor::new(&raw);
    let demo = Demo::new(&cursor)?;

    let json = serde_json::to_string(&demo)?;

    let output = "OUT.json";
    let mut output = File::create(output)?;
    output.write_all(json.as_bytes())?;

    Ok(())
}

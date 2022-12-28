//! A parser for CSGO .dem files
//! You can find demos from your own matches,
//! Or download them from some online source
//! Try https://www.hltv.org/matches/2359846/outsiders-vs-heroic-iem-rio-major-2022
//! in the 'rewatch' tab there is a GOTV demo link.

#![feature(cstr_from_bytes_until_nul)]

pub mod cursor;
mod frame;
pub mod message;
mod packet;
mod protos;

use std::fs::File;
use std::io::{BufReader, Read};

use cursor::Cursor;
use frame::Frame;

/// TODO: can the strings here be utf-8 or just ascii??
/// Valve docs say wchar_t, so... utf8?
/// TODO: There MUST be a way to make the Strings `&'a str`s.
#[derive(Debug)]
pub struct DemoHeader<'a> {
    /// Demo protocol version (stored in little endian)
    demo_protocol: i32,
    /// Network protocol version number (stored in little endian)
    network_protocol: u32,
    /// Max 259 characters (source is a 260 byte C-string)
    server_name: &'a str,
    /// Max 259 characters (source is a 260 byte C-string)
    client_name: &'a str,
    /// Max 259 characters (source is a 260 byte C-string)
    map_name: &'a str,
    /// Max 259 characters (source is a 260 byte C-string)
    game_directory: &'a str,
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
    pub fn new<'b: 'a>(data: &'b Cursor<'a>) -> anyhow::Result<DemoHeader<'a>> {
        assert_eq!(b"HL2DEMO\x00", data.read_bytes(8)?);
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

#[derive(Debug)]
pub struct Demo<'a> {
    pub header: DemoHeader<'a>,
    pub frames: Vec<Frame<'a>>,
}

impl<'a> Demo<'a> {
    pub fn new(cursor: &'a Cursor) -> anyhow::Result<Demo<'a>> {
        let header = DemoHeader::new(cursor)?;
        let mut frames = Vec::new();
        for i in 0..header.frames {
            let frame = Frame::new(cursor)?;
            frames.push(frame);
            if i == 0 {
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

    println!("DEMO: {demo:#?}");

    Ok(())
}

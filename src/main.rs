//! A parser for CSGO .dem files
//! You can find demos from your own matches,
//! Or download them from some online source
//! Try https://www.hltv.org/matches/2359846/outsiders-vs-heroic-iem-rio-major-2022
//! in the 'rewatch' tab there is a GOTV demo link.

pub mod cursor;
mod data_tables;
mod demo;
mod frame;
pub mod message;
mod packet;
mod string_tables;
mod protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

use std::fs::File;
use std::io::{BufReader, Read, Write};

use cursor::Cursor;
use demo::Demo;

fn main() -> anyhow::Result<()> {
    let fname = {
        let arg = std::env::args().nth(1);
        if let Some(f) = arg {
            println!("Using file: {f:?}");
            f
        } else {
            println!("Using default...");
            "testdata/outsiders-vs-heroic-m1-mirage.dem".into()
        }
    };
    let f = File::open(fname)?;
    let mut buf = BufReader::new(f);
    let mut raw = Vec::new();
    buf.read_to_end(&mut raw)?;

    let cursor = Cursor::new(&raw);
    let demo = Demo::parse(&cursor)?;

    let json = serde_json::to_string_pretty(&demo)?;

    let output = "OUT.pretty.json";
    let mut output = File::create(output)?;
    output.write_all(json.as_bytes())?;

    Ok(())
}

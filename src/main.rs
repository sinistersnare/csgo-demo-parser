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

use clap::Parser;
use cursor::Cursor;
use demo::Demo;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The output file name.
    /// If the file exists, it will be overwritten,
    /// if the file does not exist, it will be created.
    #[arg(short, long, default_value = "OUT.json")]
    output: std::path::PathBuf,

    /// The input .dem file to operate on.
    input: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let fname = args.input;
    let f = File::open(fname)?;
    let mut buf = BufReader::new(f);
    let mut raw = Vec::new();
    buf.read_to_end(&mut raw)?;

    let cursor = Cursor::new(&raw);
    let demo = Demo::parse(&cursor)?;

    let json = serde_json::to_string_pretty(&demo)?;

    let output = args.output;
    let mut output = File::create(output)?;
    output.write_all(json.as_bytes())?;

    Ok(())
}

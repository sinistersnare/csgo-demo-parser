// Automatically generated rust module for 'engine_gcmessages.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CEngineGotvSyncPacket {
    pub match_id: Option<u64>,
    pub instance_id: Option<u32>,
    pub signupfragment: Option<u32>,
    pub currentfragment: Option<u32>,
    pub tickrate: Option<f32>,
    pub tick: Option<u32>,
    pub rtdelay: Option<f32>,
    pub rcvage: Option<f32>,
    pub keyframe_interval: Option<f32>,
    pub cdndelay: Option<u32>,
}

impl<'a> MessageRead<'a> for CEngineGotvSyncPacket {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.match_id = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.instance_id = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.signupfragment = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.currentfragment = Some(r.read_uint32(bytes)?),
                Ok(45) => msg.tickrate = Some(r.read_float(bytes)?),
                Ok(48) => msg.tick = Some(r.read_uint32(bytes)?),
                Ok(69) => msg.rtdelay = Some(r.read_float(bytes)?),
                Ok(77) => msg.rcvage = Some(r.read_float(bytes)?),
                Ok(85) => msg.keyframe_interval = Some(r.read_float(bytes)?),
                Ok(88) => msg.cdndelay = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CEngineGotvSyncPacket {
    fn get_size(&self) -> usize {
        0
        + self.match_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.instance_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.signupfragment.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.currentfragment.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tickrate.as_ref().map_or(0, |_| 1 + 4)
        + self.tick.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rtdelay.as_ref().map_or(0, |_| 1 + 4)
        + self.rcvage.as_ref().map_or(0, |_| 1 + 4)
        + self.keyframe_interval.as_ref().map_or(0, |_| 1 + 4)
        + self.cdndelay.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.match_id { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.instance_id { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.signupfragment { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.currentfragment { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.tickrate { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.tick { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.rtdelay { w.write_with_tag(69, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.rcvage { w.write_with_tag(77, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.keyframe_interval { w.write_with_tag(85, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.cdndelay { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}


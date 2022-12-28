//! A simple Cursor implementation
//! It helps me control the lifetimes easier.

use std::cell::Cell;
use std::ffi::CStr;

use byteorder::{ByteOrder, LittleEndian};

pub struct Cursor<'a> {
    data: &'a [u8],
    // TODO: does this _need_ to be a Cell?
    //       I was getting some annoying lifetime errors trying to take &mut Cursor.
    pos: Cell<usize>,
}

/// All methods read little-endian bytes.
impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Cursor<'a> {
        Cursor {
            data,
            pos: Cell::new(0),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos.get()
    }

    pub fn available(&self) -> usize {
        self.data.len() - self.pos.get()
    }

    fn bounds_check(&self, amt: usize) -> anyhow::Result<()> {
        if self.pos.get() + amt > self.data.len() {
            anyhow::bail!("Read out of bounds");
        } else {
            Ok(())
        }
    }

    pub fn skip(&self, amt: usize) {
        self.pos.set(self.pos.get() + amt)
    }

    pub fn read_u8(&self) -> anyhow::Result<u8> {
        self.bounds_check(1)?;
        let val = self.data[self.pos.get()];
        self.skip(1);
        Ok(val)
    }

    pub fn read_i8(&self) -> anyhow::Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u32(&self) -> anyhow::Result<u32> {
        self.bounds_check(4)?;
        let cur = &self.data[self.pos.get()..];
        let val = LittleEndian::read_u32(cur);
        self.skip(4);
        Ok(val)
    }

    pub fn read_i32(&self) -> anyhow::Result<i32> {
        self.bounds_check(4)?;
        let cur = &self.data[self.pos.get()..];
        let val = LittleEndian::read_i32(cur);
        self.skip(4);
        Ok(val)
    }

    pub fn read_f32(&self) -> anyhow::Result<f32> {
        self.bounds_check(4)?;
        let cur = &self.data[self.pos.get()..];
        let val = LittleEndian::read_f32(cur);
        self.skip(4);
        Ok(val)
    }

    pub fn read_bytes(&'a self, length: usize) -> anyhow::Result<&'a [u8]> {
        self.bounds_check(length)?;
        let cur = self.pos.get();
        let buf = &self.data[cur..cur + length];
        self.skip(length);
        Ok(buf)
    }

    pub fn peek_bytes(&'a self, length: usize) -> anyhow::Result<&'a [u8]> {
        self.bounds_check(length)?;
        let cur = self.pos.get();
        let buf = &self.data[cur..cur + length];
        Ok(buf)
    }

    /// Takes an amount of bytes to read in for a CStr,
    /// consumes it, and returns a pointer to a CStr that is valid.
    /// If there is a 0-byte in the middle of the number of bytes
    /// to read, then it will be the end of the string...
    /// i.e. If our buffer looks like
    /// Hello\0\0\0\0\0\0\0\0\0
    /// And we call read_cstr(buffer, 14), the resulting CStr
    /// will only be Hello. It will not read after the first null byte.
    pub fn read_cstr(&'a self, length: usize) -> anyhow::Result<&'a str> {
        self.bounds_check(length)?;
        let cur = self.pos.get();
        let buf = &self.data[cur..cur + length];
        let cstr = CStr::from_bytes_until_nul(buf)?;
        self.skip(length);
        Ok(cstr.to_str()?)
    }

    /* Read bytes from this cursor until we hit a null byte.
    Then, interpret the bytes read as a string. */
    pub fn read_cstr_until(&'a self) -> anyhow::Result<&'a str> {
        anyhow::bail!("TODO: read_cstr_until");
    }

    /// TODO: use the quick-protobuf Reader::read_varint32 instaed of this!!
    /// Reads a variable sized integer, like a protobuf...
    /// TBH idk, I found this in the CSGO-demos-manager code,
    /// And it seems like we need this when reading packets.
    /// https://github.com/akiver/CSGO-Demos-Manager/blob/1d0e062db854ae47889339c8f80656cfd55217f6/demoinfo/DemoInfo/BitStream/BitStreamUtil.cs#L115
    pub fn read_protobuf_var_int(&'a self) -> anyhow::Result<i32> {
        let mut b: u8 = 0x80;
        let mut result: i32 = 0;
        let mut count = 0;

        while b & 0x80 != 0 {
            b = self.read_u8()?;
            if count < 4 || count == 4 && ((b & 0xF8) == 0 || (b & 0xF8) == 0xF8) {
                result |= ((b & !0x80) as i32) << (7 * count);
            } else {
                anyhow::bail!("Overflowing the variable sized int!");
            }
            count += 1;
        }

        Ok(result)
    }
}

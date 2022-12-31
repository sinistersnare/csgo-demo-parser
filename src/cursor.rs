//! A simple Cursor implementation, using the bitbuffer library.
//! It helps me control the lifetimes easier.

use std::borrow::Cow;
use std::cell::Cell;

use bitbuffer::{BitReadBuffer, LittleEndian};

#[derive(Debug)]
pub struct Cursor<'a> {
    /// The buffer we use to read data from.
    buf: BitReadBuffer<'a, LittleEndian>,
    /// The position, IN BITS, of the buffer.
    bit_pos: Cell<usize>,
}

/// All methods read little-endian bytes.
impl<'a> Cursor<'a> {
    pub fn new(buf: &'a [u8]) -> Cursor<'a> {
        let buf = BitReadBuffer::new(buf, LittleEndian);
        let bit_pos = Cell::new(0);
        Cursor { buf, bit_pos }
    }

    /// TODO: rename this... into_chunk? chunk_from_self? new_chunk_from_self?
    pub fn chunk_bytes(&'a self, amt: usize) -> anyhow::Result<Cursor<'a>> {
        let buf = self.read_bytes(amt)?.to_vec();
        Ok(Cursor {
            buf: BitReadBuffer::from(buf),
            bit_pos: Cell::new(0)
        })
    }

    pub fn remaining_bits(&self) -> usize {
        self.buf.bit_len().checked_sub(self.bit_pos.get()).unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.remaining_bits() > 0
    }

    /// checks if the NUMBER OF BITS LEFT are more than requested.
    fn check_bounds(&self, amt: usize) -> anyhow::Result<()> {
        if self.remaining_bits() > amt {
            anyhow::bail!("Read out of bounds");
        } else {
            Ok(())
        }
    }

    fn advance_bits(&self, amt: usize) -> anyhow::Result<()> {
        self.check_bounds(amt)?;
        self.bit_pos.set(self.bit_pos.get() + amt);
        Ok(())
    }

    pub fn read_bit_bool(&self) -> anyhow::Result<bool> {
        let b = self.buf.read_bool(self.bit_pos.get())?;
        self.advance_bits(1)?;
        Ok(b)
    }

    pub fn read_byte_bool(&self) -> anyhow::Result<bool> {
        let n = self.read_u8()?;
        Ok(n != 0)
    }

    pub fn read_u8(&self) -> anyhow::Result<u8> {
        let n = self.buf.read_int(self.bit_pos.get(), 8)?;
        self.advance_bits(8)?;
        Ok(n)
    }

    pub fn read_i8(&self) -> anyhow::Result<i8> {
        let n = self.buf.read_int(self.bit_pos.get(), 8)?;
        self.advance_bits(8)?;
        Ok(n)
    }

    pub fn read_u16(&self) -> anyhow::Result<u16> {
        let n = self.buf.read_int(self.bit_pos.get(), 16)?;
        self.advance_bits(16)?;
        Ok(n)
    }

    pub fn read_i16(&self) -> anyhow::Result<i16> {
        let n = self.buf.read_int(self.bit_pos.get(), 16)?;
        self.advance_bits(16)?;
        Ok(n)
    }

    pub fn read_u32(&self) -> anyhow::Result<u32> {
        let n = self.buf.read_int(self.bit_pos.get(), 32)?;
        self.advance_bits(32)?;
        Ok(n)
    }

    pub fn read_i32(&self) -> anyhow::Result<i32> {
        let n = self.buf.read_int(self.bit_pos.get(), 32)?;
        self.advance_bits(32)?;
        Ok(n)
    }

    pub fn read_f32(&self) -> anyhow::Result<f32> {
        let n = self.buf.read_float(self.bit_pos.get())?;
        self.advance_bits(32)?;
        Ok(n)
    }

    pub fn read_i64(&self) -> anyhow::Result<i64> {
        let n = self.buf.read_int(self.bit_pos.get(), 64)?;
        self.advance_bits(64)?;
        Ok(n)
    }

    pub fn read_bytes(&'a self, amt: usize) -> anyhow::Result<Cow<'a, [u8]>> {
        let bytes = self.buf.read_bytes(self.bit_pos.get(), amt)?;
        self.advance_bits(amt * 8)?;
        Ok(bytes)
    }

    /// Takes an amount of bytes to read in for a CStr,
    /// consumes it, and returns a pointer to a CStr that is valid.
    /// If there is a 0-byte in the middle of the number of bytes
    /// to read, then it will be the end of the string...
    /// i.e. If our buffer looks like
    /// Hello\0\0\0\0\0\0\0\0\0
    /// And we call read_cstr(buffer, 14), the resulting CStr
    /// will only be Hello. It will not read after the first null byte.
    pub fn read_cstr(&'a self, length: usize) -> anyhow::Result<Cow<'a, str>> {
        let s = self.buf.read_string(self.bit_pos.get(), Some(length))?;
        self.advance_bits(length * 8)?;
        Ok(s)
    }

    /* Read bytes from this cursor until we hit a null byte.
    Then, interpret the bytes read as a string.
    Currently panics if there is no null byte before the end of the buffer. */
    pub fn read_cstr_until(&'a self) -> anyhow::Result<Cow<'a, str>> {
        let s = self.buf.read_string(self.bit_pos.get(), None)?;
        self.advance_bits((s.len() + 1) * 8)?;
        Ok(s)
    }


    /// TODO: Can we utilize prost for this?
    /// Reads a variable sized integer, like a protobuf...
    /// TBH idk, I found this in the CSGO-demos-manager code,
    /// And it seems like we need this when reading packets.
    /// https://github.com/akiver/CSGO-Demos-Manager/blob/1d0e062db854ae47889339c8f80656cfd55217f6/demoinfo/DemoInfo/BitStream/BitStreamUtil.cs#L115
    pub fn read_protobuf_var_int(&self) -> anyhow::Result<i32> {
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

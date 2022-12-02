use ux::u1;

use crate::{
    bit_read::BitRead,
    bit_write::BitWrite,
    util::{get_bit, set_bit},
};

// TODO: Deriving PartialEq here requires that _all_ of 'buf' matches, but really we only care that
// the bits from start_bit_index to end_bit_index match
#[derive(Debug, PartialEq, Eq)]
pub struct BitSlice<'a> {
    buf: &'a [u8],
    start_bit_index: usize,
    end_bit_index: usize,
}

impl BitSlice<'_> {
    pub(crate) fn new(buf: &[u8], start_bit_index: usize, end_bit_index: usize) -> BitSlice {
        BitSlice {
            buf,
            start_bit_index,
            end_bit_index,
        }
    }

    pub fn len(&self) -> usize {
        // Start and end are both inclusive, so add 1
        self.end_bit_index - self.start_bit_index + 1
    }

    pub fn at(&self, index: usize) -> u1 {
        assert!(index <= self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let byte = self.buf[byte_pos];
        get_bit(byte, bit_pos % 8)
    }
}

impl BitRead for BitSlice<'_> {
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        let n = self.len().min(buf.len());
        // TODO: optimize...
        for (i, bit) in buf.iter_mut().enumerate().take(n) {
            *bit = self.at(i);
        }
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut [u1]) -> std::io::Result<()> {
        if buf.len() > self.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ));
        }

        // TODO: optimize...
        for (i, bit) in buf.iter_mut().enumerate() {
            *bit = self.at(i);
        }

        Ok(())
    }
}

impl PartialEq<&[u1]> for BitSlice<'_> {
    fn eq(&self, other: &&[u1]) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (i, &bit) in other.iter().enumerate() {
            if self.at(i) != bit {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct BitSliceMut<'a> {
    buf: &'a mut [u8],
    start_bit_index: usize,
    end_bit_index: usize,
}

impl BitSliceMut<'_> {
    pub(crate) fn new(
        buf: &mut [u8],
        start_bit_index: usize,
        end_bit_index: usize,
    ) -> BitSliceMut<'_> {
        BitSliceMut {
            buf,
            start_bit_index,
            end_bit_index,
        }
    }

    pub fn len(&self) -> usize {
        // Start and end are both inclusive, so add 1
        self.end_bit_index - self.start_bit_index + 1
    }

    pub fn at(&self, index: usize) -> u1 {
        assert!(index <= self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let byte = self.buf[byte_pos];
        get_bit(byte, bit_pos % 8)
    }

    pub fn set(&mut self, index: usize, value: u1) {
        assert!(index <= self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        // Now make bit_pos relative to the byte
        let bit_pos = bit_pos % 8;
        let mut byte = &mut self.buf[byte_pos];
        println!("index = {}, byte_pos = {}, bit_pos = {}", index, byte_pos, bit_pos);
        set_bit(&mut byte, bit_pos, value);
    }
}

impl BitWrite for BitSliceMut<'_> {
    fn write(&mut self, buf: &[u1]) -> std::io::Result<usize> {
        let n = self.len().min(buf.len());
        for (i, bit) in buf.iter().enumerate().take(n) {
            self.set(i, *bit);
        }
        Ok(n)
    }

    fn write_all(&mut self, buf: &[u1]) -> std::io::Result<()> {
        if buf.len() > self.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "failed to write whole buffer",
            ));
        }

        // TODO: optimize...
        for (i, bit) in buf.iter().enumerate() {
            self.set(i, *bit);
        }
        Ok(())
    }
}

use ux::u1;

use crate::{util::{get_bit, set_bit}, bit_read::BitRead};


// TODO: Deriving PartialEq here requires that _all_ of 'buf' matches, but really we only care that
// the bits from start_bit_index to end_bit_index match
#[derive(Debug, PartialEq)]
pub struct BitSlice<'a> {
    buf: &'a [u8],
    start_bit_index: usize,
    end_bit_index: usize
}

impl BitSlice<'_> {
    pub(crate) fn new<'a>(buf: &'a [u8], start_bit_index: usize, end_bit_index: usize) -> BitSlice<'a> {
        BitSlice {
            buf,
            start_bit_index,
            end_bit_index
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
        for i in 0..n {
            buf[i] = self.at(i);
        }
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut[u1]) -> std::io::Result<()> {
        if buf.len() > self.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"));
        }

        // TODO: optimize...
        for i in 0..buf.len() {
            buf[i] = self.at(i);
        }

        Ok(())
    }
}

pub struct BitSliceMut<'a> {
    buf: &'a mut [u8],
    start_bit_index: usize,
    end_bit_index: usize
}

impl BitSliceMut<'_> {
    pub(crate) fn new<'a>(buf: &'a mut [u8], start_bit_index: usize, end_bit_index: usize) -> BitSliceMut<'a> {
        BitSliceMut {
            buf,
            start_bit_index,
            end_bit_index
        }
    }

    pub fn len(&self) -> usize {
        // Start and end are both inclusive, so add 1
        self.end_bit_index - self.start_bit_index + 1
    }

    pub fn set(&self, index: usize, value: u1) {
        assert!(index <= self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let mut byte = self.buf[byte_pos];
        set_bit(&mut byte, bit_pos, value);
    }
}

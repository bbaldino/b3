use std::ops::RangeBounds;

use ux::u1;

use crate::{
    bit_read::BitRead,
    bit_write::BitWrite,
    util::{get_bit, set_bit, get_start_end_bit_index_from_range},
};

/// A slice of bits.  |start_bit_index| is inclusive, |end_bit_index| is exclusive
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
        self.end_bit_index - self.start_bit_index
    }

    pub fn at(&self, index: usize) -> u1 {
        assert!(index < self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let byte = self.buf[byte_pos];
        get_bit(byte, bit_pos % 8)
    }

    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> BitSlice<'_> {
        println!("bitslice::get_slice");
        let (start_bit_index, end_bit_index) = get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len + 1;
        
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        //println!("returning slice with start byte {}, end_byte {}, start_bit_index {}, end_bit_index {}", start_byte, end_byte, start_bit_index, start_bit_index + bit_len);
        BitSlice::new(
            &self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        )
    }
}

impl BitRead for BitSlice<'_> {
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        println!("BitSlice::BitRead::read, self.len = {}, buf len = {}", self.len(), buf.len());
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

/// A mutable slice of bits.  |start_bit_index| is inclusive, |end_bit_index| is exclusive
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
        self.end_bit_index - self.start_bit_index
    }

    pub fn at(&self, index: usize) -> u1 {
        assert!(index < self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let byte = self.buf[byte_pos];
        get_bit(byte, bit_pos % 8)
    }

    pub fn set(&mut self, index: usize, value: u1) {
        assert!(index < self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        // Now make bit_pos relative to the byte
        let bit_pos = bit_pos % 8;
        let mut byte = &mut self.buf[byte_pos];
        set_bit(&mut byte, bit_pos, value);
    }

    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> BitSlice<'_> {
        let (start_bit_index, end_bit_index) = get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len + 1;
        
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        //println!("returning slice with start byte {}, end_byte {}, start_bit_index {}, end_bit_index {}", start_byte, end_byte, start_bit_index, start_bit_index + bit_len);
        BitSlice::new(
            &self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        )
    }

    pub fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> BitSliceMut<'_> {
        let (start_bit_index, end_bit_index) = get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len + 1;
        
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        //println!("returning slice with start byte {}, end_byte {}, start_bit_index {}, end_bit_index {}", start_byte, end_byte, start_bit_index, start_bit_index + bit_len);
        BitSliceMut::new(
            &mut self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        )
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

impl PartialEq<&[u1]> for BitSliceMut<'_> {
    fn eq(&self, other: &&[u1]) -> bool {
        PartialEq::eq(&self.get_slice(..), other)
    }
}

#[cfg(test)]
mod tests {
    use ux::u1;

    use crate::{bitvec, bitarray};

    #[test]
    fn get_slice_from_bit_slice() {
        let vec = bitvec!(1, 0, 1, 0, 1, 0);
        let slice_one = vec.get_slice(1..);
        let slice_two = slice_one.get_slice(1..);
        assert_eq!(slice_two.len(), 4);
        assert_eq!(slice_two, &bitarray!(1, 0, 1, 0)[..]);
    }

    #[test]
    fn get_slice_from_bit_slice_mut() {
        let mut vec = bitvec!(1, 0, 1, 0, 1, 0);
        let slice_one = vec.get_slice_mut(1..);
        let slice_two = slice_one.get_slice(1..);
        assert_eq!(slice_two.len(), 4);
        assert_eq!(slice_two, &bitarray!(1, 0, 1, 0)[..]);
    }

    #[test]
    fn get_slice_mut_from_bit_slice_mut() {
        let mut vec = bitvec!(1, 0, 1, 0, 1, 0);
        let mut slice_one = vec.get_slice_mut(1..);
        let mut slice_two = slice_one.get_slice_mut(1..);
        assert_eq!(slice_two.len(), 4);
        assert_eq!(slice_two, &bitarray!(1, 0, 1, 0)[..]);
        slice_two.set(0, u1::new(0));
        assert_eq!(slice_one.at(1), u1::new(0));
    }
}

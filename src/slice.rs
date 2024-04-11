use std::ops::RangeBounds;

use ux::u1;

use crate::{
    bit_buffer::{BitBuffer, BitBufferMut},
    bit_read::BitRead,
    bit_vec::BitVec,
    bit_write::BitWrite,
    error::{B3Error, B3Result},
    util::{get_bit, get_start_end_bit_index_from_range, set_bit},
};

// TODO: Multiple operations here are done bit-by-bit and _could_ likely be optimized to do
// byte-wise operations when possible.

/// A slice of bits.  |start_bit_index| is inclusive, |end_bit_index| is exclusive
#[derive(Debug, Eq)]
pub struct BitSlice<'a> {
    buf: &'a [u8],
    start_bit_index: usize,
    end_bit_index: usize,
}

#[allow(clippy::len_without_is_empty)]
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

    /// Retrive the [`u1`] at the given index.  Panics if index is out-of-bounds.
    ///
    /// * `index`: The index.
    pub fn at(&self, index: usize) -> u1 {
        assert!(index < self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let byte = self.buf[byte_pos];
        get_bit(byte, bit_pos % 8)
    }

    /// Get an iterator over the bits in this slice.
    pub fn iter(&self) -> BitSliceIterator<'_> {
        BitSliceIterator {
            slice: self,
            curr_index: 0,
        }
    }

    /// Get a slice of this slice corresponding to the given range.
    ///
    /// * `range`: The range.
    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len;

        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        if end_byte >= self.buf.len() {
            return Err(B3Error::SliceOutOfRange {
                len: self.buf.len(),
                slice_start: start_byte,
                slice_end: end_byte,
            });
        }
        Ok(BitSlice::new(
            &self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        ))
    }
}

impl PartialEq for BitSlice<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter()
            .zip(other.iter())
            .try_for_each(|(left, right)| if left == right { Ok(()) } else { Err(()) })
            .is_ok()
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

/// An interator over a [`BitSlice`].
pub struct BitSliceIterator<'a> {
    slice: &'a BitSlice<'a>,
    curr_index: usize,
}

impl<'a> Iterator for BitSliceIterator<'a> {
    type Item = u1;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index == self.slice.len() {
            return None;
        }
        let item = self.slice.at(self.curr_index);
        self.curr_index += 1;
        Some(item)
    }
}

// TODO: can we do a blanket impl for PartialEq based on some trait?
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

impl PartialEq<BitVec> for BitSlice<'_> {
    fn eq(&self, other: &BitVec) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (i, ref bit) in other.iter().enumerate() {
            if &self.at(i) != bit {
                return false;
            }
        }
        true
    }
}

impl BitBuffer for BitSlice<'_> {
    fn len(&self) -> usize {
        self.len()
    }
    fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>> {
        self.get_slice(range)
    }
}

/// A mutable slice of bits.  |start_bit_index| is inclusive, |end_bit_index| is exclusive
#[derive(Debug)]
pub struct BitSliceMut<'a> {
    buf: &'a mut [u8],
    start_bit_index: usize,
    end_bit_index: usize,
}

#[allow(clippy::len_without_is_empty)]
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
        let byte = &mut self.buf[byte_pos];
        set_bit(byte, bit_pos, value);
    }

    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len;

        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        if end_byte >= self.buf.len() {
            return Err(B3Error::SliceOutOfRange {
                len: self.buf.len(),
                slice_start: start_byte,
                slice_end: end_byte,
            });
        }
        Ok(BitSlice::new(
            &self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        ))
    }

    pub fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> B3Result<BitSliceMut<'_>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len;

        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        if end_byte >= self.buf.len() {
            return Err(B3Error::SliceOutOfRange {
                len: self.buf.len(),
                slice_start: start_byte,
                slice_end: end_byte,
            });
        }
        Ok(BitSliceMut::new(
            &mut self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        ))
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
        // safety: this slice range will always be valid
        PartialEq::eq(&self.get_slice(..).unwrap(), other)
    }
}

impl PartialEq<BitVec> for BitSliceMut<'_> {
    fn eq(&self, other: &BitVec) -> bool {
        // safety: this slice range will always be valid
        PartialEq::eq(&self.get_slice(..).unwrap(), other)
    }
}

impl BitBuffer for BitSliceMut<'_> {
    fn len(&self) -> usize {
        self.len()
    }
    fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>> {
        self.get_slice(range)
    }
}

impl BitBufferMut for BitSliceMut<'_> {
    fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> B3Result<BitSliceMut<'_>> {
        self.get_slice_mut(range)
    }
}

#[cfg(test)]
mod tests {
    use ux::u1;

    use crate::bitvec;

    #[test]
    fn get_slice_from_bit_slice() {
        let vec = bitvec!(1, 0, 1, 0, 1, 0);
        let slice_one = vec.get_slice(1..).expect("valid slice");
        let slice_two = slice_one.get_slice(1..).expect("valid slice");
        assert_eq!(slice_two.len(), 4);
        assert_eq!(slice_two, bitvec!(1, 0, 1, 0));
    }

    #[test]
    fn get_slice_from_bit_slice_mut() {
        let mut vec = bitvec!(1, 0, 1, 0, 1, 0);
        let slice_one = vec.get_slice_mut(1..).expect("valid slice");
        let slice_two = slice_one.get_slice(1..).expect("valid slice");
        assert_eq!(slice_two.len(), 4);
        assert_eq!(slice_two, bitvec!(1, 0, 1, 0));
    }

    #[test]
    fn get_slice_mut_from_bit_slice_mut() {
        let mut vec = bitvec!(1, 0, 1, 0, 1, 0);
        let mut slice_one = vec.get_slice_mut(1..).expect("valid slice");
        let mut slice_two = slice_one.get_slice_mut(1..).expect("valid slice");
        assert_eq!(slice_two.len(), 4);
        assert_eq!(slice_two, bitvec!(1, 0, 1, 0));
        slice_two.set(0, u1::new(0));
        assert_eq!(slice_one.at(1), u1::new(0));
    }

    #[test]
    fn test_set() {
        let mut vec = bitvec!(0, 0, 0, 0);
        let mut slice = vec.get_slice_mut(..).expect("valid slice");
        slice.set(0, u1::new(1));
        assert_eq!(vec, bitvec!(1, 0, 0, 0));
    }

    #[test]
    fn test_iterator() {
        let vec = bitvec!(1, 0, 1, 0, 1, 0);
        let slice = vec.get_slice(2..4).expect("valid slice");
        let mut iter = slice.iter();
        assert_eq!(iter.next(), Some(u1::new(1)));
        assert_eq!(iter.next(), Some(u1::new(0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_partial_eq() {
        let vec_one = bitvec!(0, 1);
        let vec_two = bitvec!(1, 1);
        let slice_one = vec_one.get_slice(1..);
        let slice_two = vec_two.get_slice(1..);

        assert_eq!(slice_one, slice_two);
    }
}

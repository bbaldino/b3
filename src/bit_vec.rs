use std::ops::RangeBounds;

use ux::*;

use crate::{
    slice::{BitSlice, BitSliceMut},
    util::{get_bit, get_start_end_bit_index_from_range, set_bit},
};

#[derive(Debug)]
pub struct BitVec {
    buf: Vec<u8>,
    len: usize,
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            buf: Vec::new(),
            len: 0,
        }
    }

    pub fn from_u8_vec(vec: Vec<u8>) -> BitVec {
        let len = vec.len() * 8;
        BitVec { buf: vec, len }
    }

    pub fn with_capacity(capacity: usize) -> BitVec {
        BitVec {
            buf: Vec::with_capacity((capacity + 7) / 8),
            len: 0,
        }
    }

    pub fn push<T: Into<u1>>(&mut self, value: T) {
        // 'allocate' another byte if needed
        if self.len % 8 == 0 {
            self.buf.push(0);
        }
        let last_byte = self.buf.last_mut().unwrap();
        set_bit(last_byte, self.len % 8, value.into());
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<u1> {
        if self.len == 0 {
            return None;
        }
        let last_byte = self.buf.last().unwrap();
        let result = get_bit(*last_byte, (self.len - 1) % 8);

        self.len -= 1;
        if self.len % 8 == 0 {
            self.buf.pop();
        }

        Some(result)
    }

    pub fn at(&self, index: usize) -> u1 {
        assert!(index < self.len());
        let byte_pos = index / 8;
        let bit_index = index % 8;
        get_bit(self.buf[byte_pos], bit_index)
    }

    pub fn iter(&self) -> BitVecIterator<'_> {
        BitVecIterator {
            vec: self,
            bit_pos: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity() * 8
    }

    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> BitSlice<'_> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        let bit_len = end_bit_index - start_bit_index;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        BitSlice::new(
            &self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        )
    }

    pub fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> BitSliceMut<'_> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        let bit_len = end_bit_index - start_bit_index;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        BitSliceMut::new(
            &mut self.buf[start_byte..=end_byte],
            start_bit_index,
            start_bit_index + bit_len,
        )
    }
}

impl Default for BitVec {
    fn default() -> Self {
        Self::new()
    }
}

pub fn into_bitvec(data: &[u8]) -> BitVec {
    let mut vec = BitVec::with_capacity(data.len());
    for &val in data {
        vec.push(u1::new(val));
    }
    vec
}

pub fn from_elem(elem: u8, n: usize) -> BitVec {
    let mut vec = BitVec::with_capacity(n);
    for _ in 0..n {
        vec.push(u1::new(elem));
    }
    vec
}

/// Create a BitVec using array style syntax, either:
/// bitvec!(0, 1, 0, ...);
/// bitvec!(0; 10);
// TODO: would be nice to expand this to validate only 0s and 1s were passed
#[macro_export]
macro_rules! bitvec {
    ($($x:expr),+ $(,)?) => {
        $crate::bit_vec::into_bitvec(&[$($x),+])
    };
    ($elem:expr; $n:expr) => {
        $crate::bit_vec::from_elem($elem, $n)
    }
}

/// Create a [u1; N] array
#[macro_export]
macro_rules! bitarray {
    (0$(, $rest:tt)*) => {
        bitarray!(@internal [::ux::u1::new(0)] $($rest),*)
    };
    (1$(, $rest:tt)*) => {
        bitarray!(@internal [::ux::u1::new(1)] $($rest),*)
    };
    (@internal [$($done:expr$(,)?)+] 0$(, $rest:tt)*) => {
        bitarray!(@internal [$($done)*, ::ux::u1::new(0)] $($rest),*)
    };
    (@internal [$($done:expr$(,)?)+] 1$(, $rest:tt)*) => {
        bitarray!(@internal [$($done)*, ::ux::u1::new(1)] $($rest),*)
    };
    (@internal [$($done:expr$(,)?)+]) => {
        [$($done, )*]
    };
}

pub struct BitVecIterator<'a> {
    vec: &'a BitVec,
    bit_pos: usize,
}

impl Iterator for BitVecIterator<'_> {
    type Item = u1;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_pos == self.vec.len() {
            return None;
        }
        let val = self.vec.at(self.bit_pos);
        self.bit_pos += 1;
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut vec = BitVec::new();

        for i in 1..=16 {
            vec.push(u1::new(1));
            assert_eq!(vec.len(), i);
        }
        assert!(vec.iter().all(|b| b == u1::new(1)));
    }

    #[test]
    fn test_pop() {
        let mut vec = bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0);
        assert_eq!(vec.pop().unwrap(), u1::new(0));
        assert_eq!(vec.len(), 16);
        assert_eq!(vec.pop().unwrap(), u1::new(1));
    }

    #[test]
    fn test_get_slice() {
        let vec = bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1);
        let slice = vec.get_slice(3..);
        assert_eq!(slice.len(), 13);
        assert_eq!(slice, BitSlice::new(&[0b00000000, 0b11111111], 3, 16));

        let slice = vec.get_slice(..=5);
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, BitSlice::new(&[0b00000000], 0, 6));

        let slice = vec.get_slice(3..11);
        assert_eq!(slice.len(), 8);
        assert_eq!(slice, BitSlice::new(&[0b00000000, 0b11111111], 3, 11));
    }

    #[test]
    fn test_bit_slice_mut() {
        let mut vec = bitvec!(0);
        let mut slice = vec.get_slice_mut(..);
        assert_eq!(slice.len(), 1);
        slice.set(0, u1::new(1));
        assert_eq!(slice.at(0), u1::new(1));
    }
}

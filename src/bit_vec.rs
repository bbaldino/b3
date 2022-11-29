use std::{ops::{Range, RangeBounds}, fmt::Debug};

use ux::*;

use crate::{util::{set_bit, get_bit}, slice::BitSlice};

// TODO: track capacity here? check Vec
#[derive(Debug)]
pub struct BitVec {
    buf: Vec<u8>,
    len: usize
}

impl BitVec {
    pub fn new() -> BitVec { 
        BitVec {
            buf: Vec::new(),
            len: 0
        }
    }

    pub fn with_capacity(capacity: usize) -> BitVec {
        BitVec {
            buf: Vec::with_capacity(capacity + 7 / 8),
            len: 0
        }
    }

    pub fn push<T: Into<u1>>(&mut self, value: T) {
        // 'allocate' another byte if needed
        match self.len % 8 {
            0 => {
                self.buf.push(0);
            },
            _ => {}
        }
        let mut last_byte = self.buf.last_mut().unwrap();
        set_bit(&mut last_byte, self.len % 8, value.into());
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<u1> {
        if self.len == 0 {
            return None
        }
        let last_byte = self.buf.last().unwrap();
        let result = get_bit(*last_byte, (self.len - 1) % 8);

        self.len -= 1;
        match self.len % 8 {
            0 => {
                self.buf.pop();
            },
            _ => {}
        }

        Some(result)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    //pub fn get_slice(&self, range: Range<usize>) -> BitSlice<'_> {
    //    println!("getting slice for range {:?}", range);
    //    let start_byte = range.start / 8;
    //    let end_byte = (range.end - 1) / 8;
    //    let start_bit = range.start % 8;
    //    println!("start_byte: {}, end_byte: {}, start_bit: {}, end_bit: {}", start_byte, end_byte, start_bit, start_bit + range.end - start_bit - 1);
    //    BitSlice::new(&self.buf[start_byte..=end_byte], start_bit, start_bit + range.end - start_bit - 1)
    //}

    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> BitSlice<'_> {
        println!("getting slice with bounds: {:?} {:?}", range.start_bound(), range.end_bound());
        let start_bit_index = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end_bit_index = match range.end_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(s) => s - 1,
            std::ops::Bound::Unbounded => self.len() - 1,
        };
        let start_byte = start_bit_index / 8;
        let end_byte = end_bit_index / 8;
        let bit_len = end_bit_index - start_bit_index;
        println!("returning slice with start byte {}, end_byte {}, start_bit_index {}, end_bit_index {}", start_byte, end_byte, start_bit_index, start_bit_index + bit_len);
        BitSlice::new(&self.buf[start_byte..=end_byte], start_bit_index, start_bit_index + bit_len)
    }

    //pub fn get_slice_mut(&mut self, range: Range<usize>) -> BitSliceMut<'_> {
    //    println!("getting mut slice, range start bit index: {}, range end bit index: {}", range.start, range.end);
    //    let start_byte = range.start / 8;
    //    let end_byte = range.end / 8;
    //    let start_bit = range.start % 8;
    //    println!("start_byte: {}, end_byte: {}, start_bit: {}, end_bit: {}", start_byte, end_byte, start_bit, range.end - start_bit);
    //    BitSliceMut {
    //        buf: &mut self.buf[start_byte..=end_byte],
    //        start_bit_index: start_bit,
    //        end_bit_index: start_bit + (range.end - range.start),
    //    }
    //}
}

impl AsRef<BitVec> for BitVec {
    fn as_ref(&self) -> &BitVec {
        &self
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
        assert_eq!(slice, BitSlice::new(&[0b00000000, 0b11111111], 3, 15));

        let slice = vec.get_slice(..=5);
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, BitSlice::new(&[0b00000000], 0, 5));

        let slice = vec.get_slice(3..11);
        assert_eq!(slice.len(), 8);
        assert_eq!(slice, BitSlice::new(&[0b00000000, 0b11111111], 3, 10));
    }

    //#[test]
    //fn test_bit_slice_mut() {
    //    let mut vec = bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0);
    //    let slice = vec.get_slice_mut(7..15);
    //    assert_eq!(slice.len(), 8);
    //    assert_eq!(slice.at(0), u1::new(0));
    //    assert_eq!(slice.at(1), u1::new(1));
    //    assert_eq!(slice.at(2), u1::new(1));
    //    assert_eq!(slice.at(3), u1::new(1));
    //    assert_eq!(slice.at(4), u1::new(1));
    //    assert_eq!(slice.at(5), u1::new(1));
    //    assert_eq!(slice.at(6), u1::new(1));
    //    assert_eq!(slice.at(7), u1::new(1));
    //    assert_eq!(slice.at(8), u1::new(1));
    //    
    //}
}

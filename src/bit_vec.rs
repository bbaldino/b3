use std::ops::RangeBounds;

use ux::*;

use crate::{
    bit_buffer::{BitBuffer, BitBufferMut},
    error::{B3Error, B3Result},
    slice::{BitSlice, BitSliceMut},
    util::{get_bit, get_start_end_bit_index_from_range, set_bit},
};

#[derive(Debug, Eq, PartialEq)]
pub struct BitVec {
    buf: Vec<u8>,
    /// The length of the data inside this BitVec, in bits
    len: usize,
}

// TODO: 'release' function that returns Vec and length? do we need the length?

/// A Vector whose API is in bits, instead of bytes.
#[allow(clippy::len_without_is_empty)]
impl BitVec {
    /// Create a BitVec with an empty buffer
    pub fn new() -> BitVec {
        BitVec {
            buf: Vec::new(),
            len: 0,
        }
    }

    /// Create a BitVec from the given buffer.
    ///
    /// * `data`: The backing buffer to be used for this BitVec.  It's assumed that the vector
    /// is "full" of bits, i.e. the length of this BitVec will be set to the length of the
    /// vector * 8.
    pub fn from_vec(data: Vec<u8>) -> BitVec {
        let len = data.len() * 8;
        BitVec { buf: data, len }
    }

    /// Create a BitVec with the given capacity (in bits)
    ///
    /// * `capacity`: the initial capacity of the BitVec, in bits
    pub fn with_capacity(capacity: usize) -> BitVec {
        BitVec {
            buf: Vec::with_capacity((capacity + 7) / 8),
            len: 0,
        }
    }

    /// Push the given value onto the end of this BitVec.  The value will be converted to a u1.
    ///
    /// * `value`: The value to push.
    /// # Example
    /// ```
    /// use ux::u1;
    /// use b3::{bit_vec::BitVec, bit_traits::BitTraits};
    ///
    /// let mut vec = BitVec::new();
    /// vec.push(u1::ONE);
    /// vec.push(u1::ZERO);
    /// vec.push(u1::ONE);
    /// assert_eq!(vec.at(0), u1::ONE);
    /// assert_eq!(vec.at(1), u1::ZERO);
    /// assert_eq!(vec.at(2), u1::ONE);
    /// ```
    pub fn push<T: Into<u1>>(&mut self, value: T) {
        // 'allocate' another byte if needed
        if self.len % 8 == 0 {
            self.buf.push(0);
        }
        let last_byte = self.buf.last_mut().unwrap();
        set_bit(last_byte, self.len % 8, value.into());
        self.len += 1;
    }

    /// Remove and return the last u1 in this buffer, if there is one.
    /// # Example
    /// ```
    /// use ux::u1;
    /// use b3::{bitvec, bit_traits::BitTraits};
    /// let mut vec = bitvec!(0, 1);
    /// assert_eq!(vec.pop().unwrap(), u1::ONE);
    /// assert_eq!(vec.pop().unwrap(), u1::ZERO);
    /// assert_eq!(vec.pop(), None);
    /// ```
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

    /// Get the bit at the given index.  Panics if index is out of range.
    ///
    /// * `index`: The index
    /// # Example
    /// ```
    /// use ux::u1;
    /// use b3::{bitvec, bit_traits::BitTraits};
    /// let vec = bitvec!(0, 1, 1);
    /// assert_eq!(vec.at(0), u1::ZERO);
    /// assert_eq!(vec.at(1), u1::ONE);
    /// assert_eq!(vec.at(2), u1::ONE);
    /// ```
    pub fn at(&self, index: usize) -> u1 {
        assert!(index < self.len());
        let byte_pos = index / 8;
        let bit_index = index % 8;
        get_bit(self.buf[byte_pos], bit_index)
    }

    /// Get an iterator to the bits in this BitVec
    /// # Example
    /// ```
    /// use ux::u1;
    /// use b3::{bitvec, bit_traits::BitTraits};
    /// let vec = bitvec!(1, 0, 1);
    /// let mut iter = vec.iter();
    /// assert_eq!(iter.next(), Some(u1::ONE));
    /// assert_eq!(iter.next(), Some(u1::ZERO));
    /// assert_eq!(iter.next(), Some(u1::ONE));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> BitVecIterator<'_> {
        BitVecIterator {
            vec: self,
            bit_pos: 0,
        }
    }

    /// Return the length of this BitVec in bits.
    /// # Example
    /// ```
    /// use b3::bitvec;
    /// let vec = bitvec!(1, 0, 1);
    /// assert_eq!(vec.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return the capacity of this BitVec in bits.
    pub fn capacity(&self) -> usize {
        self.buf.capacity() * 8
    }

    /// Get a slice of this BitVec representing the given range, where the left-most bit is index
    /// 0.
    ///
    /// * `range`: the range
    ///
    /// # Example
    /// ```
    /// use b3::bitvec;
    ///
    /// let vec = bitvec!(0, 0, 0, 0, 1, 1, 1, 1);
    /// let slice = vec.get_slice(3..).expect("valid slice");
    /// assert_eq!(slice.len(), 5);
    /// assert_eq!(slice, bitvec!(0, 1, 1, 1, 1));
    /// ```
    pub fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        let bit_len = end_bit_index - start_bit_index;
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

    /// Get a mutable slice of this BitVec representing the given range.
    ///
    /// * `range`: the range
    ///
    /// # Example
    /// ```
    /// use ux::u1;
    /// use b3::{bitvec, bit_traits::BitTraits};
    ///
    /// let mut vec = bitvec!(0, 1, 1, 0);
    /// let mut slice = vec.get_slice_mut(1..=2).expect("valid slice");
    /// assert_eq!(slice.len(), 2);
    /// slice.set(0, u1::ZERO);
    /// slice.set(1, u1::ZERO);
    /// assert_eq!(vec, bitvec!(0, 0, 0, 0));
    /// ```
    pub fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> B3Result<BitSliceMut<'_>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        let bit_len = end_bit_index - start_bit_index;
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

impl Default for BitVec {
    fn default() -> Self {
        Self::new()
    }
}

impl BitBuffer for BitVec {
    fn len(&self) -> usize {
        self.len()
    }

    fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>> {
        self.get_slice(range)
    }
}

impl BitBufferMut for BitVec {
    fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> B3Result<BitSliceMut<'_>> {
        self.get_slice_mut(range)
    }
}

/// Create a BitVec from a u8 slice, where it's assumed that each u8 value fits in a u1
///
/// * `data`: the data
pub fn into_bitvec(data: &[u8]) -> BitVec {
    let mut vec = BitVec::with_capacity(data.len());
    for &val in data {
        vec.push(u1::new(val));
    }
    vec
}

/// Crate a BitVec with the given size full of the given element.  It's assume that the element
/// fits into a u1.
///
/// * `elem`: The element to fill the BitVec with.
/// * `n`: The size of the BitVec, in bits.
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
#[macro_export]
macro_rules! bitvec {
    (0$(, $rest:tt)*) => {
        bitvec!(@internal [0] $($rest),*)
    };
    (1$(, $rest:tt)*) => {
        bitvec!(@internal [1] $($rest),*)
    };
    (@internal [$($done:expr$(,)?)+] 0$(, $rest:tt)*) => {
        bitvec!(@internal [$($done)*, 0] $($rest),*)
    };
    (@internal [$($done:expr$(,)?)+] 1$(, $rest:tt)*) => {
        bitvec!(@internal [$($done)*, 1] $($rest),*)
    };
    (@internal [$($done:expr$(,)?)+]) => {
        $crate::bit_vec::into_bitvec(&[$($done),+])
    };
    ($elem:expr; $n:expr) => {
        $crate::bit_vec::from_elem($elem, $n)
    };
    ($val:expr$(, $rest:tt)*) => {
        compile_error!("Only 1s and 0s are valid when creating a bitvec")
    };
}

/// Create a [u1; N] array.  This is mainly used for testing the byteorder functions, which expect
/// specifically-sized u1 arrays as their arguments.
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

/// An iterator over the bits of a BitVec.
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
        let slice = vec.get_slice(3..).expect("valid slice");
        assert_eq!(slice.len(), 13);
        assert_eq!(slice, BitSlice::new(&[0b00000000, 0b11111111], 3, 16));

        let slice = vec.get_slice(..=5).expect("valid slice");
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, BitSlice::new(&[0b00000000], 0, 6));

        let slice = vec.get_slice(3..11).expect("valid slice");
        assert_eq!(slice.len(), 8);
        assert_eq!(slice, BitSlice::new(&[0b00000000, 0b11111111], 3, 11));
    }

    #[test]
    fn test_bit_slice_mut() {
        let mut vec = bitvec!(0);
        let mut slice = vec.get_slice_mut(..).expect("valid slice");
        assert_eq!(slice.len(), 1);
        slice.set(0, u1::new(1));
        assert_eq!(slice.at(0), u1::new(1));
    }
}

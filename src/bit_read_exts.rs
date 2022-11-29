use ux::*;
use paste::paste;

use crate::bit_read::BitRead;

macro_rules! impl_bit_read {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](&mut self) -> $type {
                let mut buf = [u1::new(0); $size_bits];
                self.read_exact(&mut buf).unwrap();
                let mut val = <$type>::new(0);
                for i in 0..$size_bits {
                    val <<= 1;
                    val |= buf[i].into();
                }
                val
            }
        }
    };
}

/// Generate a LittleEndian read operation from a buffer for the uX types
macro_rules! impl_read_le {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](&self, buf: &[u1; $size_bits]) -> $type {
                let mut val = <$type>::new(0);
                if $size_bits > 32 {
                    unimplemented!("Only uX types up to u32 supported");
                }
                if $size_bits > 24 {
                    for i in 24..std::cmp::min($size_bits, 32) {
                        val <<= 1;
                        val |= buf[i].into();
                    }
                }
                if $size_bits > 16 {
                    for i in 16..std::cmp::min($size_bits, 24) {
                        val <<= 1;
                        val |= buf[i].into();
                    }
                }
                if $size_bits > 8 {
                    for i in 8..std::cmp::min($size_bits, 16) {
                        val <<= 1;
                        val |= buf[i].into();
                    }
                }
                for i in 0..std::cmp::min($size_bits, 8) {
                    val <<= 1;
                    val |= buf[i].into();
                }
                val
            }
        }
    };
}

/// Helper macro to define a ByteOrder read operation for uX types
macro_rules! def_bo_read {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](&self, buf: &[u1; $size_bits]) -> $type;
        }
    };
}

pub trait ByteOrder {
    def_bo_read!(u9, 9);
    def_bo_read!(u11, 11);
    def_bo_read!(u12, 12);
    def_bo_read!(u18, 18);
}

pub struct BigEndian {}

pub struct LittleEndian {}

impl ByteOrder for LittleEndian {
    impl_read_le!(u9, 9);
    impl_read_le!(u11, 11);
    impl_read_le!(u12, 12);
    impl_read_le!(u18, 18);
}

pub trait BitReadExts : BitRead {
    impl_bit_read!(u2, 2);
    impl_bit_read!(u3, 3);
    impl_bit_read!(u4, 4);
    impl_bit_read!(u5, 5);
}

impl<T> BitReadExts for T where T: BitRead {}

#[cfg(test)]
mod tests {
    use crate::{bitvec, bit_cursor::BitCursor};

    use super::*;

    #[test]
    fn test_name() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0);
        let mut cursor = BitCursor::new(vec);

        assert_eq!(cursor.read_u2(), u2::new(3));
        assert_eq!(cursor.read_u4(), u4::new(0b1100));
    }
}

use std::ops::{BitOrAssign, ShlAssign};

use paste::paste;
use ux::*;

/// Generate a LittleEndian read operation from a buffer for the uX types
macro_rules! impl_read_le {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](buf: &[u1; $size_bits]) -> $type {
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

pub trait ByteOrder {
    fn read_u9(buf: &[u1; 9]) -> u9;
    fn read_u11(buf: &[u1; 11]) -> u11;
    fn read_u12(buf: &[u1; 12]) -> u12;
    fn read_u18(buf: &[u1; 18]) -> u18;
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

// I think instead of the macro, we could write a helper function to implement the LE read.  I
// think we'll need to define a trait for the uX types, though: at least one that defines the 'new'
// method?

impl ByteOrder for LittleEndian {
    impl_read_le!(u9, 9);
    impl_read_le!(u11, 11);
    impl_read_le!(u12, 12);
    impl_read_le!(u18, 18);
}

fn big_endian_read_helper<T, const N: usize>(buf: &[u1; N]) -> T
where
    T: Default + From<u1> + ShlAssign<usize> + BitOrAssign,
{
    let mut val = T::default();
    for bit in buf.iter() {
        val <<= 1;
        val |= (*bit).into();
    }
    val
}

impl ByteOrder for BigEndian {
    fn read_u9(buf: &[u1; 9]) -> u9 {
        big_endian_read_helper(buf)
    }

    fn read_u11(buf: &[u1; 11]) -> u11 {
        big_endian_read_helper(buf)
    }

    fn read_u12(buf: &[u1; 12]) -> u12 {
        big_endian_read_helper(buf)
    }

    fn read_u18(buf: &[u1; 18]) -> u18 {
        big_endian_read_helper(buf)
    }
}

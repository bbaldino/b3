use ux::*;
use paste::paste;

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

/// Helper macro to define a ByteOrder read operation for uX types
macro_rules! def_bo_read {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](buf: &[u1; $size_bits]) -> $type;
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


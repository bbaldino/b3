use paste::paste;
use ux::*;

use crate::bit_traits::BitTraits;

/// Generate a LittleEndian read operation from a buffer for the uX types
macro_rules! impl_read_le {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](buf: &[u1; $size_bits]) -> $type {
                let mut val = <$type>::default();
                if $size_bits > 32 {
                    unimplemented!("Only uX types up to u32 supported");
                }
                if $size_bits > 24 {
                    for i in 24..std::cmp::min($size_bits, 32) {
                        val <<= 1;
                        val |= <ux::u1 as Into<$type>>::into(buf[i]);
                    }
                }
                if $size_bits > 16 {
                    for i in 16..std::cmp::min($size_bits, 24) {
                        val <<= 1;
                        val |= <ux::u1 as Into<$type>>::into(buf[i]);
                    }
                }
                if $size_bits > 8 {
                    for i in 8..std::cmp::min($size_bits, 16) {
                        val <<= 1;
                        val |= <ux::u1 as Into<$type>>::into(buf[i]);
                    }
                }
                for i in 0..std::cmp::min($size_bits, 8) {
                    val <<= 1;
                    val |= <ux::u1 as Into<$type>>::into(buf[i]);
                }
                val
            }
        }
    };
}

pub trait ByteOrder {
    fn read_u9(buf: &[u1; 9]) -> u9;
    fn read_u10(buf: &[u1; 10]) -> u10;
    fn read_u11(buf: &[u1; 11]) -> u11;
    fn read_u12(buf: &[u1; 12]) -> u12;
    fn read_u13(buf: &[u1; 13]) -> u13;
    fn read_u14(buf: &[u1; 14]) -> u14;
    fn read_u15(buf: &[u1; 15]) -> u15;
    fn read_u16(buf: &[u1; 16]) -> u16;
    fn read_u17(buf: &[u1; 17]) -> u17;
    fn read_u18(buf: &[u1; 18]) -> u18;
    fn read_u19(buf: &[u1; 19]) -> u19;
    fn read_u20(buf: &[u1; 20]) -> u20;
    fn read_u21(buf: &[u1; 21]) -> u21;
    fn read_u22(buf: &[u1; 22]) -> u22;
    fn read_u23(buf: &[u1; 23]) -> u23;
    fn read_u24(buf: &[u1; 24]) -> u24;
    fn read_u25(buf: &[u1; 25]) -> u25;
    fn read_u26(buf: &[u1; 26]) -> u26;
    fn read_u27(buf: &[u1; 27]) -> u27;
    fn read_u28(buf: &[u1; 28]) -> u28;
    fn read_u29(buf: &[u1; 29]) -> u29;
    fn read_u30(buf: &[u1; 30]) -> u30;
    fn read_u31(buf: &[u1; 31]) -> u31;
    fn read_u32(buf: &[u1; 32]) -> u32;
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

impl ByteOrder for LittleEndian {
    impl_read_le!(u9, 9);
    impl_read_le!(u10, 10);
    impl_read_le!(u11, 11);
    impl_read_le!(u12, 12);
    impl_read_le!(u13, 13);
    impl_read_le!(u14, 14);
    impl_read_le!(u15, 15);
    impl_read_le!(u16, 16);
    impl_read_le!(u17, 17);
    impl_read_le!(u18, 18);
    impl_read_le!(u19, 19);
    impl_read_le!(u20, 20);
    impl_read_le!(u21, 21);
    impl_read_le!(u22, 22);
    impl_read_le!(u23, 23);
    impl_read_le!(u24, 24);
    impl_read_le!(u25, 25);
    impl_read_le!(u26, 26);
    impl_read_le!(u27, 27);
    impl_read_le!(u28, 28);
    impl_read_le!(u29, 29);
    impl_read_le!(u30, 30);
    impl_read_le!(u31, 31);
    impl_read_le!(u32, 32);
}

fn big_endian_read_helper<T: BitTraits, const N: usize>(buf: &[u1; N]) -> T
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

    fn read_u10(buf: &[u1; 10]) -> u10 {
        big_endian_read_helper(buf)
    }

    fn read_u11(buf: &[u1; 11]) -> u11 {
        big_endian_read_helper(buf)
    }

    fn read_u12(buf: &[u1; 12]) -> u12 {
        big_endian_read_helper(buf)
    }

    fn read_u13(buf: &[u1; 13]) -> u13 {
        big_endian_read_helper(buf)
    }

    fn read_u14(buf: &[u1; 14]) -> u14 {
        big_endian_read_helper(buf)
    }

    fn read_u15(buf: &[u1; 15]) -> u15 {
        big_endian_read_helper(buf)
    }

    fn read_u16(buf: &[u1; 16]) -> u16 {
        big_endian_read_helper(buf)
    }

    fn read_u17(buf: &[u1; 17]) -> u17 {
        big_endian_read_helper(buf)
    }

    fn read_u18(buf: &[u1; 18]) -> u18 {
        big_endian_read_helper(buf)
    }

    fn read_u19(buf: &[u1; 19]) -> u19 {
        big_endian_read_helper(buf)
    }

    fn read_u20(buf: &[u1; 20]) -> u20 {
        big_endian_read_helper(buf)
    }

    fn read_u21(buf: &[u1; 21]) -> u21 {
        big_endian_read_helper(buf)
    }

    fn read_u22(buf: &[u1; 22]) -> u22 {
        big_endian_read_helper(buf)
    }

    fn read_u23(buf: &[u1; 23]) -> u23 {
        big_endian_read_helper(buf)
    }

    fn read_u24(buf: &[u1; 24]) -> u24 {
        big_endian_read_helper(buf)
    }

    fn read_u25(buf: &[u1; 25]) -> u25 {
        big_endian_read_helper(buf)
    }

    fn read_u26(buf: &[u1; 26]) -> u26 {
        big_endian_read_helper(buf)
    }

    fn read_u27(buf: &[u1; 27]) -> u27 {
        big_endian_read_helper(buf)
    }

    fn read_u28(buf: &[u1; 28]) -> u28 {
        big_endian_read_helper(buf)
    }

    fn read_u29(buf: &[u1; 29]) -> u29 {
        big_endian_read_helper(buf)
    }

    fn read_u30(buf: &[u1; 30]) -> u30 {
        big_endian_read_helper(buf)
    }

    fn read_u31(buf: &[u1; 31]) -> u31 {
        big_endian_read_helper(buf)
    }

    fn read_u32(buf: &[u1; 32]) -> u32 {
        big_endian_read_helper(buf)
    }
}

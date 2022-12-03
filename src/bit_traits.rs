use std::ops::{ShlAssign, BitOrAssign};

use ux::*;

use paste::paste;

pub trait BitTraits: Default + ShlAssign<usize> + From<u1> + BitOrAssign<Self> {
    const BITS: usize;
    const ZERO: Self;
}

macro_rules! impl_bit_traits_for_ux {
    ($type:ty, $num_bits:expr) => {
        paste! {
            impl BitTraits for $type {
                const BITS: usize = $num_bits;
                const ZERO: Self = $type::new(0);
            }
        }
    };
}

impl BitTraits for u8 {
    const BITS: usize = 8;
    const ZERO: Self = 0;
}

impl BitTraits for u16 {
    const BITS: usize = 16;
    const ZERO: Self = 0;
}

impl BitTraits for u32 {
    const BITS: usize = 32;
    const ZERO: Self = 0;
}

impl_bit_traits_for_ux!(u1, 1);
impl_bit_traits_for_ux!(u2, 1);
impl_bit_traits_for_ux!(u3, 1);
impl_bit_traits_for_ux!(u4, 1);
impl_bit_traits_for_ux!(u5, 1);
impl_bit_traits_for_ux!(u6, 1);
impl_bit_traits_for_ux!(u7, 1);
impl_bit_traits_for_ux!(u9, 1);
impl_bit_traits_for_ux!(u10, 10);
impl_bit_traits_for_ux!(u11, 11);
impl_bit_traits_for_ux!(u12, 12);
impl_bit_traits_for_ux!(u13, 13);
impl_bit_traits_for_ux!(u14, 14);
impl_bit_traits_for_ux!(u15, 15);
impl_bit_traits_for_ux!(u17, 17);
impl_bit_traits_for_ux!(u18, 18);
impl_bit_traits_for_ux!(u19, 19);
impl_bit_traits_for_ux!(u20, 20);
impl_bit_traits_for_ux!(u21, 21);
impl_bit_traits_for_ux!(u22, 22);
impl_bit_traits_for_ux!(u23, 23);
impl_bit_traits_for_ux!(u24, 24);
impl_bit_traits_for_ux!(u25, 25);
impl_bit_traits_for_ux!(u26, 26);
impl_bit_traits_for_ux!(u27, 27);
impl_bit_traits_for_ux!(u28, 28);
impl_bit_traits_for_ux!(u29, 29);
impl_bit_traits_for_ux!(u30, 30);
impl_bit_traits_for_ux!(u31, 31);

use std::ops::{BitAnd, BitOrAssign, ShlAssign, ShrAssign};

use ux::*;

use paste::paste;

/// A helper trait to simplify macro definitions for common operations
pub trait BitTraits:
    Default
    + ShlAssign<usize>
    + ShrAssign<usize>
    + From<u1>
    + BitOrAssign<Self>
    + BitAnd<Self, Output = Self>
    + PartialEq
    + Copy
{
    const BITS: usize;
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_bit_traits_for_ux {
    ($type:ty, $num_bits:expr) => {
        paste! {
            impl BitTraits for $type {
                const BITS: usize = $num_bits;
                const ZERO: Self = $type::new(0);
                const ONE: Self = $type::new(1);
            }
        }
    };
}

impl BitTraits for u8 {
    const BITS: usize = 8;
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl BitTraits for u16 {
    const BITS: usize = 16;
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl BitTraits for u32 {
    const BITS: usize = 32;
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl_bit_traits_for_ux!(u1, 1);
impl_bit_traits_for_ux!(u2, 2);
impl_bit_traits_for_ux!(u3, 3);
impl_bit_traits_for_ux!(u4, 4);
impl_bit_traits_for_ux!(u5, 5);
impl_bit_traits_for_ux!(u6, 6);
impl_bit_traits_for_ux!(u7, 7);
impl_bit_traits_for_ux!(u9, 9);
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

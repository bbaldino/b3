use std::ops::{Shl, Shr, ShrAssign};

use ux::*;

use crate::bit_write::BitWrite;

// Note: found it very difficult to implement a generic helper, mainly because of the masking
// required to isolate and compare the LSB.  Even still, the implementation of these feels pretty
// bad...
// A macro helper also had problems, because of the paste! issue
// (https://github.com/dtolnay/paste/issues/90)

// TODO: use BitTraits here to make life easier

fn bit_write_exts_write_helper<T, const N: usize, U: BitWrite + ?Sized>(buf: &mut U, mut value: T) -> std::io::Result<()>
where
    T: Shl<usize> + Shr<usize> + Copy + ShrAssign<usize>,
    <T as Shr<usize>>::Output: Shl<usize>,
    <<T as Shr<usize>>::Output as Shl<usize>>::Output: PartialEq<T>
{
    let mut arr = [u1::default(); N];
    let index_offset = N - 1;
    for i in 0..N {
        // Shifting right, then left, then comparing was the most generic way I could think of to
        // test for the value of the LSB.
        let unshifted = value;
        let shifted = (value >> 1usize) << 1usize;
        let bit = if shifted == unshifted {
            u1::new(0)
        } else {
            u1::new(1)
        };
        arr[index_offset - i] = bit;
        value >>= 1;
    }
    buf.write_all(&arr)
}

pub trait BitWriteExts: BitWrite {
    fn write_u1(&mut self, value: u1) -> std::io::Result<()> {
        self.write_all(&[value])
    }

    fn write_u2(&mut self, value: u2) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u2, 2, Self>(self, value) 
    }

    fn write_u3(&mut self, value: u3) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u3, 3, Self>(self, value) 
    }

    fn write_u4(&mut self, value: u4) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u4, 4, Self>(self, value) 
    }

    fn write_u5(&mut self, value: u5) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u5, 5, Self>(self, value) 
    }

    fn write_u6(&mut self, value: u6) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u6, 6, Self>(self, value) 
    }

    fn write_u7(&mut self, value: u7) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u7, 7, Self>(self, value) 
    }

    fn write_u8(&mut self, value: u8) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u8, 8, Self>(self, value) 
    }

    fn write_u9(&mut self, value: u9) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u9, 9, Self>(self, value) 
    }

    fn write_u10(&mut self, value: u10) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u10, 10, Self>(self, value) 
    }

    fn write_u11(&mut self, value: u11) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u11, 11, Self>(self, value) 
    }

    fn write_u12(&mut self, value: u12) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u12, 12, Self>(self, value) 
    }

    fn write_u13(&mut self, value: u13) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u13, 13, Self>(self, value) 
    }

    fn write_u14(&mut self, value: u14) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u14, 14, Self>(self, value) 
    }

    fn write_u15(&mut self, value: u15) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u15, 15, Self>(self, value) 
    }

    fn write_u16(&mut self, value: u16) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u16, 16, Self>(self, value) 
    }

    fn write_u17(&mut self, value: u17) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u17, 17, Self>(self, value) 
    }

    fn write_u18(&mut self, value: u18) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u18, 18, Self>(self, value) 
    }

    fn write_u19(&mut self, value: u19) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u19, 19, Self>(self, value) 
    }

    fn write_u20(&mut self, value: u20) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u20, 20, Self>(self, value) 
    }

    fn write_u21(&mut self, value: u21) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u21, 21, Self>(self, value) 
    }

    fn write_u22(&mut self, value: u22) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u22, 22, Self>(self, value) 
    }

    fn write_u23(&mut self, value: u23) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u23, 23, Self>(self, value) 
    }

    fn write_u24(&mut self, value: u24) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u24, 24, Self>(self, value) 
    }

    fn write_u25(&mut self, value: u25) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u25, 25, Self>(self, value) 
    }

    fn write_u26(&mut self, value: u26) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u26, 26, Self>(self, value) 
    }

    fn write_u27(&mut self, value: u27) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u27, 27, Self>(self, value) 
    }

    fn write_u28(&mut self, value: u28) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u28, 28, Self>(self, value) 
    }

    fn write_u29(&mut self, value: u29) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u29, 29, Self>(self, value) 
    }

    fn write_u30(&mut self, value: u30) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u30, 30, Self>(self, value) 
    }

    fn write_u31(&mut self, value: u31) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u31, 31, Self>(self, value) 
    }

    fn write_u32(&mut self, value: u32) -> std::io::Result<()> {
        bit_write_exts_write_helper::<u32, 32, Self>(self, value) 
    }
}

impl<T> BitWriteExts for T where T: BitWrite {}

#[cfg(test)]
mod tests {
    use crate::{bit_cursor::BitCursor, bitvec, bitarray};

    use super::*;

    #[test]
    fn test_bit_write_exts() {
        let vec = bitvec!(0; 2);
        let mut cursor = BitCursor::new(vec);

        assert!(cursor.write_u2(u2::new(1)).is_ok());
        assert_eq!(cursor.into_inner().get_slice(..), &bitarray!(0, 1)[..]);

        let vec = bitvec!(0; 12);
        let mut cursor = BitCursor::new(vec);
        assert!(cursor.write_u12(u12::new(0b110011001100)).is_ok());
        assert_eq!(cursor.into_inner().get_slice(..), &bitarray!(1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0)[..]);
    }
}

use std::ops::{ShlAssign, BitOrAssign};

use ux::*;

use crate::{bit_read::BitRead, byte_order::ByteOrder};

fn bit_read_exts_helper<T, const N: usize, U: BitRead>(buf: &mut U) -> T
where
    T: Default + ShlAssign<usize> + BitOrAssign + From<u1>,
    U: ?Sized
{

    let mut read_buf = [u1::default(); N];
    buf.read_exact(&mut read_buf).unwrap();
    let mut val = T::default();
    for bit in read_buf.iter() {
        val <<= 1;
        val |= (*bit).into();
    }
    val
}


pub trait BitReadExts: BitRead {
    fn read_u1(&mut self) -> u1 {
        bit_read_exts_helper::<u1, 1, Self>(self)
    }

    fn read_u2(&mut self) -> u2 {
        bit_read_exts_helper::<u2, 2, Self>(self)
    }

    fn read_u3(&mut self) -> u3 {
        bit_read_exts_helper::<u3, 3, Self>(self)
    }

    fn read_u4(&mut self) -> u4 {
        bit_read_exts_helper::<u4, 4, Self>(self)
    }

    fn read_u5(&mut self) -> u5 {
        bit_read_exts_helper::<u5, 5, Self>(self)
    }

    fn read_u6(&mut self) -> u6 {
        bit_read_exts_helper::<u6, 6, Self>(self)
    }

    fn read_u7(&mut self) -> u7 {
        bit_read_exts_helper::<u7, 7, Self>(self)
    }

    fn read_u8(&mut self) -> u8 {
        bit_read_exts_helper::<u8, 8, Self>(self)
    }

    fn read_u9<T: ByteOrder>(&mut self) -> u9 {
        let mut buf = [u1::new(0); 9];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u9(&buf)
    }

    fn read_u10<T: ByteOrder>(&mut self) -> u10 {
        let mut buf = [u1::new(0); 10];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u10(&buf)
    }

    fn read_u11<T: ByteOrder>(&mut self) -> u11 {
        let mut buf = [u1::new(0); 11];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u11(&buf)
    }

    fn read_u12<T: ByteOrder>(&mut self) -> u12 {
        let mut buf = [u1::new(0); 12];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u12(&buf)
    }

    fn read_u13<T: ByteOrder>(&mut self) -> u13 {
        let mut buf = [u1::new(0); 13];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u13(&buf)
    }

    fn read_u14<T: ByteOrder>(&mut self) -> u14 {
        let mut buf = [u1::new(0); 14];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u14(&buf)
    }

    fn read_u15<T: ByteOrder>(&mut self) -> u15 {
        let mut buf = [u1::new(0); 15];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u15(&buf)
    }

    fn read_u16<T: ByteOrder>(&mut self) -> u16 {
        let mut buf = [u1::new(0); 16];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u16(&buf)
    }

    fn read_u17<T: ByteOrder>(&mut self) -> u17 {
        let mut buf = [u1::new(0); 17];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u17(&buf)
    }

    fn read_u18<T: ByteOrder>(&mut self) -> u18 {
        let mut buf = [u1::new(0); 18];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u18(&buf)
    }

    fn read_u19<T: ByteOrder>(&mut self) -> u19 {
        let mut buf = [u1::new(0); 19];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u19(&buf)
    }

    fn read_u20<T: ByteOrder>(&mut self) -> u20 {
        let mut buf = [u1::new(0); 20];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u20(&buf)
    }

    fn read_u21<T: ByteOrder>(&mut self) -> u21 {
        let mut buf = [u1::new(0); 21];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u21(&buf)
    }

    fn read_u22<T: ByteOrder>(&mut self) -> u22 {
        let mut buf = [u1::new(0); 22];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u22(&buf)
    }

    fn read_u23<T: ByteOrder>(&mut self) -> u23 {
        let mut buf = [u1::new(0); 23];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u23(&buf)
    }

    fn read_u24<T: ByteOrder>(&mut self) -> u24 {
        let mut buf = [u1::new(0); 24];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u24(&buf)
    }

    fn read_u25<T: ByteOrder>(&mut self) -> u25 {
        let mut buf = [u1::new(0); 25];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u25(&buf)
    }

    fn read_u26<T: ByteOrder>(&mut self) -> u26 {
        let mut buf = [u1::new(0); 26];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u26(&buf)
    }

    fn read_u27<T: ByteOrder>(&mut self) -> u27 {
        let mut buf = [u1::new(0); 27];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u27(&buf)
    }

    fn read_u28<T: ByteOrder>(&mut self) -> u28 {
        let mut buf = [u1::new(0); 28];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u28(&buf)
    }

    fn read_u29<T: ByteOrder>(&mut self) -> u29 {
        let mut buf = [u1::new(0); 29];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u29(&buf)
    }

    fn read_u30<T: ByteOrder>(&mut self) -> u30 {
        let mut buf = [u1::new(0); 30];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u30(&buf)
    }

    fn read_u31<T: ByteOrder>(&mut self) -> u31 {
        let mut buf = [u1::new(0); 31];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u31(&buf)
    }

    fn read_u32<T: ByteOrder>(&mut self) -> u32 {
        let mut buf = [u1::new(0); 32];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u32(&buf)
    }
}

impl<T> BitReadExts for T where T: BitRead {}

#[cfg(test)]
mod tests {
    use crate::{bit_cursor::BitCursor, bitvec};

    use super::*;

    #[test]
    fn test_name() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0);
        let mut cursor = BitCursor::new(vec);

        assert_eq!(cursor.read_u2(), u2::new(3));
        assert_eq!(cursor.read_u4(), u4::new(0b1100));
    }
}

use std::{
    io::{Seek, SeekFrom},
    ops::RangeBounds,
};

use ux::u1;

use crate::{
    bit_read::BitRead,
    bit_vec::BitVec,
    bit_write::BitWrite,
    slice::{BitSlice, BitSliceMut},
};

#[derive(Debug)]
pub struct BitCursor<T> {
    inner: T,
    pos: usize,
}

impl<T> BitCursor<T> {
    pub fn new(inner: T) -> BitCursor<T> {
        BitCursor { inner, pos: 0 }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn position(&self) -> usize {
        self.pos
    }
}

impl BitCursor<BitVec> {
    pub fn from_vec(vec: Vec<u8>) -> BitCursor<BitVec> {
        BitCursor {
            inner: BitVec::from_u8_vec(vec),
            pos: 0,
        }
    }

    pub fn remaining_slice(&self) -> BitSlice<'_> {
        let len = self.pos.min(self.inner.len());
        self.inner.get_slice(len..)
    }

    pub fn remaining_slice_mut(&mut self) -> BitSliceMut<'_> {
        let len = self.pos.min(self.inner.len());
        self.inner.get_slice_mut(len..)
    }

    pub fn sub_cursor<T: RangeBounds<usize>>(&self, range: T) -> BitCursor<BitSlice<'_>> {
        let slice = self.inner.get_slice(range);
        BitCursor {
            inner: slice,
            pos: 0,
        }
    }

    pub fn bits_remaining(&self) -> usize {
        self.remaining_slice().len()
    }

    pub fn bytes_remaining(&self) -> usize {
        self.bits_remaining() / 8
    }
}

impl BitCursor<BitSlice<'_>> {
    pub fn remaining_slice(&self) -> BitSlice<'_> {
        let len = self.pos.min(self.inner.len());
        self.inner.get_slice(len..)
    }

    pub fn bits_remaining(&self) -> usize {
        self.remaining_slice().len()
    }

    pub fn bytes_remaining(&self) -> usize {
        self.bits_remaining() / 8
    }
}

impl BitCursor<BitSliceMut<'_>> {
    pub fn remaining_slice(&self) -> BitSlice<'_> {
        let len = self.pos.min(self.inner.len());
        self.inner.get_slice(len..)
    }

    pub fn remaining_slice_mut(&mut self) -> BitSliceMut<'_> {
        let len = self.pos.min(self.inner.len());
        self.inner.get_slice_mut(len..)
    }

    pub fn bits_remaining(&self) -> usize {
        self.remaining_slice().len()
    }

    pub fn bytes_remaining(&self) -> usize {
        self.bits_remaining() / 8
    }
}

impl BitRead for BitCursor<BitVec> {
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        // Read buf.len() bits from pos to pos + buf.len() into buf
        let n = self.remaining_slice().len().min(buf.len());
        BitRead::read(&mut self.remaining_slice(), buf)?;
        self.pos += n;
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut [u1]) -> std::io::Result<()> {
        let n = buf.len();
        BitRead::read_exact(&mut self.remaining_slice(), buf)?;
        self.pos += n;

        Ok(())
    }
}

impl BitWrite for BitCursor<BitVec> {
    fn write(&mut self, buf: &[u1]) -> std::io::Result<usize> {
        let n = self.remaining_slice().len().min(buf.len());
        BitWrite::write(&mut self.remaining_slice_mut(), buf)?;
        self.pos += n;
        Ok(n)
    }

    fn write_all(&mut self, buf: &[u1]) -> std::io::Result<()> {
        let n = self.remaining_slice().len().min(buf.len());
        BitWrite::write_all(&mut self.remaining_slice_mut(), buf)?;
        self.pos += n;
        Ok(())
    }
}

impl Seek for BitCursor<BitVec> {
    fn seek(&mut self, style: std::io::SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n as usize;
                return Ok(self.pos as u64);
            }
            SeekFrom::End(n) => (self.inner.len() as u64, n),
            SeekFrom::Current(n) => (self.pos as u64, n),
        };
        match base_pos.checked_add_signed(offset) {
            Some(n) => {
                self.pos = n as usize;
                Ok(self.pos as u64)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

impl Seek for BitCursor<BitSlice<'_>> {
    fn seek(&mut self, style: std::io::SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n as usize;
                return Ok(self.pos as u64);
            }
            SeekFrom::End(n) => (self.inner.len() as u64, n),
            SeekFrom::Current(n) => (self.pos as u64, n),
        };
        match base_pos.checked_add_signed(offset) {
            Some(n) => {
                self.pos = n as usize;
                Ok(self.pos as u64)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

impl BitRead for BitCursor<BitSlice<'_>> {
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        // Read buf.len() bits from pos to pos + buf.len() into buf
        let n = self.remaining_slice().len().min(buf.len());
        BitRead::read(&mut self.remaining_slice(), buf)?;
        self.pos += n;
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut [u1]) -> std::io::Result<()> {
        let n = buf.len();
        BitRead::read_exact(&mut self.remaining_slice(), buf)?;
        self.pos += n;

        Ok(())
    }
}

impl BitWrite for BitCursor<BitSliceMut<'_>> {
    fn write(&mut self, buf: &[u1]) -> std::io::Result<usize> {
        let n = self.remaining_slice().len().min(buf.len());
        BitWrite::write(&mut self.remaining_slice_mut(), buf)?;
        self.pos += n;
        Ok(n)
    }

    fn write_all(&mut self, buf: &[u1]) -> std::io::Result<()> {
        let n = self.remaining_slice().len().min(buf.len());
        BitWrite::write_all(&mut self.remaining_slice_mut(), buf)?;
        self.pos += n;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ux::u1;

    use crate::{bit_read_exts::BitReadExts, bitarray, bitvec};

    #[test]
    fn test_read() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 1, 1);
        let mut cursor = BitCursor::new(vec);
        let mut read_buf = [u1::new(0); 2];

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
    }

    #[test]
    fn test_write() {
        let vec = bitvec!(0; 16);
        let mut cursor = BitCursor::new(vec);

        assert!(cursor.write(&bitarray!(0, 1, 1, 0)).is_ok());
        assert!(cursor.write(&bitarray!(0, 1, 1, 0)).is_ok());
        assert!(cursor.write(&bitarray!(0, 1, 1, 0)).is_ok());
        assert!(cursor.write(&bitarray!(0, 1, 1, 0)).is_ok());

        assert_eq!(
            cursor.into_inner().get_slice(..),
            &bitarray!(0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0)[..]
        );
    }

    #[test]
    fn test_seek() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0);
        let mut cursor = BitCursor::new(vec);

        cursor.seek(SeekFrom::Current(4)).unwrap();
        assert_eq!(cursor.read_u1().unwrap(), u1::new(0));
        cursor.seek(SeekFrom::End(-5)).unwrap();
        assert_eq!(cursor.read_u1().unwrap(), u1::new(1));
        cursor.seek(SeekFrom::Current(-5)).unwrap();
        assert_eq!(cursor.read_u1().unwrap(), u1::new(0));
    }
}

use std::{
    io::{Seek, SeekFrom},
    ops::RangeBounds,
};

use ux::u1;

use crate::{
    bit_buffer::{BitBuffer, BitBufferMut},
    bit_read::BitRead,
    bit_vec::BitVec,
    bit_write::BitWrite,
    error::B3Result,
    slice::{BitSlice, BitSliceMut},
    util::get_start_end_bit_index_from_range,
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

impl<T> BitCursor<T>
where
    T: BitBuffer,
{
    pub fn remaining_slice(&self) -> BitSlice<'_> {
        let len = self.pos.min(self.inner.len());
        // safety: we've just checked that len is valid
        self.inner.get_slice(len..).unwrap()
    }

    /// Get a "sub cursor", where the given range is calculated relative to the cursor's
    /// current position.
    pub fn sub_cursor<R: RangeBounds<usize>>(&self, range: R) -> B3Result<BitCursor<BitSlice<'_>>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.inner.len());
        let start_bit_index = start_bit_index + self.position();
        let end_bit_index = end_bit_index + self.position();
        let slice = self.inner.get_slice(start_bit_index..end_bit_index)?;
        Ok(BitCursor {
            inner: slice,
            pos: 0,
        })
    }

    pub fn bits_remaining(&self) -> usize {
        self.remaining_slice().len()
    }

    pub fn bytes_remaining(&self) -> usize {
        self.bits_remaining() / 8
    }
}

impl<T> BitCursor<T>
where
    T: BitBufferMut,
{
    pub fn remaining_slice_mut(&mut self) -> BitSliceMut<'_> {
        let len = self.pos.min(self.inner.len());
        // safety: we've just checked that len is valid
        self.inner.get_slice_mut(len..).unwrap()
    }

    pub fn sub_cursor_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> B3Result<BitCursor<BitSliceMut<'_>>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.inner.len());
        let start_bit_index = start_bit_index + self.position();
        let end_bit_index = end_bit_index + self.position();
        let slice = self.inner.get_slice_mut(start_bit_index..end_bit_index)?;
        Ok(BitCursor {
            inner: slice,
            pos: 0,
        })
    }
}

impl BitCursor<BitVec> {
    pub fn from_vec(vec: Vec<u8>) -> BitCursor<BitVec> {
        BitCursor {
            inner: BitVec::from_vec(vec),
            pos: 0,
        }
    }
}

impl<T> BitWrite for BitCursor<T>
where
    T: BitBufferMut,
{
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

impl<T> Seek for BitCursor<T>
where
    T: BitBuffer,
{
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

impl<T> BitRead for BitCursor<T>
where
    T: BitBuffer,
{
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

#[cfg(test)]
mod tests {
    use super::*;

    use ux::{u1, u3};

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
            cursor.into_inner().get_slice(..).unwrap(),
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

    #[test]
    fn test_sub_cursor() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0);
        let mut cursor = BitCursor::new(vec);

        let mut sub_cursor = cursor.sub_cursor(1..).expect("valid cursor");

        assert_eq!(sub_cursor.read_u3().unwrap(), u3::new(7));

        // Original cursor position should be in the same place
        assert_eq!(cursor.read_u1().unwrap(), u1::new(1));
    }
}

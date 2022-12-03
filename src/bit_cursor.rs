use ux::u1;

use crate::{
    bit_read::BitRead,
    bit_vec::BitVec,
    bit_write::BitWrite,
    slice::{BitSlice, BitSliceMut},
};

#[derive(Debug)]
pub struct BitCursor {
    inner: BitVec,
    pos: usize,
}

impl BitCursor {
    pub fn new(inner: BitVec) -> BitCursor {
        BitCursor { inner, pos: 0 }
    }

    pub fn into_inner(self) -> BitVec {
        self.inner
    }

    pub fn remaining_slice(&self) -> BitSlice<'_> {
        let len = self.pos.min(self.inner.len());
        println!("bitcursor::remaining_slice.  self.pos = {}, inner len = {}, result len = {len}", self.pos, self.inner.len());
        self.inner.get_slice(len..)
    }

    pub fn remaining_slice_mut(&mut self) -> BitSliceMut<'_> {
        let len = self.pos.min(self.inner.len());
        self.inner.get_slice_mut(len..)
    }
}

impl BitRead for BitCursor {
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        println!("read: cursor, curr position = {}, reading {} bits, remaining slice len: {}", self.pos, buf.len(), self.remaining_slice().len());
        // Read buf.len() bits from pos to pos + buf.len() into buf
        let n = self.remaining_slice().len().min(buf.len());
        BitRead::read(&mut self.remaining_slice(), buf)?;
        self.pos += n;
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut [u1]) -> std::io::Result<()> {
        println!("read_exact: cursor, curr position = {}, reading {} bits, remaining slice len: {}", self.pos, buf.len(), self.remaining_slice().len());
        let n = buf.len();
        BitRead::read_exact(&mut self.remaining_slice(), buf)?;
        self.pos += n;

        Ok(())
    }
}

impl BitWrite for BitCursor {
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

    use crate::{bitarray, bitvec, bit_read_exts::BitReadExts};

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
}

use crate::{bit_vec::BitVec, bit_read::BitRead, slice::BitSlice};

pub struct BitCursor {
    inner: BitVec,
    pos: usize,
}

impl BitCursor {
    pub fn new(inner: BitVec) -> BitCursor {
        BitCursor {
            inner,
            pos: 0
        }
    }

    pub fn remaining_slice(&self) -> BitSlice<'_> {
        let len = self.pos.min(self.inner.as_ref().len());
        self.inner.as_ref().get_slice(len..)
    }
}

impl BitRead for BitCursor {
    fn read(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        // Read buf.len() bits from pos to pos + buf.len() into buf
        let slice = self.inner.get_slice(self.pos..self.pos + buf.len());
        let mut bits_read = 0usize;
        for i in 0..buf.len() {
            buf[i] = slice.at(i);
            bits_read += 1
        }
        self.pos += bits_read;
        Ok(bits_read)
    }

    fn read_exact(&mut self, buf: &mut[ux::u1]) -> std::io::Result<()> {
        let n = buf.len();
        BitRead::read_exact(&mut self.remaining_slice(), buf)?;
        self.pos += n;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use ux::u1;

    use crate::bitvec;

    use super::*;

    #[test]
    fn test_name() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0);
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
    }
}

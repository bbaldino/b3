use std::{ops::{Index, IndexMut}, slice::SliceIndex, io::Cursor};

pub use ux::*;

#[derive(Debug)]
pub struct BitVec {
    // TODO: in the future look at using u8 or something else to be more efficient?
    buf: Vec<u1>
}

impl BitVec {
    pub fn new() -> BitVec { 
        BitVec {
            buf: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> BitVec {
        BitVec {
            buf: Vec::with_capacity(capacity)
        }
    }

    pub fn push<T: Into<u1>>(&mut self, value: T) {
        self.buf.push(value.into());
    }

    pub fn pop(&mut self) -> Option<u1> {
        self.buf.pop()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}


// Ideally, I could just implement Index & IndexMut and get the proper slice type (&[u1]): the
// problem, though, is that I think this requires that the 'backing type' (the one held in BitVec
// itself) to also store u1's directly (or else, what would the slice be backed by?).  I think this
// is why the other BitVec crate uses a separate BitSlice type: it can then be backed by u8s, but
// still act like a 'bit slice'.  so will try that.

// TODO: would be nice to expand this to validate only 0s and 1s were passed
#[macro_export]
macro_rules! bitvec {
    ($($x:expr),+ $(,)?) => {
        $crate::into_bitvec(&[$($x),+])
    };
    ($elem:expr; $n:expr) => {
        $crate::from_elem($elem, $n)
    }
}

pub fn into_bitvec(data: &[u8]) -> BitVec {
    let mut vec = BitVec::with_capacity(data.len());
    for &val in data {
        vec.push(u1::new(val));
    }
    vec
}

pub fn from_elem(elem: u8, n: usize) -> BitVec {
    let mut vec = BitVec::with_capacity(n);
    for _ in 0..n {
        vec.push(u1::new(elem));
    }
    vec
}

impl<T> Index<T> for BitVec where T: SliceIndex<[u1]> {
    type Output = T::Output;

    fn index(&self, index: T) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T> IndexMut<T> for BitVec where T: SliceIndex<[u1]> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

pub trait BitRead {
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut vec = BitVec::new();

        assert!(vec.pop().is_none());
        vec.push(u1::new(1));
        vec.push(u1::new(0));
        assert_eq!(vec.pop().unwrap(), u1::new(0));
    }
}

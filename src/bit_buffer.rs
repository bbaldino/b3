use std::ops::RangeBounds;

use crate::{
    error::B3Result,
    slice::{BitSlice, BitSliceMut},
};

pub trait BitBuffer {
    fn len(&self) -> usize;
    fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>>;
}

pub trait BitBufferMut: BitBuffer {
    fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> B3Result<BitSliceMut<'_>>;
}

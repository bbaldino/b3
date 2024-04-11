use std::ops::RangeBounds;

use crate::{
    error::B3Result,
    slice::{BitSlice, BitSliceMut},
};

/// [`BitBuffer`] is a trait leveraged by [`BitCursor`] for reading data.
#[allow(clippy::len_without_is_empty)]
pub trait BitBuffer {
    fn len(&self) -> usize;
    fn get_slice<T: RangeBounds<usize>>(&self, range: T) -> B3Result<BitSlice<'_>>;
}

/// [`BitBufferMut`] is a trait leveraged by [`BitCursor`] for writing data.
pub trait BitBufferMut: BitBuffer {
    fn get_slice_mut<T: RangeBounds<usize>>(&mut self, range: T) -> B3Result<BitSliceMut<'_>>;
}

use thiserror::Error;

pub type B3Result<T> = Result<T, B3Error>;

#[derive(Error, Debug, PartialEq)]
pub enum B3Error {
    #[error("Tried to take a slice from array of length {len} from {slice_start} to {slice_end}")]
    SliceOutOfRange {
        len: usize,
        slice_start: usize,
        slice_end: usize,
    },
}

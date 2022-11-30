use ux::*;
use paste::paste;

use crate::{bit_read::BitRead, byte_order::ByteOrder};

macro_rules! impl_bit_read {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](&mut self) -> $type {
                let mut buf = [u1::new(0); $size_bits];
                self.read_exact(&mut buf).unwrap();
                let mut val = <$type>::new(0);
                for i in 0..$size_bits {
                    val <<= 1;
                    val |= buf[i].into();
                }
                val
            }
        }
    };
}

// /// Macro for generating BitReadExts read methods for types larger than 8 bits (which require a
// /// ByteOrder parameter).
// /// TODO: this doesn't work, perhaps because paste! doesn't like the nested brackets.
// /// See https://github.com/dtolnay/paste/issues/90
// macro_rules! impl_bit_read_bo {
//     ($type:ty, $size_bits:expr) => {
//         paste! {
//             fn [<read_ $type>]<T: ByteOrder>(&mut self) -> $type {
//                 let mut buf = [u1::new(0); $size_bits];
//                 self.read_exact(&mut buf).unwrap();
//                 <T>::read_u9(&buf)
//                 //[< <T>::read_ $type>](&buf)
//             }
//         }
//     };
// }


pub trait BitReadExts : BitRead {
    impl_bit_read!(u2, 2);
    impl_bit_read!(u3, 3);
    impl_bit_read!(u4, 4);
    impl_bit_read!(u5, 5);

    // See note on impl_bit_read_bo as to why we can't use the macro
    fn read_u9<T: ByteOrder>(&mut self) -> u9 {
        let mut buf = [u1::new(0); 9];
        self.read_exact(&mut buf).unwrap();
        <T>::read_u9(&buf)
    }
}

impl<T> BitReadExts for T where T: BitRead {}

#[cfg(test)]
mod tests {
    use crate::{bitvec, bit_cursor::BitCursor};

    use super::*;

    #[test]
    fn test_name() {
        let vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0);
        let mut cursor = BitCursor::new(vec);

        assert_eq!(cursor.read_u2(), u2::new(3));
        assert_eq!(cursor.read_u4(), u4::new(0b1100));
    }
}

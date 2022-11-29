pub mod bit_vec;
pub mod bit_cursor;
pub mod bit_read;
pub mod slice;
mod util;
pub mod bit_read_exts;

pub use ux::*;

struct BitArray<const T: usize> {
    bits: [u1; T],
}

impl Into<u4> for BitArray<4> {
    fn into(self) -> u4 {
        let mut value = u4::new(0);
        for bit in self.bits {
            value |= bit.into();
            value <<= 1;
        }
        value
    }
}

// fn read_u4(&mut self) -> u4 {
//     let mut bits = [u1::new(0); 4];
// 
// 
//     let mut bits = BitArray { bits: [u1::new(0); 4] };
//     self.read_exact(&mut bits);
//     bits.into()
// }


// impl BitSliceMut<'_> {
//     pub fn len(&self) -> usize {
//         self.end_bit_index - self.start_bit_index
//     }
// 
//     // We can't use the Index trait, because it returns a reference, and since we have to create
//     // the u1 on the fly here, we can't return a ref to it.
//     pub fn at(&self, index: usize) -> u1 {
//         println!("bitslicemut: {:?}", self);
//         let bit_pos = self.start_bit_index + index;
//         let byte_pos = bit_pos / 8;
//         let byte = self.buf[byte_pos];
//         println!("bit_pos = {}, byte_post = {}", bit_pos, byte_pos);
//         get_bit(byte, bit_pos % 8)
//     }
// 
//     fn write(&mut self, buf: BitSliceMut) -> std::io::Result<usize> {
//         // TODO: can probably improve this.  for now just go through and write one bit at a time
//         todo!();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;


    //#[test]
    //fn test_name() {
    //    let mut vec = BitVec::new();

    //    assert!(vec.pop().is_none());
    //    vec.push(u1::new(1));
    //    vec.push(u1::new(0));
    //    vec.push(u1::new(1));
    //    println!("{:?}", vec);
    //    assert_eq!(vec.pop().unwrap(), u1::new(1));
    //    println!("{:?}", vec);
    //    let slice = vec.get_slice(1..3);
    //    println!("{:?}", slice);
    //}
}

# b3: brian's bit buffer

### Archived
Ultimately this crate was unsuccessful.  It sought out to accomplish something similar to [bitvec](https://github.com/ferrilab/bitvec) but for a more focused use case (without the BitOrder or BitStorage flexibility) as a tool to power my [bitcursor](https://github.com/bbaldino/bitcursor) crate.  Ultimately I found that the _magic_ achieved by the bitvec crate in order to accomplish treating a slice of u8 as though it were a slice of u1 to be quite complex and best left to the bitvec crate rather than be recreated, so bitcursor now leverages bitvec instead.

b3 is a library with utilities to make reading non-standard-width values (u1, u7, u16, etc.) from a buffer easier.  It leverages the non-standard-width types defined in the [uX](https://crates.io/crates/ux) crate.  

Note that there are many libraries out there that strive to accomplish this same thing and are likely more performant, idiomatic, and better named.  This project (currently, at least) is mainly something fun to play around with and learn Rust.

### BitVec

`BitVec` mimics `Vec` but contains discrete `u1` bit values in each position.  All APIs are centered around the `u1` type, which represents a single bit:

```
let mut vec = BitVec::new();
// Or, construct with capacity:
// BitVec::with_capacity(..);
// Or, use macro helper (supports (elem; count) and (elem1, elem2, ...) syntax)
// bitvec!(0; 10);
// bitvec!(1, 0, 1, 0, 1, 0);

vec.push(u1::new(0));
// Length is measured in bits
assert_eq!(vec.len(), 1));
// The index operator isn't supported, use 'at' instead to access an element
assert_eq!(vec.at(0), u1::new(0));
assert_eq!(vec.pop().unwrap(), u1::new(0));
```

### BitSlice, BitSliceMut
`BitVec` doesn't support taking a slice via the `Index` trait, so it defines `get_slice` and `get_slice_mut` methods which return `BitSlice` and `BitSliceMut`, respectively.

```
let mut vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0);
let slice = vec.get_slice(2..=5);
assert_eq!(slice.len(), 4);
assert_eq!(slice, bitarray!(1, 1, 0, 0));
```
```
let mut vec = bitvec!(1, 1, 1, 1, 0, 0, 0, 0);
let mut slice = vec.get_slice_mut(2..=5);
slice.set(0, u1::new(0));
assert_eq!(vec.at(2), u1::new(0));
```

### BitCursor
`BitCursor` mimics `std::io::Cursor`:
```
let mut vec = bitvec!(1, 0, 1, 0, 1, 0);
let mut cursor = BitCursor::new(vec);
```
It also allows taking a "sub cursor" which limits how much can be read or written
```
let mut vec = bitvec!(1, 0, 1, 0, 1, 0);
let mut cursor = BitCursor::new(vec);
let mut sub_cursor = cursor.sub_cursor(..2);
assert_eq!(sub_cursor.bits_remaining(), 2);
assert_eq!(sub_cursor.read_u2().unwrap(), u2::new(2));
assert_eq!(sub_cursor.bits_remaining(), 0);
// Original cursor position is unchanged
assert_eq!(cursor.bits_remaining(), 6);

```

### BitRead, BitWrite
`BitRead` and `BitWrite` are traits that mimic `std::io::Read` and `std::io::Write`, but define operations on the bit level:

#### Reading
```
let mut read_buf = &[u1::new(0); 10];
// Read up to 10 bits from the cursor into read_buf
cursor.read(&mut read_buf);
// Read exactly 10 bits from the cursor into read_buf or fail
cursor.read_exact(&mut read_buf);
```
#### Writing
```
let mut write_buf = bitarray!(1, 0, 1, 0, 1, 0);
// Write up to 10 bits from write_buf into cursor
cursor.write(&write_buf);
// Write exactly 10 bits from write_buf into cursor or fail
cursor.write_exact(&read_buf);
```

### BitReadExts, BitWriteExts
Like how the [byteorder](https://crates.io/crates/byteorder) crate provides extensions to `std::io::Read` and `std::io::Write` for reading integer types in big and little endian modes, b3 defines `BitReadExts` and `BitWriteExts` which extend the `BitRead` and `BitWrite` traits and allow reading specific uX type in different orders:

#### Reading
```
let u19_val = cursor.read_u19::<BigEndian>();
let u21_val = cursor.read_u21::<LittleEndian>();
// NetworkOrder is an alias for BigEndian
let u12_val = cursor.read_u12::<NetworkOrder>();
```
#### Writing
```
cursor.write_u19::<BigEndian>(u19::new(42));
cursor.write_u21::<LittleEndian>(u21::new(43));
// NetworkOrder is an alias for BigEndian
cursor.write_u12::<NetworkOrder>(u12::new(44));
```

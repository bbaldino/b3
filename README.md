# b3: brian's bit buffer

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

### Learnings
This project was largely done as a fun/learning exercise, and I think I did manage to learn a thing or two along the way:


##### The 'cursor' style scheme is likely not popular for a reason:
This crate defines the `BitCursor` type which tracks a position in a buffer that's being read from/written to.  The cursor is stateful
and always points to the position where the next read or write should take place.  I liked this cursor scheme, as it felt like a natural
way to make it easy to read and write.  I've found some sticky problems with it, though:

1. What type should the cursor have inside?

At first I started with `BitCursor` holding a `BitVec` inside of it.  This was nice: `BitCursor` didn't require a generic type and it was easy
to use.  I later hit upon a use case where I wanted to have a `BitCursor` track a positon in a `BitSlice`, though, so the hard-coded `BitVec`
no longer worked.

I then changed `BitCursor` to be generic; `std::io::Cursor` is also implemented this way, so I thought maybe this was on the right track anyway.
After making this change, though, things got even more painful: I had separate implementations `BitCursor` (for `BitCursor<BitVec>` and 
`BitCursor<BitSlice>`) which meant there was no 'standard' set of methods available on a `BitCursor<T>`.  An impl of `BitCursor<T>` where `T: AsBitSlice` 
would've helped, but even then using the cursor in methods would've been awkward: the same constraint on T would've been required.  Ultimately
I found this to be very difficult and awkward to use as a type.

2. `BitCursor` always has to be mutable

A smaller nit, but also annoying.  Since advancing in the buffer requires modifying the cursor, even a "read-only" cursor requires mutable
references to be passed around.  I think it's cleaner when that's not required to make it more obvious when non-mutable operations are
taking place.


Other libs (Deku, serde, nom) instead take an approach of passing a slice to a read/write operation, which then returns a slice to the
'remaining' data (i.e. the data after what was just read/written).  This eliminates the need for a cursor-style type entirely.  I was
aware of the existence of those crates, but wanted to experiment with the 'cursor' style API to see how it felt.  I think I've come to
the conclusion that those other libs have the right idea :)

##### My 'BitSlice' implementation isn't smart enough
A bit-level buffer could be implemented by tracking something like a `Vec<bool>`, but in practice this is very clumsy to work with.  This
would mean that when you wanted to read from/write to a `Vec<u8>` (the common case when dealing with data from a network), you'd need to first
'explode' the `Vec<u8>` into a `Vec<bool>` or vice-versa, which is a waste of time.  So instead I hold the information as a `Vec<u8>` and then
index into it bit-by-bit to get what's needed.  This is nice because it avoids the conversion, but it brings with it some other headaches.

The main issue is: how do you take a slice of this data? `Index` requires a references to the output type (which, here, would be a bool/'u1'),
and we don't _have_ an actual array of bools/u1s to back it, so we can't return a reference to it.  My solution was to implement a `get_slice`
method which returned a `BitSlice` type.  `BitSlice` would hold a reference to part of the `BitVec`'s internal `Vec<u8>` corresponding to the
range to which it was supposed to point.  Since `BitSlice` holds a reference, it needs a lifetime parameter.  TODO: more concrete issues with
this?


Much superior to this is what's done in the [`bitvec` crate](https://github.com/ferrilab/bitvec), which does some clever work around separately
maintaining the pointer location to the relevant data held by the bitvec itself, and therefore doesn't require a lifetime annotation, making
them much more ergonomic to work with.  Similar to the situation with deku/nom/serde above, I knew about the existence of 
the `bitvec` crate before writing this, and part of the exercise was to take a shot at writing my own before digging into how `bitvec` worked, 
such that I could then compare the two.

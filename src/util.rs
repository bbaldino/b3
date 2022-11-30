use ux::u1;

/// Set the |bit_index| bit of |byte| to |value|
pub(crate) fn set_bit(byte: &mut u8, bit_index: usize, value: u1) {
    // Mask out bit_index
    // assign value to u8, shift it to the index
    // or value with byte
    let mask = match bit_index {
        0 => 0b01111111,
        1 => 0b10111111,
        2 => 0b11011111,
        3 => 0b11101111,
        4 => 0b11110111,
        5 => 0b11111011,
        6 => 0b11111101,
        7 => 0b11111110,
        _ => unreachable!()
    };
    *byte &= mask;
    let mut value: u8 = value.into();
    value <<= 7 - bit_index;
    *byte |= value;
}

/// Get the |bit_index| bits of |byte| as a u1
pub(crate) fn get_bit(byte: u8, bit_index: usize) -> u1 {
    let mask = match bit_index {
        0 => 0b10000000,
        1 => 0b01000000,
        2 => 0b00100000,
        3 => 0b00010000,
        4 => 0b00001000,
        5 => 0b00000100,
        6 => 0b00000010,
        7 => 0b00000001,
        _ => unreachable!()
    };
    let result = byte & mask;
    u1::new(result >> (7 - bit_index))
}

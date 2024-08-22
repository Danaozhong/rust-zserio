///  Gets the minimum number of bits required to encode given number of different values.
/// For example:
/// ```rust
/// use zserio::numbits;
/// assert!(numbits(0) == 0);
/// assert!(numbits(1) == 1);
/// assert!(numbits(7) == 3);
/// assert!(numbits(8) == 3);
/// assert!(numbits(9) == 4);
/// assert!(numbits(32) == 5);
/// assert!(numbits(33) == 6);
/// assert!(numbits(6917529027641081856u64) == 63);
/// assert!(numbits(std::u64::MAX) == 64);
/// assert!(numbits(std::u32::MAX) == 32);
/// ```
pub fn numbits<T: num::PrimInt>(value: T) -> u8 {
    if T::is_zero(&value) {
        return 0;
    }
    if T::is_one(&value) {
        return 1;
    }
    bit_length(value - T::from(1).expect("failed to subtract 1"))
}

// Calculates the bit length of an integer type, i.e. the numbers
// of bits needed to represent this number.
/// For example:
/// ```rust
/// use zserio::bit_length;
/// assert!(bit_length(0) == 0);
/// assert!(bit_length(1) == 1);
/// assert!(bit_length(7) == 3);
/// assert!(bit_length(8) == 4);
/// assert!(bit_length(8) == 4);
/// assert!(bit_length(32) == 6);
/// assert!(bit_length(6917529027641081856u64) == 63);
/// assert!(bit_length(std::u64::MAX) == 64);
/// assert!(bit_length(std::u32::MAX) == 32);
/// ```
pub fn bit_length<T: num::PrimInt>(value: T) -> u8 {
    let mut x = value.to_u64().expect("failed to convert to u64");
    let mut n = 0u8;
    if x >= 1 << 32 {
        x >>= 32;
        n = 32;
    }
    if x >= 1 << 16 {
        x >>= 16;
        n += 16
    }
    if x >= 1 << 8 {
        x >>= 8;
        n += 8;
    }
    n + BIT_LENGTH_BYTE.chars().nth(x as usize).unwrap() as u8
}

/// Returns the minimum number of bits required to represent x. the result is 0 for x == 0.
const BIT_LENGTH_BYTE: &str = "\x00\x01\x02\x02\x03\x03\x03\x03\x04\x04\x04\x04\x04\x04\x04\x04\
 \x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\x05\
 \x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\
 \x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\
 \x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\
 \x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\
 \x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\
 \x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\x07\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\
 \x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08";

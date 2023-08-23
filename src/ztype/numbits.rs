use std::mem;

// Calculates the bit length of an integer type, i.e. the numbers
// of bits needed to represent this number.
pub fn numbits<T: num::PrimInt>(value: T) -> u64 {
    (mem::size_of::<T>() - value.leading_zeros() as usize) as u64
}

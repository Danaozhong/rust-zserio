/// Aligns a bit count to the next multiply of align_to_bitcount.
///
/// # Example
///
/// ```
/// use zserio::ztype::align_to;
/// let bitcount = align_to(8, 15); // returns 16
/// ```
pub fn align_to(align_to_bitcount: u8, bit_position: u64) -> u64 {
    (((bit_position - 1) / align_to_bitcount as u64) + 1) * align_to_bitcount as u64
}

#[cfg(test)]
mod tests {
    use super::align_to;

    #[test]
    fn alignment() {
        assert_eq!(align_to(8, 15), 16);
        assert_eq!(align_to(5, 9), 10);
        assert_eq!(align_to(10, 990), 990);
    }
}

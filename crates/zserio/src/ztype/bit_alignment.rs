/**
* Aligns a bit count to the next multiply of align_to_bitcount.
   Example:
*  align_to(8, 15) = 16
*  align_to(5, 9) = 10
*  align_to(10, 990) = 990
*/
pub fn align_to(align_to_bitcount: u8, bit_position: u64) -> u64 {
    (((bit_position - 1) / align_to_bitcount as u64) + 1) * align_to_bitcount as u64
}

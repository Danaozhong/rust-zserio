use rust_bitwriter::BitWriter;

use crate::ztype::bits_encode;
use crate::ztype::varuint_encode::unsigned_bitsize;

use super::varuint_encode::write_varsize;

pub fn write_string(writer: &mut BitWriter, s: &str) {
    write_varsize(writer, s.len() as u64);

    for c in s.chars() {
        writer.write_u8(c as u8, 8).expect("failed to write string");
    }
}

pub fn bitsize_string(s: &str) -> u64 {
    let header_size = unsigned_bitsize(s.len() as u64, 5) as u64;
    header_size + (s.len() * 8) as u64
}

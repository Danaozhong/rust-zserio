use rust_bitwriter::BitWriter;

use crate::error::Result;
use crate::ztype::unsigned_bitsize;

use super::varuint_encode::write_varsize;

pub fn write_string(writer: &mut BitWriter, s: &str) -> Result<()> {
    write_varsize(writer, s.len() as u32);

    for c in s.chars() {
        writer.write_u8(c as u8, 8)?;
    }
    Ok(())
}

pub fn bitsize_string(s: &str) -> u64 {
    let header_size = unsigned_bitsize(s.len() as u64, 5).unwrap() as u64;
    header_size + (s.len() * 8) as u64
}

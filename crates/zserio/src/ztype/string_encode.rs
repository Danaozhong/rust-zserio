use rust_bitwriter::BitWriter;

use crate::error::Result;
use crate::ztype::unsigned_bitsize;

use super::varuint_encode::write_varsize;

pub fn write_string(writer: &mut BitWriter, s: &str) -> Result<()> {
    let string_bytes = s.as_bytes();

    write_varsize(writer, string_bytes.len() as u32)?;

    for string_byte in string_bytes {
        writer.write_u8(*string_byte, 8)?;
    }
    Ok(())
}

pub fn bitsize_string(s: &str) -> Result<u64> {
    let header_size = unsigned_bitsize(s.len() as u64, 5)? as u64;
    Ok(header_size + (s.len() * 8) as u64)
}

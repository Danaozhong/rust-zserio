use crate::error::Result;
use rust_bitwriter::BitWriter;

pub fn write_bytes(writer: &mut BitWriter, bytes: &Vec<u8>) -> Result<()> {
    for byte in bytes {
        writer.write_u8(*byte, 8)?;
    }
    Ok(())
}

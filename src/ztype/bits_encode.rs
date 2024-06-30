use crate::error::Result;
use rust_bitwriter::BitWriter;

pub fn write_unsigned_bits(writer: &mut BitWriter, v: u64, bits: u8) -> Result<()> {
    if bits > 64 {
        panic! {"cannot write more than 64 bits"};
    }
    Ok(writer.write_u64(v, bits)?)
}

pub fn write_signed_bits(writer: &mut BitWriter, v: i64, bits: u8) -> Result<()> {
    if bits > 64 {
        panic! {"cannot write more than 64 bits"};
    }
    Ok(writer.write_i64(v, bits)?)
}

pub fn write_uint64(writer: &mut BitWriter, v: u64) -> Result<()> {
    write_unsigned_bits(writer, v, 64)
}
pub fn write_uint32(writer: &mut BitWriter, v: u32) -> Result<()> {
    write_unsigned_bits(writer, v as u64, 32)
}
pub fn write_uint16(writer: &mut BitWriter, v: u16) -> Result<()> {
    write_unsigned_bits(writer, v as u64, 16)
}
pub fn write_uint8(writer: &mut BitWriter, v: u8) -> Result<()> {
    write_unsigned_bits(writer, v as u64, 8)
}

pub fn write_int64(writer: &mut BitWriter, v: i64) -> Result<()> {
    Ok(writer.write_i64(v, 64)?)
}
pub fn write_int32(writer: &mut BitWriter, v: i32) -> Result<()> {
    Ok(writer.write_i32(v, 32)?)
}
pub fn write_int16(writer: &mut BitWriter, v: i16) -> Result<()> {
    Ok(writer.write_i16(v, 16)?)
}
pub fn write_int8(writer: &mut BitWriter, v: i8) -> Result<()> {
    Ok(writer.write_i8(v, 8)?)
}

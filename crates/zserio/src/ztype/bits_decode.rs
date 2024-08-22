use crate::error::Result;
use bitreader::BitReader;

pub fn read_unsigned_bits(reader: &mut BitReader, bits: u8) -> Result<u64> {
    if bits > 64 {
        panic! {"cannot read more than 64 bits"};
    }
    Ok(reader.read_u64(bits)?)
}

pub fn read_signed_bits(reader: &mut BitReader, bits: u8) -> Result<i64> {
    if bits > 64 {
        panic! {"cannot read more than 64 bits"};
    }
    Ok(reader.read_i64(bits)?)
}

pub fn read_uint64(reader: &mut BitReader) -> Result<u64> {
    Ok(reader.read_u64(64)?)
}

pub fn read_uint32(reader: &mut BitReader) -> Result<u32> {
    Ok(reader.read_u32(32)?)
}
pub fn read_uint16(reader: &mut BitReader) -> Result<u16> {
    Ok(reader.read_u16(16)?)
}
pub fn read_uint8(reader: &mut BitReader) -> Result<u8> {
    Ok(reader.read_u8(8)?)
}

pub fn read_int64(reader: &mut BitReader) -> Result<i64> {
    Ok(reader.read_i64(64)?)
}
pub fn read_int32(reader: &mut BitReader) -> Result<i32> {
    Ok(reader.read_i32(32)?)
}
pub fn read_int16(reader: &mut BitReader) -> Result<i16> {
    Ok(reader.read_i16(16)?)
}
pub fn read_int8(reader: &mut BitReader) -> Result<i8> {
    Ok(reader.read_i8(8)?)
}

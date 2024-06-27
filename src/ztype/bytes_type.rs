use crate::error::Result;
use crate::ztype::reader::read_bytes;
use crate::ztype::writer::write_bytes;
use crate::ztype::{read_varsize, varsize_bitsize, write_varsize};
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

#[derive(Debug, PartialEq, Clone)]
pub struct BytesType {
    pub byte_size: u32,
    pub data_blob: Vec<u8>,
}

/// Reads an zserio bytes type from a bit stream.
pub fn read_bytes_type(bit_reader: &mut BitReader) -> Result<BytesType> {
    let byte_size = read_varsize(bit_reader)?;
    Ok(BytesType {
        byte_size,
        data_blob: read_bytes(bit_reader, byte_size)?,
    })
}

/// Writes a zserio bytes type to a bitstream.
pub fn write_bytes_type(bit_writer: &mut BitWriter, bytes_type: &BytesType) {
    write_varsize(bit_writer, bytes_type.byte_size);
    write_bytes(bit_writer, &bytes_type.data_blob);
}

pub fn bitsize_of_bytes(bytes_type: &BytesType) -> u64 {
    varsize_bitsize(bytes_type.byte_size).unwrap() as u64 + (bytes_type.byte_size as u64) * 8
}

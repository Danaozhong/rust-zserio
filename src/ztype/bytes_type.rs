use crate::ztype::reader::read_bytes;
use crate::ztype::writer::write_bytes;
use crate::ztype::{read_varsize, write_varsize};
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

#[derive(PartialEq)]
pub struct BytesType {
    pub byte_size: u64,
    pub data_blob: Vec<u8>,
}

/// Reads an zserio bytes type from a bit stream.
pub fn read_bytes_type(bit_reader: &mut BitReader) -> BytesType {
    let byte_size = read_varsize(bit_reader);
    BytesType {
        byte_size,
        data_blob: read_bytes(bit_reader, byte_size as u32),
    }
}

/// Writes a zserio bytes type to a bitstream.
pub fn write_bytes_type(bit_writer: &mut BitWriter, bytes_type: &BytesType) {
    write_varsize(bit_writer, bytes_type.byte_size);
    write_bytes(bit_writer, &bytes_type.data_blob);
}

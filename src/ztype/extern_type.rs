use crate::error::Result;
use crate::ztype::reader::read_bytes;
use crate::ztype::writer::write_bytes;
use crate::ztype::{read_varsize, varsize_bitsize, write_varsize};
use crate::ZserioError;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

#[derive(Debug, PartialEq, Clone)]
pub struct ExternType {
    pub bit_size: u32,
    pub data_blob: Vec<u8>,
}

/// Reads an zserio extern type from a bit stream.
pub fn read_extern_type(bit_reader: &mut BitReader) -> Result<ExternType> {
    let bit_size = read_varsize(bit_reader)?;
    let num_of_full_bytes = bit_size / 8;
    let remaining_bits = (bit_size % 8) as u8;

    let mut extern_bytes = read_bytes(bit_reader, num_of_full_bytes)?;
    if remaining_bits != 0 {
        let mut last_byte = bit_reader
            .read_u8(remaining_bits)
            .expect("failed to read last bits");
        let bit_shift = 8 - remaining_bits;
        last_byte <<= bit_shift;
        extern_bytes.push(last_byte);
    }
    Ok(ExternType {
        bit_size,
        data_blob: extern_bytes,
    })
}

/// Writes a zserio extern type to a bitstream.
pub fn write_extern_type(bit_writer: &mut BitWriter, extern_type: &ExternType) -> Result<()> {
    write_varsize(bit_writer, extern_type.bit_size)?;
    let num_of_full_bytes = (extern_type.bit_size / 8) as usize;
    let remaining_bits = (extern_type.bit_size % 8) as u8;
    write_bytes(
        bit_writer,
        &extern_type.data_blob[0..num_of_full_bytes].to_vec(),
    )?;
    if remaining_bits != 0 {
        let mut last_byte = *extern_type
            .data_blob
            .last()
            .ok_or(ZserioError::DataError("last byte for extern is missing"))?;
        let bit_shift = 8 - remaining_bits;
        last_byte >>= bit_shift;

        bit_writer.write_u8(last_byte, remaining_bits)?;
    }
    Ok(())
}

pub fn bitsize_of_extern(extern_type: &ExternType) -> u64 {
    varsize_bitsize(extern_type.bit_size).unwrap() as u64 + (extern_type.bit_size as u64)
}

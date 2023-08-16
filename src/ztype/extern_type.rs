use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct ExternType {
    pub bit_size: u64,
    pub data_blob: Vec<u8>,
}

pub fn read_extern(_bit_reader: &mut BitReader) -> ExternType {
    ExternType {
        bit_size: 0,
        data_blob: vec![],
    }
}

pub fn write_extern(_writer: &mut BitWriter, _extern_type: &ExternType) {}

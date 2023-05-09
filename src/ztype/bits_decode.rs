

use bitreader::BitReader;

pub fn read_unsigned_bits(reader: &mut BitReader, bits: u8) -> u64 {
    if bits > 64 {
        panic!{"cannot read more than 64 bits"};
    }
    reader.read_u64(bits).expect("failed to read bits")
}

pub fn read_signed_bits(reader: &mut BitReader, bits: u8) -> i64 {
    if bits > 64 {
        panic!{"cannot read more than 64 bits"};
    }
    reader.read_i64(bits).expect("failed to read bits")
}

pub fn read_u32(reader: &mut BitReader) -> u32 { read_unsigned_bits(reader, 32) as u32 }
pub fn read_u16(reader: &mut BitReader) -> u16 { read_unsigned_bits(reader, 16) as u16 }
pub fn read_u8(reader: &mut BitReader) -> u8 { read_unsigned_bits(reader, 8) as u8 }

pub fn read_i32(reader: &mut BitReader) -> i32 { read_signed_bits(reader, 32) as i32 }
pub fn read_i16(reader: &mut BitReader) -> i16 { read_signed_bits(reader, 16) as i16 }
pub fn read_i8(reader: &mut BitReader) -> i8 { read_signed_bits(reader, 8) as i8 }

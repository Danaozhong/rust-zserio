use bitreader::BitReader;

pub fn read_unsigned_bits(reader: &mut BitReader, bits: u8) -> u64 {
    if bits > 64 {
        panic! {"cannot read more than 64 bits"};
    }
    reader.read_u64(bits).expect("failed to read bits")
}

pub fn read_signed_bits(reader: &mut BitReader, bits: u8) -> i64 {
    if bits > 64 {
        panic! {"cannot read more than 64 bits"};
    }
    reader.read_i64(bits).expect("failed to read bits")
}

pub fn read_uint64(reader: &mut BitReader) -> u64 {
    reader.read_u64(64).unwrap()
}
pub fn read_uint32(reader: &mut BitReader) -> u32 {
    reader.read_u32(32).unwrap()
}
pub fn read_uint16(reader: &mut BitReader) -> u16 {
    reader.read_u16(16).unwrap()
}
pub fn read_uint8(reader: &mut BitReader) -> u8 {
    reader.read_u8(8).unwrap()
}

pub fn read_int64(reader: &mut BitReader) -> i64 {
    reader.read_i64(64).unwrap()
}
pub fn read_int32(reader: &mut BitReader) -> i32 {
    reader.read_i32(32).unwrap()
}
pub fn read_int16(reader: &mut BitReader) -> i16 {
    reader.read_i16(16).unwrap()
}
pub fn read_int8(reader: &mut BitReader) -> i8 {
    reader.read_i8(8).unwrap()
}

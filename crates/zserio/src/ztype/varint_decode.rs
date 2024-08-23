use crate::error::Result;
use bitreader::BitReader;

pub fn read_varint16(reader: &mut BitReader) -> Result<i16> {
    read_sized_int(reader, 2).map(|v| v as i16)
}

pub fn read_varint32(reader: &mut BitReader) -> Result<i32> {
    read_sized_int(reader, 4).map(|v| v as i32)
}

pub fn read_varint64(reader: &mut BitReader) -> Result<i64> {
    read_sized_int(reader, 8)
}

pub fn read_varint(reader: &mut BitReader) -> Result<i64> {
    read_sized_int(reader, 9)
}

fn read_sized_int(reader: &mut BitReader, bit_length: u8) -> Result<i64> {
    let mut b = reader.read_u8(8)?;
    let is_negative = (b & 0x80) != 0;
    let mut has_next_byte = (b & 0x40) != 0;
    let mut value = (b & (0x3f)) as i64;

    for _ in 0..bit_length - 2 {
        if !has_next_byte {
            break;
        }

        b = reader.read_u8(8)?;
        has_next_byte = (b & 0x80) != 0;
        value = (value << 7) | ((b & 0x7f) as i64);
    }
    if has_next_byte {
        b = reader.read_u8(8)?;
        value = (value << 8) | (b as i64);
    }
    if is_negative {
        value = -value;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0x0], 0)] // zero
    #[case(vec![0xff, 0xff, 0xff, 0xff], crate::ztype::limits::MIN_VARINT32)] // minimum
    #[case(vec![0x7f, 0xff, 0xff, 0xff], crate::ztype::limits::MAX_VARINT32)] // maximum
    fn test_read_varint32(#[case] input: Vec<u8>, #[case] expected: i32) {
        let slice_of_u8 = input.as_slice();
        let mut reader = BitReader::new(slice_of_u8);
        assert_eq!(read_varint32(&mut reader).unwrap(), expected);
    }
}

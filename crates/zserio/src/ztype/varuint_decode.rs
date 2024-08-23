use crate::error::Result;
use bitreader::BitReader;

pub fn read_varsize(reader: &mut BitReader) -> Result<u32> {
    read_sized_uint(reader, 5).map(|v| v as u32)
}

pub fn read_varuint16(reader: &mut BitReader) -> Result<u16> {
    read_sized_uint(reader, 2).map(|v| v as u16)
}

pub fn read_varuint32(reader: &mut BitReader) -> Result<u32> {
    read_sized_uint(reader, 4).map(|v| v as u32)
}

pub fn read_varuint64(reader: &mut BitReader) -> Result<u64> {
    read_sized_uint(reader, 8)
}

pub fn read_varuint(reader: &mut BitReader) -> Result<u64> {
    read_sized_uint(reader, 9)
}

fn read_sized_uint(reader: &mut BitReader, num_byes: u8) -> Result<u64> {
    let mut b = reader.read_u64(8)?;
    let mut has_next_byte = (b & 0x80) != 0;
    let mut value = b & (0x7f);

    for _ in 0..num_byes - 2 {
        if !has_next_byte {
            break;
        }

        b = reader.read_u64(8)?;
        has_next_byte = (b & 0x80) != 0;
        value = (value << 7) | (b & (0x7f));
    }
    if has_next_byte {
        b = reader.read_u64(8)?;
        value = (value << 8) | b;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    // Due to how rstest works rust will incorrectly warn about super::* being an
    // unused import.
    #![allow(unused_imports)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0x0, 0)] // minimum possible value
    #[case(0xFFFFFFFF, 536870911)] // maximum possible value
    fn test_read_varuint32(#[case] input: u32, #[case] expected: u32) {
        let slice_of_u8 = &input.to_be_bytes();
        let mut reader = BitReader::new(slice_of_u8);
        assert_eq!(read_varuint32(&mut reader).unwrap(), expected);
    }
}

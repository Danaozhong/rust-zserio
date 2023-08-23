use bitreader::BitReader;
use rstest::rstest;

pub fn read_varsize(reader: &mut BitReader) -> u32 {
    read_sized_uint(reader, 5) as u32
}

pub fn read_varuint16(reader: &mut BitReader) -> u16 {
    read_sized_uint(reader, 2) as u16
}

pub fn read_varuint32(reader: &mut BitReader) -> u32 {
    read_sized_uint(reader, 4) as u32
}

pub fn read_varuint64(reader: &mut BitReader) -> u64 {
    read_sized_uint(reader, 8)
}

pub fn read_varuint(reader: &mut BitReader) -> u64 {
    read_sized_uint(reader, 9)
}

fn read_sized_uint(reader: &mut BitReader, num_byes: u8) -> u64 {
    let mut b = reader.read_u64(8).unwrap();
    let mut has_next_byte = (b & 0x80) != 0;
    let mut value = b & (0x7f);

    for _ in 0..num_byes - 2 {
        if !has_next_byte {
            break;
        }

        b = reader.read_u64(8).unwrap();
        has_next_byte = (b & 0x80) != 0;
        value = (value << 7) | (b & (0x7f));
    }
    if has_next_byte {
        b = reader.read_u64(8).unwrap();
        value = (value << 8) | b;
    }
    value
}

mod tests {
    use super::*;

    #[rstest]
    #[case(0x0, 0)] // minimum possible value
    #[case(0xFFFFFFFF, 536870911)] // maximum possible value
    fn test_read_varuint32(#[case] input: u32, #[case] expected: u32) {
        let slice_of_u8 = &input.to_be_bytes();
        let mut reader = BitReader::new(slice_of_u8);
        assert_eq!(read_varuint32(&mut reader), expected);
    }
}

use bitreader::BitReader;
use rstest::rstest;

pub fn read_varint16(reader: &mut BitReader) -> i16 {
    read_sized_int(reader, 2) as i16
}

pub fn read_varint32(reader: &mut BitReader) -> i32 {
    read_sized_int(reader, 4) as i32
}

pub fn read_varint64(reader: &mut BitReader) -> i64 {
    read_sized_int(reader, 8)
}

pub fn read_varint(reader: &mut BitReader) -> i64 {
    read_sized_int(reader, 9)
}

fn read_sized_int(reader: &mut BitReader, num_byes: u8) -> i64 {
    let mut b = reader.read_u8(8).unwrap();
    let is_negative = (b & 0x80) != 0;
    let mut has_next_byte = (b & 0x40) != 0;
    let mut value = (b & (0x3f)) as i64;

    for _ in 0..num_byes - 2 {
        if !has_next_byte {
            break;
        }

        b = reader.read_u8(8).unwrap();
        has_next_byte = (b & 0x80) != 0;
        value = (value << 7) | ((b & 0x7f) as i64);
    }
    if has_next_byte {
        b = reader.read_u8(8).unwrap();
        value = (value << 8) | (b as i64);
    }
    if is_negative {
        value = -value;
    }
    value
}

mod tests {

    use super::*;

    #[rstest]
    #[case(vec![0x0], 0)] // zero
    #[case(vec![0xff, 0xff, 0xff, 0xff], crate::ztype::limits::MIN_VARINT32)] // minimum
    #[case(vec![0x7f, 0xff, 0xff, 0xff], crate::ztype::limits::MAX_VARINT32)] // maximum
    fn test_read_varint32(#[case] input: Vec<u8>, #[case] expected: i32) {
        let slice_of_u8 = input.as_slice();
        let mut reader = BitReader::new(slice_of_u8);
        assert_eq!(read_varint32(&mut reader), expected);
    }
}

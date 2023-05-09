
use bitreader::BitReader;
use rstest::rstest;

pub fn read_varuint32(mut reader: BitReader) -> u32 {
    let mut b = reader.read_u32(8).unwrap();
    let mut has_next_byte = (b & 0x80) != 0;
    let mut value = b & (0x7f);

    for _ in 0..2 {
        if !has_next_byte {
            break
        }
        
        b = reader.read_u32(8).unwrap();
        has_next_byte = (b & 0x80) != 0;
        value = (value << 7) | (b & (0x7f));
    }
    if has_next_byte {
        b = reader.read_u32(8).unwrap();
        value = (value << 8) | b;
    }
    value
}


pub fn read_varuint(mut reader: BitReader) -> u64 {
    let mut b = reader.read_u64(8).unwrap();
    let mut has_next_byte = (b & 0x80) != 0;
    let mut value = b & (0x7f);

    for _ in 0..7 {
        if !has_next_byte {
            break
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
        let slice_of_u8 =  &input.to_be_bytes();
        let reader = BitReader::new(slice_of_u8);
        assert_eq!(read_varuint32(reader), expected);
    } 
}
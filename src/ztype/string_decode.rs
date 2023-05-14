use bitreader::BitReader;

use crate::ztype::varuint_decode::read_varsize;

pub fn read_string(reader: &mut BitReader) -> String {
    // zserio first writes the string length as a varsize uint
    let str_len = read_varsize(reader) as usize;

    // create a buffer to store the string
    let mut buffer = vec![0_u8; str_len];
    reader
        .read_u8_slice(&mut buffer)
        .expect("failed to read string from buffer");
    String::from_utf8(buffer).unwrap()
}

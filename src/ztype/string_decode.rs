use bitreader::BitReader;

use crate::error::Result;
use crate::ztype::varuint_decode::read_varsize;

pub fn read_string(reader: &mut BitReader) -> Result<String> {
    // zserio first writes the string length as a varsize uint
    let str_len = read_varsize(reader).map(|v| v as usize).unwrap();

    // create a buffer to store the string
    let mut buffer = vec![0_u8; str_len];
    reader.read_u8_slice(&mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

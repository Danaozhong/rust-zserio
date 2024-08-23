use crate::error::Result;
use bitreader::BitReader;

pub fn read_bytes(reader: &mut BitReader, num_bytes: u32) -> Result<Vec<u8>> {
    let mut result = vec![];

    for _ in 0..num_bytes {
        result.push(reader.read_u8(8)?);
    }
    Ok(result)
}

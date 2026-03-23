use std::io::ErrorKind;

use crate::error::{Result, ZserioError};
use bitreader::BitReader;

pub fn read_bytes(reader: &mut BitReader, num_bytes: u32) -> Result<Vec<u8>> {
    if reader.remaining() < (num_bytes as u64 * 8) {
        return Err(ZserioError::IoError(std::io::Error::new(
            ErrorKind::UnexpectedEof,
            "requested byte count is larger than available data",
        )));
    }
    let mut result = vec![0_u8; num_bytes as usize];
    reader.read_u8_slice(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    // Due to how rstest works rust will incorrectly warn about super::* being an
    // unused import.
    #![allow(unused_imports)]
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_read_bytes_zero() {
        // num_bytes=0: no data should be read, even from an empty reader
        let data: &[u8] = &[];
        let mut reader = BitReader::new(data);
        assert_eq!(read_bytes(&mut reader, 0).unwrap(), vec![]);
    }

    #[rstest]
    // num_bytes=1: single byte
    #[case(&[0x42], 1, vec![0x42])]
    // num_bytes=3: multiple bytes returned in order
    #[case(&[0x01, 0x02, 0x03], 3, vec![0x01, 0x02, 0x03])]
    // num_bytes=2 out of 4 available: reads only the requested count
    #[case(&[0xDE, 0xAD, 0xBE, 0xEF], 2, vec![0xDE, 0xAD])]
    fn test_read_bytes(#[case] data: &[u8], #[case] num_bytes: u32, #[case] expected: Vec<u8>) {
        let mut reader = BitReader::new(data);
        assert_eq!(read_bytes(&mut reader, num_bytes).unwrap(), expected);
    }

    #[test]
    fn test_read_bytes_leaves_remainder() {
        // Read 2 bytes from a 3-byte buffer; 1 byte (8 bits) should remain
        let data: &[u8] = &[0xAA, 0xBB, 0xCC];
        let mut reader = BitReader::new(data);
        assert_eq!(read_bytes(&mut reader, 2).unwrap(), vec![0xAA, 0xBB]);
        assert_eq!(reader.remaining(), 8);
    }

    #[test]
    fn test_read_bytes_truncated() {
        // Request 5 bytes but only 2 are available
        let data: &[u8] = &[0x01, 0x02];
        let mut reader = BitReader::new(data);
        assert!(
            read_bytes(&mut reader, 5).is_err(),
            "expected an error when fewer bytes are available than requested"
        );
    }

    #[test]
    fn test_read_bytes_empty_reader() {
        // Reader has no data at all; any non-zero request must fail
        let data: &[u8] = &[];
        let mut reader = BitReader::new(data);
        assert!(
            read_bytes(&mut reader, 1).is_err(),
            "expected an error when reading from an empty reader"
        );
    }
}

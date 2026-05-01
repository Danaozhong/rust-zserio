use std::io::ErrorKind;

use bitreader::BitReader;

use crate::error::{Result, ZserioError};
use crate::ztype::varuint_decode::read_varsize;

pub fn read_string(reader: &mut BitReader) -> Result<String> {
    // zserio first writes the string length as a varsize uint
    let str_len = read_varsize(reader).map(|v| v as usize).unwrap();

    if reader.remaining() < (str_len as u64 * 8) {
        return Err(ZserioError::IoError(std::io::Error::new(
            ErrorKind::UnexpectedEof,
            "claimed string size is larger than available data",
        )));
    }
    // create a buffer to store the string
    let mut buffer = vec![0_u8; str_len];
    reader.read_u8_slice(&mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

#[cfg(test)]
mod tests {
    // Due to how rstest works rust will incorrectly warn about super::* being an
    // unused import.
    #![allow(unused_imports)]
    use super::*;
    use rstest::rstest;

    // ── happy-path cases ────────────────────────────────────────────────────

    #[test]
    fn test_read_string_empty() {
        // Wire format: varsize(0) = 0x00, no payload
        let data: &[u8] = &[0x00];
        let mut reader = BitReader::new(data);
        assert_eq!(read_string(&mut reader).unwrap(), "");
    }

    #[rstest]
    // Wire format: varsize(len) + UTF-8 bytes
    // "hi"          → 0x02 'h' 'i'
    // "hello"       → 0x05 'h' 'e' 'l' 'l' 'o'
    // "Hello, World!" → 0x0D + 13 ASCII bytes
    #[case(&[0x02, b'h', b'i'], "hi")]
    #[case(&[0x05, b'h', b'e', b'l', b'l', b'o'], "hello")]
    #[case(&[0x0D, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!'], "Hello, World!")]
    fn test_read_string_ascii(#[case] data: &[u8], #[case] expected: &str) {
        let mut reader = BitReader::new(data);
        assert_eq!(read_string(&mut reader).unwrap(), expected);
    }

    #[test]
    fn test_read_string_utf8_multibyte() {
        // "日本語" encodes to 9 UTF-8 bytes:
        //   日 = E6 97 A5
        //   本 = E6 9C AC
        //   語 = E8 AA 9E
        // Wire format: varsize(9) = 0x09, then the 9 bytes
        let data: &[u8] = &[0x09, 0xE6, 0x97, 0xA5, 0xE6, 0x9C, 0xAC, 0xE8, 0xAA, 0x9E];
        let mut reader = BitReader::new(data);
        assert_eq!(read_string(&mut reader).unwrap(), "日本語");
    }

    #[test]
    fn test_read_string_emoji() {
        // "hi 🌍" encodes to 7 UTF-8 bytes:
        //   'h' = 68, 'i' = 69, ' ' = 20
        //   🌍 = F0 9F 8C 8D (4 bytes)
        // Wire format: varsize(7) = 0x07, then the 7 bytes
        let data: &[u8] = &[0x07, b'h', b'i', b' ', 0xF0, 0x9F, 0x8C, 0x8D];
        let mut reader = BitReader::new(data);
        assert_eq!(read_string(&mut reader).unwrap(), "hi 🌍");
    }

    #[test]
    fn test_read_string_two_byte_varsize_length() {
        // A 200-byte string requires a 2-byte varsize prefix.
        // varsize(200): 200 = 0b1100_1000
        //   high 7 bits = 0b000_0001 → first byte  = 0x80 | 0x01 = 0x81
        //   low  7 bits = 0b100_1000 → second byte = 0x48
        // Payload: 200 × b'a'
        let mut data: Vec<u8> = vec![0x81, 0x48];
        data.extend_from_slice(&[b'a'; 200]);
        let mut reader = BitReader::new(&data);
        assert_eq!(read_string(&mut reader).unwrap(), "a".repeat(200));
    }

    #[test]
    fn test_read_string_leaves_remaining_bytes_unconsumed() {
        // "hi" followed by a trailing sentinel byte 0xAB.
        // Only the 2 declared bytes should be consumed.
        let data: &[u8] = &[0x02, b'h', b'i', 0xAB];
        let mut reader = BitReader::new(data);
        assert_eq!(read_string(&mut reader).unwrap(), "hi");
        assert_eq!(reader.remaining(), 8); // one byte left
    }

    // ── error cases ─────────────────────────────────────────────────────────

    #[test]
    fn test_read_string_truncated_payload() {
        // varsize claims 5 bytes but only 2 bytes of payload follow
        let data: &[u8] = &[0x05, b'a', b'b'];
        let mut reader = BitReader::new(data);
        assert!(
            read_string(&mut reader).is_err(),
            "expected an error when payload is shorter than declared length"
        );
    }

    #[test]
    fn test_read_string_length_exceeds_buffer() {
        // varsize claims 10 bytes but no payload follows at all
        let data: &[u8] = &[0x0A];
        let mut reader = BitReader::new(data);
        assert!(
            read_string(&mut reader).is_err(),
            "expected an error when payload is entirely missing"
        );
    }

    #[test]
    fn test_read_string_invalid_utf8() {
        // varsize(1) = 0x01, payload = 0xFF which is not valid UTF-8
        let data: &[u8] = &[0x01, 0xFF];
        let mut reader = BitReader::new(data);
        assert!(
            read_string(&mut reader).is_err(),
            "expected an error for invalid UTF-8 payload"
        );
    }
}

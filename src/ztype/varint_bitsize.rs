pub fn varint16_bitsize(v: i16) -> u8 {
    signed_bitsize(v as i64, 2)
}

pub fn varint32_bitsize(v: i32) -> u8 {
    signed_bitsize(v as i64, 4)
}

pub fn varint64_bitsize(v: i64) -> u8 {
    signed_bitsize(v, 8)
}

pub fn varint_bitsize(v: i64) -> u8 {
    signed_bitsize(v, 9)
}

pub fn signed_bitsize(mut v: i64, max_bytes: u8) -> u8 {
    if v < 0 {
        v = -v
    }

    let mut max: i64 = (1 << 6) - 1;

    let mut bytes = 1;
    loop {
        if v <= max {
            return bytes * 8;
        }
        bytes += 1;
        assert!(bytes <= max_bytes);

        if bytes == max_bytes {
            // The last byte can take 8 bits, since it does not need a
            // more-flag in the msb.
            max = (max << 8) | 0xff;
        } else {
            max = (max << 7) | 0xff;
        }
    }
}

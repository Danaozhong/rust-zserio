pub fn varuint16_bitsize(v: u16) -> u8 {
    unsigned_bitsize(v as u64, 2)
}

pub fn varuint32_bitsize(v: u32) -> u8 {
    unsigned_bitsize(v as u64, 4)
}

pub fn varuint64_bitsize(v: u64) -> u8 {
    unsigned_bitsize(v, 8)
}

pub fn varuint_bitsize(v: u64) -> u8 {
    unsigned_bitsize(v, 9)
}

pub fn varsize_bitsize(v: u32) -> u8 {
    unsigned_bitsize(v as u64, 5)
}

pub fn unsigned_bitsize(v: u64, max_bytes: u8) -> u8 {
    let mut max: u64 = (1 << 7) - 1;

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

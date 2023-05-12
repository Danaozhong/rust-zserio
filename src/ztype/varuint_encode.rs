
use rust_bitwriter::BitWriter;

const MAX_VARSIZE: u64 = (1 << (2 + 7 + 7 + 7 + 8)) - 1;


// write_varsize writes a zserio varsize value to the bitstream.
pub fn write_varsize(writer: &mut BitWriter, v: u64) {
    assert!(v <= MAX_VARSIZE);
	write_varuint(writer, v, 5)
}


pub fn write_varuint(writer: &mut BitWriter, v: u64, max_bytes: u8) {
    let needed_bytes = unsigned_bitsize(v, max_bytes) / 8;

    let needs_complete_bit_range = needed_bytes == max_bytes;
    for i in 0..needed_bytes {
        let mut b: u64 = 0x0;
        let mut remaining_bits = 8;
        let has_next_byte = i < (needed_bytes - 1);
        if has_next_byte {
            remaining_bits -= 1;
            b |= 0x80;
        } else if !needs_complete_bit_range {
            remaining_bits -= 1;
        }
        let mut shift_bits = (needed_bytes - (i + 1)) * 7;
		if needs_complete_bit_range && has_next_byte {
			shift_bits += 1;
		}
		b |= (v >> shift_bits as u64) & (0xff >> (8 - remaining_bits));
		writer.write_unsigned_bits(b, 8, 8);
    }
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
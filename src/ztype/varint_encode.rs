use crate::ztype::signed_bitsize;
use rust_bitwriter::BitWriter;

pub fn write_varint16(writer: &mut BitWriter, v: i16) {
    write_varint_type(writer, v as i64, 2)
}

pub fn write_varint32(writer: &mut BitWriter, v: i32) {
    write_varint_type(writer, v as i64, 4)
}

pub fn write_varint64(writer: &mut BitWriter, v: i64) {
    write_varint_type(writer, v, 8)
}

pub fn write_varint(writer: &mut BitWriter, v: i64) {
    write_varint_type(writer, v, 9)
}

pub fn write_varint_type(writer: &mut BitWriter, v: i64, max_bytes: u8) {
    let abs_value = if v < 0 { -v as u64 } else { v as u64 };

    let needed_bytes = signed_bitsize(v, max_bytes) / 8;

    let needs_complete_bit_range = needed_bytes == max_bytes;
    for i in 0..needed_bytes {
        let mut b: u64 = 0x0;
        let mut remaining_bits = 8;
        let has_next_byte = i < (needed_bytes - 1);
        let has_sign_bit = i == 0;
        if has_sign_bit {
            if v < 0 {
                b |= 0x80;
            }
            remaining_bits -= 1;
        }
        if has_next_byte {
            remaining_bits -= 1;
            b |= 1 << remaining_bits;
        } else if !needs_complete_bit_range {
            remaining_bits -= 1;
        }
        let mut shift_bits = (needed_bytes - (i + 1)) * 7;
        if needs_complete_bit_range && has_next_byte {
            shift_bits += 1;
        }
        b |= (abs_value >> shift_bits) & (0xff >> (8 - remaining_bits));
        let _ = writer.write_unsigned_bits(b, 8, 8);
    }
}

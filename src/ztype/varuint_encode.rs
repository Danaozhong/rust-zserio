use crate::ztype::unsigned_bitsize;
use rust_bitwriter::BitWriter;

const MAX_VARSIZE: u32 = (1 << (2 + 7 + 7 + 7 + 8)) - 1;

pub fn write_varuint16(writer: &mut BitWriter, v: u16) {
    write_varuint_type(writer, v as u64, 2)
}

pub fn write_varuint32(writer: &mut BitWriter, v: u32) {
    write_varuint_type(writer, v as u64, 4)
}

pub fn write_varuint64(writer: &mut BitWriter, v: u64) {
    write_varuint_type(writer, v, 8)
}

pub fn write_varuint(writer: &mut BitWriter, v: u64) {
    write_varuint_type(writer, v, 9)
}

// write_varsize writes a zserio varsize value to the bitstream.
pub fn write_varsize(writer: &mut BitWriter, v: u32) {
    assert!(v <= MAX_VARSIZE);
    write_varuint_type(writer, v as u64, 5)
}

pub fn write_varuint_type(writer: &mut BitWriter, v: u64, max_bytes: u8) {
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
        let _ = writer.write_unsigned_bits(b, 8, 8);
    }
}

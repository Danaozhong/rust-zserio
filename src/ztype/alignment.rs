use bitreader::BitReader;
use rust_bitwriter::BitWriter;

/// Aligns the bit reader to a bit boundary, e.g. alignment_bits == 8 would align to the next
/// byte boundary.
pub fn align_reader(reader: &mut BitReader, alignment_bits: u8) {
    let u64_alignment_bits = alignment_bits as u64;
    let cur_alignment = reader.position() % u64_alignment_bits;
    let bits_to_skip = (u64_alignment_bits - cur_alignment) % u64_alignment_bits;
    reader.skip(bits_to_skip).expect("failed to align reader");
}

pub fn align_writer(writer: &mut BitWriter, alignment_bits: u8) {
    let u64_alignment_bits = alignment_bits as u64;
    let cur_alignment = writer.bit_count() % u64_alignment_bits;
    let bits_to_skip = (u64_alignment_bits - cur_alignment) % u64_alignment_bits;
    writer.skip(bits_to_skip).expect("failed to align writer");
}

pub fn align_bitsize(bit_pos: u64, alignment_bits: u8) -> u64 {
    let u64_alignment_bits = alignment_bits as u64;
    let cur_alignment = bit_pos % u64_alignment_bits;
    (u64_alignment_bits - cur_alignment) % u64_alignment_bits
}

use rust_bitwriter::BitWriter;

pub fn write_unsigned_bits(writer: &mut BitWriter, v: u64, bits: u8) {
    if bits > 64 {
        panic! {"cannot write more than 64 bits"};
    }
    writer.write_u64(v, bits).expect("failed to write bits");
}

pub fn write_signed_bits(writer: &mut BitWriter, v: i64, bits: u8) {
    if bits > 64 {
        panic! {"cannot write more than 64 bits"};
    }
    writer.write_i64(v, bits).expect("failed to write bits");
}

pub fn write_u64(writer: &mut BitWriter, v: u64) {
    write_unsigned_bits(writer, v, 64);
}
pub fn write_u32(writer: &mut BitWriter, v: u32) {
    write_unsigned_bits(writer, v as u64, 32);
}
pub fn write_u16(writer: &mut BitWriter, v: u16) {
    write_unsigned_bits(writer, v as u64, 16);
}
pub fn write_u8(writer: &mut BitWriter, v: u8) {
    write_unsigned_bits(writer, v as u64, 8);
}

pub fn write_i64(writer: &mut BitWriter, v: i64) {
    writer.write_i64(v, 64).expect("failed to write i64");
}
pub fn write_i32(writer: &mut BitWriter, v: i32) {
    writer.write_i32(v, 32).expect("failed to write i32");
}
pub fn write_i16(writer: &mut BitWriter, v: i16) {
    writer.write_i16(v, 16).expect("failed to write i16");
}
pub fn write_i8(writer: &mut BitWriter, v: i8) {
    writer.write_i8(v, 8).expect("failed to write i8");
}

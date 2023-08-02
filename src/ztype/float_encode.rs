use half::f16;
use rust_bitwriter::BitWriter;

pub fn write_bytes(writer: &mut BitWriter, bytes: Vec<u8>) {
    for byte in bytes {
        writer.write_u8(byte, 8);
    }
}

pub fn write_f64(writer: &mut BitWriter, v: f64) {
    write_bytes(writer, f64::to_be_bytes(v).to_vec());
}

pub fn write_f32(writer: &mut BitWriter, v: f32) {
    write_bytes(writer, f32::to_be_bytes(v).to_vec());
}

pub fn write_f16(writer: &mut BitWriter, v: f32) {
    write_bytes(writer, f16::to_be_bytes(f16::from_f32(v)).to_vec());
}

use rust_bitwriter::BitWriter;

pub fn write_bytes(writer: &mut BitWriter, bytes: &Vec<u8>) {
    for byte in bytes {
        writer.write_u8(*byte, 8).expect("writing bytes failed");
    }
}

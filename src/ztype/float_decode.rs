use bitreader::BitReader;
use half::f16;

pub fn read_bytes(reader: &mut BitReader, num_bytes: u32) -> Vec<u8> {
    let mut result = vec![];

    for _ in 0..num_bytes {
        result.push(reader.read_u8(8).unwrap());
    }
    result
}

pub fn read_f64(reader: &mut BitReader) -> f64 {
    f64::from_be_bytes(
        read_bytes(reader, 8)
            .try_into()
            .expect("failed to convert to bytes"),
    )
}
pub fn read_f32(reader: &mut BitReader) -> f32 {
    f32::from_be_bytes(
        read_bytes(reader, 4)
            .try_into()
            .expect("failed to convert to bytes"),
    )
}
pub fn read_f16(reader: &mut BitReader) -> f32 {
    f16::from_be_bytes(
        read_bytes(reader, 2)
            .try_into()
            .expect("failed to convert to bytes"),
    )
    .to_f32()
}

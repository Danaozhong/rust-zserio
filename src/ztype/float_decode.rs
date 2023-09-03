use crate::ztype::reader::read_bytes;
use bitreader::BitReader;
use half::f16;

pub fn read_float64(reader: &mut BitReader) -> f64 {
    f64::from_be_bytes(
        read_bytes(reader, 8)
            .try_into()
            .expect("failed to convert to bytes"),
    )
}
pub fn read_float32(reader: &mut BitReader) -> f32 {
    f32::from_be_bytes(
        read_bytes(reader, 4)
            .try_into()
            .expect("failed to convert to bytes"),
    )
}
pub fn read_float16(reader: &mut BitReader) -> f32 {
    f16::from_be_bytes(
        read_bytes(reader, 2)
            .try_into()
            .expect("failed to convert to bytes"),
    )
    .to_f32()
}

use crate::error::Result;
use crate::ztype::reader::read_bytes;
use crate::ZserioError;
use bitreader::BitReader;
use half::f16;

pub fn read_float64(reader: &mut BitReader) -> Result<f64> {
    Ok(f64::from_be_bytes(read_bytes(reader, 8)?.try_into().or(
        Err(ZserioError::DataError("can not convert bytes to float64")),
    )?))
}
pub fn read_float32(reader: &mut BitReader) -> Result<f32> {
    Ok(f32::from_be_bytes(read_bytes(reader, 4)?.try_into().or(
        Err(ZserioError::DataError("can not convert bytes to float32")),
    )?))
}
pub fn read_float16(reader: &mut BitReader) -> Result<f32> {
    Ok(f16::from_be_bytes(
        read_bytes(reader, 2)?
            .try_into()
            .or(Err(ZserioError::DataError(
                "can not convert bytes to float16",
            )))?,
    )
    .to_f32())
}

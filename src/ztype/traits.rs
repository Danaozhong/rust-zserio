use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub trait ZserioPackableOject {
    fn new() -> Self;
    fn marshal_zserio(&self, writer: &mut BitWriter);
    fn unmarshal_zserio(&mut self, reader: &mut BitReader);
    fn zserio_bitsize(&self, bit_position: u64) -> u64;
}

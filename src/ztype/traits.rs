use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub trait ZserioPackableOject {
    fn new() -> Self;
    fn zserio_read(&mut self, reader: &mut BitReader);
    fn zserio_write(&self, writer: &mut BitWriter);
    fn zserio_read_packed(&mut self, context_node: &mut PackingContextNode, reader: &mut BitReader);
    fn zserio_write_packed(&self, context_node: &mut PackingContextNode, writer: &mut BitWriter);
    fn zserio_bitsize(&self, bit_position: u64) -> u64;
}

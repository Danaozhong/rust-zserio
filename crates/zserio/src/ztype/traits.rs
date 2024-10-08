use crate::error::Result;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub trait ZserioPackableObject: Default {
    fn zserio_read(&mut self, reader: &mut BitReader) -> Result<()>;
    fn zserio_write(&self, writer: &mut BitWriter) -> Result<()>;
    fn zserio_read_packed(
        &mut self,
        context_node: &mut PackingContextNode,
        reader: &mut BitReader,
    ) -> Result<()>;
    fn zserio_write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
    ) -> Result<()>;
    fn zserio_bitsize(&self, bit_position: u64) -> Result<u64>;
    fn zserio_bitsize_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
    ) -> Result<u64>;
    fn zserio_create_packing_context(context_node: &mut PackingContextNode);
    fn zserio_init_packing_context(&self, context_node: &mut PackingContextNode) -> Result<()>;
}

use crate::ztype;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct VarUint32ArrayTrait {}

impl array_trait::ArrayTrait<u32> for VarUint32ArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, value: &u32) -> u64 {
        ztype::varuint32_bitsize(*value) as u64
    }

    fn initialize_offsets(&self, bit_position: u64, value: &u32) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader) -> u32 {
        ztype::read_varuint32(reader)
    }

    fn write(&self, writer: &mut BitWriter, value: &u32) {
        ztype::write_varuint32(writer, *value);
    }

    fn to_u64(&self, value: &u32) -> u64 {
        *value as u64
    }
    fn from_u64(&self, value: u64) -> u32 {
        value as u32
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &u32) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &u32,
    ) -> u64 {
        context_node.context.bitsize_of(self, bit_position, element)
    }

    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &u32,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn read_packed(&self, context_node: &mut PackingContextNode, reader: &mut BitReader) -> u32 {
        context_node.context.read(self, reader)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &u32,
    ) {
        context_node.context.write(self, writer, element);
    }
}

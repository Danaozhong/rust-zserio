use crate::ztype;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct VarInt32ArrayTrait {}

impl array_trait::ArrayTrait<i32> for VarInt32ArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, value: &i32) -> u64 {
        ztype::varint32_bitsize(*value) as u64
    }

    fn initialize_offsets(&self, bit_position: u64, value: &i32) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader) -> i32 {
        ztype::read_varint32(reader)
    }

    fn write(&self, writer: &mut BitWriter, value: &i32) {
        ztype::write_varint32(writer, *value);
    }

    fn to_u64(&self, value: &i32) -> u64 {
        *value as u64
    }
    fn from_u64(&self, value: u64) -> i32 {
        value as i32
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &i32) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &i32,
    ) -> u64 {
        context_node.context.bitsize_of(self, bit_position, element)
    }

    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &i32,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn read_packed(&self, context_node: &mut PackingContextNode, reader: &mut BitReader) -> i32 {
        context_node.context.read(self, reader)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &i32,
    ) {
        context_node.context.write(self, writer, element);
    }
}

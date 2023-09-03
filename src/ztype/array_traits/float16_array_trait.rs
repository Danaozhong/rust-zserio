use crate::ztype;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct Float16ArrayTrait {}

impl array_trait::ArrayTrait<f32> for Float16ArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, _: &f32) -> u64 {
        16
    }

    fn initialize_offsets(&self, bit_position: u64, value: &f32) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader, value: &mut f32, _index: usize) {
        *value = ztype::read_float16(reader)
    }

    fn write(&self, writer: &mut BitWriter, value: &f32) {
        ztype::write_float16(writer, *value);
    }

    fn to_u64(&self, _: &f32) -> u64 {
        0 // delta-encoding not supported for float arrays
    }
    fn from_u64(&self, _: u64) -> f32 {
        0.0 // delta-encoding not supported for float arrays
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &f32) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &f32,
    ) -> u64 {
        context_node.context.bitsize_of(self, bit_position, element)
    }

    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &f32,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn read_packed(
        &self,
        context_node: &mut PackingContextNode,
        reader: &mut BitReader,
        value: &mut f32,
        index: usize,
    ) {
        context_node.context.read(self, reader, value, index);
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &f32,
    ) {
        context_node.context.write(self, writer, element);
    }
}

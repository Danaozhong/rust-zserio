use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct BooleanArrayTrait {}

impl array_trait::ArrayTrait<bool> for BooleanArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, _: &bool) -> u64 {
        1
    }

    fn initialize_offsets(&self, bit_position: u64, value: &bool) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader, value: &mut bool, _index: usize) {
        *value = reader.read_bool().expect("failed to read bool");
    }

    fn write(&self, writer: &mut BitWriter, value: &bool) {
        writer.write_bool(*value).expect("failed to write bool");
    }

    fn to_u64(&self, _: &bool) -> u64 {
        0 // delta-encoding not supported for bool arrays
    }
    fn from_u64(&self, _: u64) -> bool {
        false // delta-encoding not supported for bool arrays
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &bool) {
        context_node.context.as_mut().unwrap().init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &bool,
    ) -> u64 {
        context_node
            .context
            .as_mut()
            .unwrap()
            .bitsize_of(self, bit_position, element)
    }

    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &bool,
    ) -> u64 {
        bit_position
            + context_node
                .context
                .as_mut()
                .unwrap()
                .bitsize_of(self, bit_position, element)
    }

    fn read_packed(
        &self,
        context_node: &mut PackingContextNode,
        reader: &mut BitReader,
        value: &mut bool,
        index: usize,
    ) {
        context_node
            .context
            .as_mut()
            .unwrap()
            .read(self, reader, value, index);
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &bool,
    ) {
        context_node
            .context
            .as_mut()
            .unwrap()
            .write(self, writer, element);
    }
}

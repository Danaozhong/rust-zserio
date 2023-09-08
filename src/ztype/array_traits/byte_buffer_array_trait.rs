use crate::ztype;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct ByteBufferArrayTrait {}

impl array_trait::ArrayTrait<ztype::BytesType> for ByteBufferArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, value: &ztype::BytesType) -> u64 {
        ztype::bitsize_of_bytes(value)
    }

    fn initialize_offsets(&self, bit_position: u64, value: &ztype::BytesType) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader, value: &mut ztype::BytesType, _index: usize) {
        *value = ztype::read_bytes_type(reader);
    }

    fn write(&self, writer: &mut BitWriter, value: &ztype::BytesType) {
        ztype::write_bytes_type(writer, value);
    }

    fn to_u64(&self, _: &ztype::BytesType) -> u64 {
        panic!("delta-encoding not supported for extern types");
    }
    fn from_u64(&self, _: u64) -> ztype::BytesType {
        panic!("delta-encoding not supported for extern types");
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &ztype::BytesType) {
        context_node.context.as_mut().unwrap().init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &ztype::BytesType,
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
        element: &ztype::BytesType,
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
        value: &mut ztype::BytesType,
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
        element: &ztype::BytesType,
    ) {
        context_node
            .context
            .as_mut()
            .unwrap()
            .write(self, writer, element);
    }
}

use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::read_string;
use crate::ztype::varsize_bitsize;
use crate::ztype::write_string;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct StringArrayTrait {}

impl array_trait::ArrayTrait<String> for StringArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, value: &String) -> u64 {
        value.len() as u64 + varsize_bitsize(value.len() as u64) as u64
    }

    fn initialize_offsets(&self, bit_position: u64, value: &String) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader) -> String {
        read_string(reader)
    }

    fn write(&self, writer: &mut BitWriter, value: &String) {
        write_string(writer, value);
    }

    fn to_u64(&self, _value: &String) -> u64 {
        return 0; // not supported for strings
    }
    fn from_u64(&self, _value: u64) -> String {
        return "".into(); // not supported for strings
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &String) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &String,
    ) -> u64 {
        context_node.context.bitsize_of(self, bit_position, element)
    }
    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &String,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &String,
    ) {
        context_node.context.write(self, writer, element);
    }
}

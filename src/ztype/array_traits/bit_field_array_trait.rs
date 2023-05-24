use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::read_signed_bits;
use crate::ztype::write_signed_bits;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct BitFieldArrayTrait {
    pub num_bits: u8,
}

impl array_trait::ArrayTrait<i64> for BitFieldArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        true
    }

    fn bitsize_of(&self, _bit_position: u64, _value: &i64) -> u64 {
        self.num_bits as u64
    }

    fn initialize_offsets(&self, bit_position: u64, _: &i64) -> u64 {
        bit_position + self.bitsize_of(bit_position, &0) as u64
    }

    fn read(&self, reader: &mut BitReader) -> i64 {
        read_signed_bits(reader, self.bitsize_of(0, &0) as u8)
    }

    fn write(&self, writer: &mut BitWriter, value: &i64) {
        write_signed_bits(writer, *value, self.bitsize_of(0, &0) as u8);
    }

    fn to_u64(&self, value: &i64) -> u64 {
        *value as u64
    }
    fn from_u64(&self, value: u64) -> i64 {
        value as i64
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &i64) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &i64,
    ) -> u64 {
        context_node.context.bitsize_of(self, bit_position, element)
    }
    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &i64,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &i64,
    ) {
        context_node.context.write(self, writer, element);
    }
}

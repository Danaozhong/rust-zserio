use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::read_unsigned_bits;
use crate::ztype::write_unsigned_bits;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct UnsignedBitFieldArrayTrait {
    pub num_bits: u8,
}

impl array_trait::ArrayTrait<u64> for UnsignedBitFieldArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        true
    }

    fn bitsize_of(&self, _bit_position: u64, _value: &u64) -> u64 {
        self.num_bits as u64
    }

    fn initialize_offsets(&self, bit_position: u64, _: &u64) -> u64 {
        bit_position + self.bitsize_of(0, &0)
    }

    fn read(&self, reader: &mut BitReader) -> u64 {
        read_unsigned_bits(reader, self.bitsize_of(0, &0) as u8)
    }

    fn write(&self, writer: &mut BitWriter, value: &u64) {
        write_unsigned_bits(writer, *value, self.bitsize_of(0, &0) as u8);
    }

    fn to_u64(&self, value: &u64) -> u64 {
        *value
    }
    fn from_u64(&self, value: u64) -> u64 {
        value
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &u64) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &u64,
    ) -> u64 {
        context_node.context.bitsize_of(self, bit_position, element)
    }

    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &u64,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn read_packed(&self, context_node: &mut PackingContextNode, reader: &mut BitReader) -> u64 {
        context_node.context.read(self, reader)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &u64,
    ) {
        context_node.context.write(self, writer, element);
    }
}

use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;



use crate::ztype::ZserioPackableOject;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;


pub struct ObjectArrayTrait {}

impl<T> array_trait::ArrayTrait<T> for ObjectArrayTrait
where
    T: ZserioPackableOject,
{
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, bit_position: u64, value: &T) -> u64 {
        value.zserio_bitsize(bit_position)
    }

    fn initialize_offsets(&self, bit_position: u64, value: &T) -> u64 {
        bit_position + self.bitsize_of(bit_position, value)
    }

    fn read(&self, reader: &mut BitReader) -> T {
        let mut value = T::new();
        value.unmarshal_zserio(reader);
        value
    }

    fn write(&self, writer: &mut BitWriter, value: &T) {
        value.marshal_zserio(writer)
    }

    fn to_u64(&self, _value: &T) -> u64 {
        panic!("array trait does not support delta compression");
    }
    fn from_u64(&self, _value: u64) -> T {
        panic!("array trait does not support delta compression");
    }

    fn init_context(&self, _context_node: &mut PackingContextNode, _element: &T) {
        // TODO
    }

    fn bitsize_of_packed(
        &self,
        _context_node: &mut PackingContextNode,
        _bit_position: u64,
        _element: &T,
    ) -> u64 {
        // TODO
        0
    }
    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &T,
    ) -> u64 {
        bit_position + context_node.context.bitsize_of(self, bit_position, element)
    }

    fn write_packed(
        &self,
        _context_node: &mut PackingContextNode,
        _writer: &mut BitWriter,
        _element: &T,
    ) {
        // TODO
    }
}
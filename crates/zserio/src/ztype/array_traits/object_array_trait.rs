use crate::error::Result;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::ZserioPackableObject;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct ObjectArrayTrait {}

impl<T> array_trait::ArrayTrait<T> for ObjectArrayTrait
where
    T: ZserioPackableObject,
{
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, bit_position: u64, value: &T) -> Result<u64> {
        value.zserio_bitsize(bit_position)
    }

    fn initialize_offsets(&self, bit_position: u64, value: &T) -> Result<u64> {
        Ok(bit_position + self.bitsize_of(bit_position, value)?)
    }

    fn read(&self, reader: &mut BitReader, value: &mut T, _index: usize) -> Result<()> {
        value.zserio_read(reader)
    }

    fn write(&self, writer: &mut BitWriter, value: &T) -> Result<()> {
        value.zserio_write(writer)
    }

    fn to_u64(&self, _value: &T) -> u64 {
        panic!("array trait does not support delta compression");
    }
    fn from_u64(&self, _value: u64) -> T {
        panic!("array trait does not support delta compression");
    }

    fn create_context(&self) -> PackingContextNode {
        let mut packing_context_node = PackingContextNode::new();
        T::zserio_create_packing_context(&mut packing_context_node);
        packing_context_node
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &T) -> Result<()> {
        element.zserio_init_packing_context(context_node)
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &T,
    ) -> Result<u64> {
        element.zserio_bitsize_packed(context_node, bit_position)
    }
    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &T,
    ) -> Result<u64> {
        Ok(bit_position
            + context_node
                .context
                .as_mut()
                .unwrap()
                .bitsize_of(self, bit_position, element)?)
    }

    fn read_packed(
        &self,
        context_node: &mut PackingContextNode,
        reader: &mut BitReader,
        value: &mut T,
        _index: usize,
    ) -> Result<()> {
        value.zserio_read_packed(context_node, reader)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &T,
    ) -> Result<()> {
        element.zserio_write_packed(context_node, writer)
    }
}

use crate::error::Result;
use crate::ztype;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct BitBufferArrayTrait {}

impl array_trait::ArrayTrait<ztype::ExternType> for BitBufferArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        false
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, value: &ztype::ExternType) -> Result<u64> {
        Ok(ztype::bitsize_of_extern(value))
    }

    fn initialize_offsets(&self, bit_position: u64, value: &ztype::ExternType) -> Result<u64> {
        Ok(bit_position + self.bitsize_of(bit_position, value)?)
    }

    fn read(
        &self,
        reader: &mut BitReader,
        value: &mut ztype::ExternType,
        _index: usize,
    ) -> Result<()> {
        *value = ztype::read_extern_type(reader)?;
        Ok(())
    }

    fn write(&self, writer: &mut BitWriter, value: &ztype::ExternType) -> Result<()> {
        ztype::write_extern_type(writer, value)
    }

    fn to_u64(&self, _: &ztype::ExternType) -> u64 {
        panic!("delta-encoding not supported for extern types");
    }
    fn from_u64(&self, _: u64) -> ztype::ExternType {
        panic!("delta-encoding not supported for extern types");
    }

    fn init_context(
        &self,
        context_node: &mut PackingContextNode,
        element: &ztype::ExternType,
    ) -> Result<()> {
        context_node.context.as_mut().unwrap().init(self, element)
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &ztype::ExternType,
    ) -> Result<u64> {
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
        element: &ztype::ExternType,
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
        value: &mut ztype::ExternType,
        index: usize,
    ) -> Result<()> {
        context_node
            .context
            .as_mut()
            .unwrap()
            .read(self, reader, value, index)
    }

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &ztype::ExternType,
    ) -> Result<()> {
        context_node
            .context
            .as_mut()
            .unwrap()
            .write(self, writer, element)
    }
}

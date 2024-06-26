#![allow(clippy::unnecessary_cast)] // The duplicate_item macro will cause unnecessary cast warnings that are hard to fix
use crate::error::Result;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::read_unsigned_bits;
use crate::ztype::write_unsigned_bits;
use bitreader::BitReader;
use duplicate::duplicate_item;
use rust_bitwriter::BitWriter;

pub struct UnsignedBitFieldArrayTrait {
    pub num_bits: u8,
}

#[duplicate_item(name; [u64]; [u32]; [u16]; [u8])]
impl array_trait::ArrayTrait<name> for UnsignedBitFieldArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        true
    }

    fn bitsize_of(&self, _bit_position: u64, _value: &name) -> Result<u64> {
        Ok(self.num_bits as u64)
    }

    fn initialize_offsets(&self, bit_position: u64, _: &name) -> Result<u64> {
        Ok(bit_position + self.bitsize_of(bit_position, &0u8)?)
    }

    fn read(&self, reader: &mut BitReader, value: &mut name, _index: usize) -> Result<()> {
        *value = read_unsigned_bits(reader, self.bitsize_of(0, &0u8)? as u8)? as name;
        Ok(())
    }

    fn write(&self, writer: &mut BitWriter, value: &name) -> Result<()> {
        write_unsigned_bits(writer, *value as u64, self.bitsize_of(0, &0u8)? as u8)
    }

    fn to_u64(&self, value: &name) -> u64 {
        *value as u64
    }
    fn from_u64(&self, value: u64) -> name {
        value as name
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &name) -> Result<()> {
        context_node.context.as_mut().unwrap().init(self, element)
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &name,
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
        element: &name,
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
        value: &mut name,
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
        element: &name,
    ) -> Result<()> {
        context_node
            .context
            .as_mut()
            .unwrap()
            .write(self, writer, element)
    }
}

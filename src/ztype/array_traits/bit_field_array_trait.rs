#![allow(clippy::unnecessary_cast)] // The duplicate_item macro will cause unnecessary cast warnings that are hard to fix

use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::read_signed_bits;
use crate::ztype::write_signed_bits;
use bitreader::BitReader;
use duplicate::duplicate_item;
use rust_bitwriter::BitWriter;

pub struct BitFieldArrayTrait {
    pub num_bits: u8,
}

#[duplicate_item(name; [i64]; [i32]; [i16]; [i8])]
impl array_trait::ArrayTrait<name> for BitFieldArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        true
    }

    fn bitsize_of(&self, _bit_position: u64, _value: &name) -> u64 {
        self.num_bits as u64
    }

    fn initialize_offsets(&self, bit_position: u64, _: &name) -> u64 {
        bit_position + self.bitsize_of(bit_position, &0)
    }

    fn read(&self, reader: &mut BitReader, value: &mut name, _index: usize) {
        *value = read_signed_bits(reader, self.bitsize_of(0, &0) as u8) as name;
    }

    fn write(&self, writer: &mut BitWriter, value: &name) {
        write_signed_bits(writer, *value as i64, self.bitsize_of(0, &0) as u8);
    }

    fn to_u64(&self, value: &name) -> u64 {
        *value as u64
    }
    fn from_u64(&self, value: u64) -> name {
        value as name
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &name) {
        context_node.context.as_mut().unwrap().init(self, element);
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &name,
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
        element: &name,
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
        value: &mut name,
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
        element: &name,
    ) {
        context_node
            .context
            .as_mut()
            .unwrap()
            .write(self, writer, element);
    }
}

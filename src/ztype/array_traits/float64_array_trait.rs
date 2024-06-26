use crate::error::Result;
use crate::ztype;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub struct Float64ArrayTrait {}

impl array_trait::ArrayTrait<f64> for Float64ArrayTrait {
    fn is_bitsizeof_constant(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        false
    }

    fn bitsize_of(&self, _bit_position: u64, _: &f64) -> Result<u64> {
        Ok(64)
    }

    fn initialize_offsets(&self, bit_position: u64, value: &f64) -> Result<u64> {
        Ok(bit_position + self.bitsize_of(bit_position, value)?)
    }

    fn read(&self, reader: &mut BitReader, value: &mut f64, _index: usize) -> Result<()> {
        *value = ztype::read_float64(reader)?;
        Ok(())
    }

    fn write(&self, writer: &mut BitWriter, value: &f64) -> Result<()> {
        ztype::write_float64(writer, *value)
    }

    fn to_u64(&self, _: &f64) -> u64 {
        0 // delta-encoding not supported for float arrays
    }
    fn from_u64(&self, _: u64) -> f64 {
        0.0 // delta-encoding not supported for float arrays
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &f64) -> Result<()> {
        context_node.context.as_mut().unwrap().init(self, element)
    }

    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &f64,
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
        element: &f64,
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
        value: &mut f64,
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
        element: &f64,
    ) -> Result<()> {
        context_node
            .context
            .as_mut()
            .unwrap()
            .write(self, writer, element)
    }
}

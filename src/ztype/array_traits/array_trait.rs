use crate::error::Result;
use crate::ztype::array_traits::delta_context::DeltaContext;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;
pub trait ArrayTrait<T> {
    fn is_bitsizeof_constant(&self) -> bool;
    fn needs_bitsizeof_position(&self) -> bool;
    fn bitsize_of(&self, bit_position: u64, value: &T) -> Result<u64>;
    fn initialize_offsets(&self, bit_position: u64, value: &T) -> Result<u64>;
    fn read(&self, reader: &mut BitReader, value: &mut T, index: usize) -> Result<()>;
    fn write(&self, writer: &mut BitWriter, value: &T) -> Result<()>;
    fn to_u64(&self, value: &T) -> u64;
    #[allow(clippy::wrong_self_convention)]
    fn from_u64(&self, value: u64) -> T;

    // all functions below are for using packed contexts. They provide a default implementation
    // for array traits using delta contexts.
    fn create_context(&self) -> PackingContextNode {
        PackingContextNode {
            children: vec![],
            context: Some(DeltaContext::new()),
        }
    }

    fn init_context(&self, context_node: &mut PackingContextNode, element: &T) -> Result<()>;
    fn bitsize_of_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &T,
    ) -> Result<u64>;
    fn initialize_offsets_packed(
        &self,
        context_node: &mut PackingContextNode,
        bit_position: u64,
        element: &T,
    ) -> Result<u64>;

    fn read_packed(
        &self,
        context_node: &mut PackingContextNode,
        reader: &mut BitReader,
        value: &mut T,
        index: usize,
    ) -> Result<()>;

    fn write_packed(
        &self,
        context_node: &mut PackingContextNode,
        writer: &mut BitWriter,
        element: &T,
    ) -> Result<()>;
}

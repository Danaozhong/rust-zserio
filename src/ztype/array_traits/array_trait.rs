use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
pub trait ArrayTrait<T> {

    fn is_bitsizeof_constant(&self) -> bool;
    fn needs_bitsizeof_position(&self) -> bool;
    fn bitsize_of(&self) -> u8;
    fn initialize_offsets(&self, bit_position: u64) -> u64;
    fn read(&self, reader: &mut BitReader) -> T;
	fn write(&self, writer: &mut BitWriter, value: &T);
	fn to_u64(&self, value: &T) -> u64;
	fn from_u64(&self, value: u64) -> T;

	// all functions below are for using packed contexts. They provide a default implementation
	// for array traits using delta contexts.
	fn create_context(&self) -> PackingContextNode {
		PackingContextNode::new()
	}

	fn init_context(&self, context_node: &mut PackingContextNode, element: &T);
    fn bitsize_of_packed(&self, context_node: &mut PackingContextNode, bit_position: u64, element: &T) -> u8;
    fn initialize_offsets_packed(&self, bit_position: u64, element: &T) -> u64;
	fn write_packed(&self, context_node: &mut PackingContextNode, writer: &mut BitWriter, element: &T);
}
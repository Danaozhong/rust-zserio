use rust_bitwriter::BitWriter;

use crate::ztype::bits_encode;
use crate::ztype::varuint_encode::write_varsize;
use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;

pub struct Array<T> {
    pub array_trait: Box<dyn ArrayTrait<T>>,
    pub is_packed: bool,
    pub fixed_size: Option<u64>,
	packing_context_node: Option<PackingContextNode>,
	is_aligned: bool,
}


impl<T> Array<T> {

	fn create_packing_context_node_if_not_exists(&mut self) {
		if self.packing_context_node.is_none() {
			self.packing_context_node = PackingContextNode::new().into();
		}
	}

	pub fn write_array(
		&self,
		writer: &mut BitWriter,
		array: &mut Array<T>,
		data: &Vec<T>,
	) {
		if let Some(expected_array_len) = array.fixed_size  {
			// for fixed-size arrays, the provided length must match
			assert_eq!(expected_array_len, data.len() as u64);
		} else {
			// for auto arrays, write the length of the array
			write_varsize(writer, data.len() as u64);
		}

		if data.len() == 0 {
			return;
		}
		if array.is_packed {
			array.create_packing_context_node_if_not_exists();

			for element in data {
				array.array_trait.init_context(array.packing_context_node.as_mut().unwrap(), &element);
			}
		}
		for (index, element) in data.iter().enumerate() {
			if array.is_aligned {
				writer.align(1);
			}
			if array.is_packed {
				array.array_trait.write_packed(array.packing_context_node.as_mut().unwrap(), writer, element);
			} else {
				array.array_trait.write(writer, element);
			}
		}
	}
}

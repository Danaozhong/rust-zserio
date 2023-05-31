use bitreader::BitReader;
use rust_bitwriter::BitWriter;

use crate::ztype::read_varsize;

use crate::ztype::{align_to, varsize_bitsize};

use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::varuint_encode::write_varsize;

pub struct Array<T> {
    pub array_trait: Box<dyn ArrayTrait<T>>,
    pub is_packed: bool,
    pub fixed_size: Option<u64>,
    pub is_aligned: bool,
    pub packing_context_node: Option<PackingContextNode>,
}

impl<T> Array<T> {
    fn create_packing_context_node_if_not_exists(&mut self) {
        if self.packing_context_node.is_none() {
            self.packing_context_node = PackingContextNode::new().into();
        }
    }

    pub fn zserio_write(&mut self, writer: &mut BitWriter, data: &Vec<T>) {
        if let Some(expected_array_len) = self.fixed_size {
            // for fixed-size arrays, the provided length must match
            assert_eq!(expected_array_len, data.len() as u64);
        } else {
            // for auto arrays, write the length of the array
            write_varsize(writer, data.len() as u64);
        }

        if data.is_empty() {
            return;
        }
        if self.is_packed {
            self.create_packing_context_node_if_not_exists();

            for element in data {
                self.array_trait
                    .init_context(self.packing_context_node.as_mut().unwrap(), element);
            }
        }
        for (_index, element) in data.iter().enumerate() {
            if self.is_aligned {
                let _ = writer.align(1);
            }
            if self.is_packed {
                self.array_trait.write_packed(
                    self.packing_context_node.as_mut().unwrap(),
                    writer,
                    element,
                );
            } else {
                self.array_trait.write(writer, element);
            }
        }
    }

    pub fn zserio_read(&mut self, reader: &mut BitReader) -> Vec<T> {
        let array_length;
        if let Some(expected_array_len) = self.fixed_size {
            array_length = expected_array_len;
        } else {
            array_length = read_varsize(reader);
        }

        let mut data = Vec::<T>::with_capacity(array_length as usize);
        if array_length > 0 {
            if self.is_packed {
                self.create_packing_context_node_if_not_exists();
            }
            for _index in 0..array_length {
                if self.is_aligned {
                    reader.align(8).expect("failed to align reader");
                }
                if self.is_packed {
                    data.push(
                        self.array_trait
                            .read_packed(self.packing_context_node.as_mut().unwrap(), reader),
                    );
                } else {
                    data.push(self.array_trait.read(reader));
                }
            }
        }
        data
    }

    pub fn zserio_bitsize(&mut self, data: &Vec<T>, bit_position: u64) -> u64 {
        let mut end_position = bit_position;
        if self.fixed_size.is_none() {
            end_position += varsize_bitsize(data.len() as u64) as u64;
        }
        if !data.is_empty() {
            if self.array_trait.is_bitsizeof_constant() {
                // Since the bitsize is anyway constant, just pass the first element
                let element_size = self.array_trait.bitsize_of(end_position, &data[0]);
                if self.is_aligned {
                    // make sure the first element is aligned
                    end_position = align_to(8, end_position);

                    // count all array elements alignment positions
                    end_position += (data.len() - 1) as u64 * align_to(8, element_size);
                }

                // count the actual payload
                end_position += data.len() as u64 * element_size;
            } else {
                // the bitsize of each array element may differ, as such, each element need to be
                // added individually.
                for element in data {
                    if self.is_aligned {
                        end_position = align_to(8, end_position);
                    }
                    end_position += self.array_trait.bitsize_of(end_position, element);
                }
            }
        }
        end_position - bit_position
    }

    pub fn zserio_bitsize_packed(&mut self, data: &Vec<T>, bit_position: u64) -> u64 {
        let mut end_position = bit_position;
        if self.fixed_size.is_none() {
            end_position += varsize_bitsize(data.len() as u64) as u64;
        }
        if !data.is_empty() {
            self.create_packing_context_node_if_not_exists();

            for element in data {
                self.array_trait
                    .init_context(self.packing_context_node.as_mut().unwrap(), element);
            }

            for element in data {
                if self.is_aligned {
                    end_position = align_to(8, end_position);
                }
                end_position += self.array_trait.bitsize_of_packed(
                    self.packing_context_node.as_mut().unwrap(),
                    end_position,
                    element,
                );
            }
        }
        end_position - bit_position
    }
}

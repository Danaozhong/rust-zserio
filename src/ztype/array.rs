use bitreader::BitReader;
use rust_bitwriter::BitWriter;

use crate::ztype::read_varsize;

use crate::ztype::{align_to, varsize_bitsize};

use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::varuint_encode::write_varsize;

pub struct Array<T> {
    pub array_trait: Box<dyn ArrayTrait<T>>,
    pub is_packed: bool,
    pub fixed_size: Option<usize>,
    pub is_aligned: bool,
}

impl<T> Array<T> {
    pub fn zserio_write(&mut self, writer: &mut BitWriter, data: &Vec<T>) {
        if let Some(expected_array_len) = self.fixed_size {
            // for fixed-size arrays, the provided length must match
            assert_eq!(expected_array_len, data.len());
        } else {
            // for auto arrays, write the length of the array
            write_varsize(writer, data.len() as u32);
        }

        if data.is_empty() {
            return;
        }
        if self.is_packed {
            // Create the packing context, and all child-contexts
            let mut packing_context_node = self.array_trait.create_context();

            // Initialize the contexts, to identify the delta packing sizes
            for element in data {
                self.array_trait
                    .init_context(&mut packing_context_node, element);
            }

            // Actually write the data.
            for element in data.iter() {
                if self.is_aligned {
                    let _ = writer.align(1);
                }
                self.array_trait
                    .write_packed(&mut packing_context_node, writer, element);
            }
        } else {
            for element in data.iter() {
                if self.is_aligned {
                    let _ = writer.align(1);
                }
                self.array_trait.write(writer, element);
            }
        }
    }

    pub fn zserio_read_array_length(&mut self, reader: &mut BitReader) -> usize {
        if let Some(expected_array_len) = self.fixed_size {
            expected_array_len
        } else {
            read_varsize(reader).unwrap() as usize
        }
    }

    pub fn zserio_read(&mut self, reader: &mut BitReader, data: &mut [T]) {
        if !data.is_empty() {
            if self.is_packed {
                // Create the packing context, and all child-contexts
                let mut packing_context_node = self.array_trait.create_context();
                for (index, data_item) in data.iter_mut().enumerate() {
                    if self.is_aligned {
                        reader.align(1).expect("failed to align reader");
                    }
                    self.array_trait.read_packed(
                        &mut packing_context_node,
                        reader,
                        data_item,
                        index,
                    );
                }
            } else {
                for (index, data_item) in data.iter_mut().enumerate() {
                    if self.is_aligned {
                        reader.align(1).expect("failed to align reader");
                    }
                    self.array_trait.read(reader, data_item, index);
                }
            }
        }
    }

    pub fn zserio_bitsize(&mut self, data: &Vec<T>, bit_position: u64) -> u64 {
        let mut end_position = bit_position;
        if self.fixed_size.is_none() {
            end_position += varsize_bitsize(data.len() as u32) as u64;
        }
        if !data.is_empty() {
            if self.is_packed {
                // Packing is used
                // Create the packing context, and all child-contexts
                let mut packing_context_node = self.array_trait.create_context();
                // Initialize the contexts, to identify the delta packing sizes
                for element in data {
                    self.array_trait
                        .init_context(&mut packing_context_node, element);
                }

                for data_item in data {
                    if self.is_aligned {
                        end_position = align_to(8, end_position);
                    }
                    end_position += self.array_trait.bitsize_of_packed(
                        &mut packing_context_node,
                        end_position,
                        data_item,
                    );
                }
            } else {
                // Array is not packed
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
        }
        end_position - bit_position
    }

    pub fn zserio_bitsize_packed(&mut self, data: &Vec<T>, bit_position: u64) -> u64 {
        let mut end_position = bit_position;
        if self.fixed_size.is_none() {
            end_position += varsize_bitsize(data.len() as u32) as u64;
        }
        if !data.is_empty() {
            let mut packing_context_node = self.array_trait.create_context();

            for element in data {
                self.array_trait
                    .init_context(&mut packing_context_node, element);
            }

            for element in data {
                if self.is_aligned {
                    end_position = align_to(8, end_position);
                }
                end_position += self.array_trait.bitsize_of_packed(
                    &mut packing_context_node,
                    end_position,
                    element,
                );
            }
        }
        end_position - bit_position
    }
}

use crate::error::Result;
use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::read_varsize;
use crate::ztype::varsize_bitsize;
use crate::ztype::varuint_encode::write_varsize;
use bitreader::BitReader;
use num::traits::Unsigned;
use rust_bitwriter::BitWriter;

use crate::ztype::alignment::{align_bitsize, align_reader, align_writer};

pub struct Array<T> {
    pub array_trait: Box<dyn ArrayTrait<T>>,
    pub is_packed: bool,
    pub fixed_size: Option<usize>,
}

pub trait OffsetTrait: Unsigned + Copy + std::convert::Into<u64> {}

impl OffsetTrait for u8 {}
impl OffsetTrait for u16 {}
impl OffsetTrait for u32 {}
impl OffsetTrait for u64 {}

impl<T> Array<T> {
    pub fn zserio_write<U: OffsetTrait>(
        &mut self,
        writer: &mut BitWriter,
        data: &Vec<T>,
        index_offsets: Option<&Vec<U>>,
    ) -> Result<()> {
        if let Some(expected_array_len) = self.fixed_size {
            // for fixed-size arrays, the provided length must match
            assert_eq!(expected_array_len, data.len());
        } else {
            // for auto arrays, write the length of the array
            write_varsize(writer, data.len() as u32)?;
        }

        if data.is_empty() {
            return Ok(());
        }
        if self.is_packed {
            // Create the packing context, and all child-contexts
            let mut packing_context_node = self.array_trait.create_context();

            // Initialize the contexts, to identify the delta packing sizes
            for element in data {
                self.array_trait
                    .init_context(&mut packing_context_node, element)?;
            }

            // Actually write the data.
            for (index, element) in data.iter().enumerate() {
                align_array_element_writer(writer, index, index_offsets)?;
                self.array_trait
                    .write_packed(&mut packing_context_node, writer, element)?;
            }
        } else {
            for (index, element) in data.iter().enumerate() {
                align_array_element_writer(writer, index, index_offsets)?;
                self.array_trait.write(writer, element)?;
            }
        }
        Ok(())
    }

    pub fn zserio_read_array_length(&mut self, reader: &mut BitReader) -> Result<usize> {
        match self.fixed_size {
            Some(expected_array_len) => Ok(expected_array_len),
            None => Ok(read_varsize(reader)? as usize),
        }
    }

    pub fn zserio_read<U: OffsetTrait>(
        &mut self,
        reader: &mut BitReader,
        data: &mut [T],
        index_offsets: Option<&Vec<U>>,
    ) -> Result<()> {
        if !data.is_empty() {
            if self.is_packed {
                // Create the packing context, and all child-contexts
                let mut packing_context_node = self.array_trait.create_context();
                for (index, data_item) in data.iter_mut().enumerate() {
                    align_array_element_reader(reader, index, index_offsets)?;
                    self.array_trait.read_packed(
                        &mut packing_context_node,
                        reader,
                        data_item,
                        index,
                    )?;
                }
            } else {
                for (index, data_item) in data.iter_mut().enumerate() {
                    align_array_element_reader(reader, index, index_offsets)?;
                    self.array_trait.read(reader, data_item, index)?;
                }
            }
        }
        Ok(())
    }

    pub fn zserio_bitsize<U: OffsetTrait>(
        &mut self,
        data: &Vec<T>,
        index_offsets: Option<&Vec<U>>,
        bit_position: u64,
    ) -> Result<u64> {
        let mut end_position = bit_position;
        if self.fixed_size.is_none() {
            end_position += varsize_bitsize(data.len() as u32)? as u64;
        }
        if !data.is_empty() {
            if self.is_packed {
                // Packing is used
                // Create the packing context, and all child-contexts
                let mut packing_context_node = self.array_trait.create_context();
                // Initialize the contexts, to identify the delta packing sizes
                for element in data {
                    self.array_trait
                        .init_context(&mut packing_context_node, element)?;
                }

                for (index, element) in data.iter().enumerate() {
                    end_position = align_array_element_bitsize(end_position, index, index_offsets);
                    end_position += self.array_trait.bitsize_of_packed(
                        &mut packing_context_node,
                        end_position,
                        element,
                    )?;
                }
            } else {
                // Array is not packed
                if self.array_trait.is_bitsizeof_constant() && index_offsets.is_none() {
                    // Since the bitsize is anyway constant, just pass the first element
                    let element_size = self.array_trait.bitsize_of(end_position, &data[0])?;

                    // count the actual payload
                    end_position += data.len() as u64 * element_size;
                } else {
                    // the bitsize of each array element may differ, as such, each element need to be
                    // added individually.
                    for (index, element) in data.iter().enumerate() {
                        end_position =
                            align_array_element_bitsize(end_position, index, index_offsets);
                        end_position += self.array_trait.bitsize_of(end_position, element)?;
                    }
                }
            }
        }
        Ok(end_position - bit_position)
    }

    pub fn zserio_bitsize_packed<U: OffsetTrait>(
        &mut self,
        data: &Vec<T>,
        index_offsets: Option<&Vec<U>>,
        bit_position: u64,
    ) -> Result<u64> {
        let mut end_position = bit_position;
        if self.fixed_size.is_none() {
            end_position += varsize_bitsize(data.len() as u32)? as u64;
        }
        if !data.is_empty() {
            let mut packing_context_node = self.array_trait.create_context();

            for element in data {
                self.array_trait
                    .init_context(&mut packing_context_node, element)?;
            }

            for (index, element) in data.iter().enumerate() {
                end_position = align_array_element_bitsize(end_position, index, index_offsets);
                end_position += self.array_trait.bitsize_of_packed(
                    &mut packing_context_node,
                    end_position,
                    element,
                )?;
            }
        }
        Ok(end_position - bit_position)
    }
}

fn align_array_element_writer<T: OffsetTrait>(
    writer: &mut BitWriter,
    _index: usize,
    index_offsets: Option<&Vec<T>>,
) -> Result<()> {
    // A small helper function to byte-align each array element, if desired, during writing.
    if index_offsets.is_some() {
        align_writer(writer, 8)?;
    }
    Ok(())
}
fn align_array_element_reader<T: OffsetTrait>(
    reader: &mut BitReader,
    _index: usize,
    index_offsets: Option<&Vec<T>>,
) -> Result<()> {
    // A small helper function to byte-align each array element, if desired, during reading.
    if index_offsets.is_some() {
        align_reader(reader, 8)?;
    }
    Ok(())
}

fn align_array_element_bitsize<T: OffsetTrait>(
    bit_position: u64,
    _index: usize,
    index_offsets: Option<&Vec<T>>,
) -> u64 {
    // A small helper function to byte-align each array element, if desired, during bitsize calculation.
    let mut end_position = bit_position;
    if index_offsets.is_some() {
        end_position += align_bitsize(end_position, 8);
    }
    end_position
}

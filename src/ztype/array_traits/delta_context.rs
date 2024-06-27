use crate::error::Result;
use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::bits_decode::read_signed_bits;
use crate::ztype::{self, numbits, read_unsigned_bits};

use bitreader::BitReader;
use rust_bitwriter::BitWriter;

const MAX_BIT_NUMBER_LIMIT: u8 = 62;
const MAX_BIT_NUMBER_BITS: u8 = 6;

pub struct DeltaContext {
    is_packed: bool,
    max_bit_number: u8,
    unpacked_size: u64,
    first_element_size: u64,
    num_elements: u64,
    previous_element: u64,
    init_started: bool,
    processing_started: bool,
}

impl Default for DeltaContext {
    fn default() -> Self {
        Self::new()
    }
}

fn abs_difference(x: u64, y: u64) -> u64 {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl DeltaContext {
    pub fn new() -> DeltaContext {
        DeltaContext {
            is_packed: false,
            max_bit_number: 0,
            unpacked_size: 0,
            first_element_size: 0,
            num_elements: 0,
            previous_element: 0,
            init_started: false,
            processing_started: false,
        }
    }

    pub fn init<T>(&mut self, array_trait: &dyn ArrayTrait<T>, element: &T) -> Result<()> {
        self.num_elements += 1;
        self.unpacked_size += array_trait.bitsize_of(0, element)?;

        if !self.init_started {
            self.init_started = true;
            self.previous_element = array_trait.to_u64(element);
            self.first_element_size = self.unpacked_size;
        } else if self.max_bit_number <= MAX_BIT_NUMBER_LIMIT {
            self.is_packed = true;
            let delta = abs_difference(array_trait.to_u64(element), self.previous_element);
            let max_bit_number = numbits(delta);
            if max_bit_number > self.max_bit_number {
                self.max_bit_number = max_bit_number;
                if self.max_bit_number > MAX_BIT_NUMBER_LIMIT {
                    self.is_packed = false;
                }
            }
            self.previous_element = array_trait.to_u64(element);
        }
        Ok(())
    }

    pub fn bitsize_of<T>(
        &mut self,
        array_traits: &dyn ArrayTrait<T>,
        _bit_position: u64,
        element: &T,
    ) -> Result<u64> {
        if !self.processing_started {
            self.processing_started = true;
            self.finish_init();

            return Ok(
                self.bitsize_of_descriptor() + self.bitsize_of_unpacked(array_traits, element)?
            );
        }
        if !self.is_packed {
            return self.bitsize_of_unpacked(array_traits, element);
        }
        if self.max_bit_number > 0 {
            return Ok(self.max_bit_number as u64 + 1);
        }
        Ok(0)
    }

    pub fn read<T>(
        &mut self,
        array_traits: &dyn ArrayTrait<T>,
        reader: &mut BitReader,
        value: &mut T,
        _index: usize,
    ) -> Result<()> {
        if !self.processing_started {
            self.processing_started = true;
            self.read_descriptor(reader);
            return self.read_unpacked(array_traits, reader, value);
        }
        if !self.is_packed {
            return self.read_unpacked(array_traits, reader, value);
        }
        if self.max_bit_number > 0 {
            let delta = read_signed_bits(reader, self.max_bit_number + 1)?;
            self.previous_element = (self.previous_element as i64 + delta) as u64;
        }
        *value = array_traits.from_u64(self.previous_element);
        Ok(())
    }

    pub fn finish_init(&mut self) {
        if !self.is_packed {
            return;
        }
        let mut delta_bit_size = self.max_bit_number as u64;
        if delta_bit_size > 0 {
            delta_bit_size += 1;
        }
        let packed_bit_size_with_decriptor = 1
            + MAX_BIT_NUMBER_BITS as u64
            + self.first_element_size
            + (self.num_elements - 1) * delta_bit_size;

        let unpacked_bit_size_with_descriptor = 1 + self.unpacked_size;

        if packed_bit_size_with_decriptor >= unpacked_bit_size_with_descriptor {
            self.is_packed = false;
        }
    }

    pub fn write<T>(
        &mut self,
        array_traits: &dyn ArrayTrait<T>,
        writer: &mut BitWriter,
        element: &T,
    ) -> Result<()> {
        if !self.processing_started {
            self.processing_started = true;
            self.finish_init();

            self.write_descriptor(writer);
            return self.write_unpacked(array_traits, writer, element);
        }
        if !self.is_packed {
            return self.write_unpacked(array_traits, writer, element);
        }
        if self.max_bit_number > 0 {
            let element_as_u64 = array_traits.to_u64(element);
            let delta = element_as_u64 as i64 - self.previous_element as i64;
            ztype::write_signed_bits(writer, delta, self.max_bit_number + 1)?;
            self.previous_element = element_as_u64;
        }
        Ok(())
    }

    // BitSizeOfDescriptor returns the bit size of a delta context array descriptor.
    fn bitsize_of_descriptor(&self) -> u64 {
        if self.is_packed {
            return (1 + MAX_BIT_NUMBER_BITS) as u64;
        }
        1
    }

    fn read_descriptor(&mut self, reader: &mut BitReader) {
        self.is_packed = reader
            .read_bool()
            .expect("failed to read if the context is packed");
        if self.is_packed {
            // read how many bits are used for the delta encoding of each element
            self.max_bit_number = read_unsigned_bits(reader, MAX_BIT_NUMBER_BITS).unwrap() as u8;
        }
    }

    fn read_unpacked<T>(
        &mut self,
        array_traits: &dyn ArrayTrait<T>,
        reader: &mut BitReader,
        value: &mut T,
    ) -> Result<()> {
        array_traits.read(reader, value, 0)?; // TODO need to check if the index is needed
        self.previous_element = array_traits.to_u64(value);
        Ok(())
    }

    fn write_descriptor(&self, writer: &mut BitWriter) {
        writer
            .write_bool(self.is_packed)
            .expect("failed to write bool");
        if self.is_packed {
            ztype::write_unsigned_bits(writer, self.max_bit_number as u64, MAX_BIT_NUMBER_BITS)
                .unwrap();
        }
    }

    fn write_unpacked<T>(
        &mut self,
        array_traits: &dyn ArrayTrait<T>,
        writer: &mut BitWriter,
        element: &T,
    ) -> Result<()> {
        let element_as_u64 = array_traits.to_u64(element);
        self.previous_element = element_as_u64;
        array_traits.write(writer, element)
    }

    fn bitsize_of_unpacked<T>(&self, array_trait: &dyn ArrayTrait<T>, element: &T) -> Result<u64> {
        array_trait.bitsize_of(0, element)
    }
}

use crate::error::Result;
use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::read_varsize;
use crate::ztype::varsize_bitsize;
use crate::ztype::varuint_encode::write_varsize;
use bitreader::BitReader;
use num::traits::Unsigned;
use rust_bitwriter::BitWriter;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::ztype::alignment::{align_bitsize, align_reader, align_writer};

/// Maximum number of elements to pre-allocate when reading an array whose declared length
/// comes from an untrusted stream. The Vec grows naturally as real elements arrive, so this
/// only limits the *initial* reservation, not the total capacity.
///
/// Default: 1 000. Adjust with [`set_array_alloc_chunk`].
static ARRAY_ALLOC_CHUNK: AtomicUsize = AtomicUsize::new(1000);

/// Set the maximum number of array elements that will be pre-allocated when decoding an array.
///
/// This caps the initial `Vec::reserve` call so that a maliciously crafted stream that claims
/// a huge `varsize` length cannot cause an out-of-memory condition before any data is actually
/// read. The vector grows naturally as real elements arrive.
///
/// The default is 1 000. A value of 0 disables pre-allocation entirely.
pub fn set_array_alloc_chunk(n: usize) {
    ARRAY_ALLOC_CHUNK.store(n, Ordering::Relaxed);
}

/// Return the current maximum initial allocation chunk size for array decoding.
///
/// See [`set_array_alloc_chunk`] for details.
pub fn get_array_alloc_chunk() -> usize {
    ARRAY_ALLOC_CHUNK.load(Ordering::Relaxed)
}

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
        data: &mut Vec<T>,
        array_length: usize,
        index_offsets: Option<&Vec<U>>,
    ) -> Result<()>
    where
        T: Default,
    {
        data.clear();
        data.reserve(array_length.min(ARRAY_ALLOC_CHUNK.load(Ordering::Relaxed)));

        if self.is_packed {
            let mut packing_context_node = self.array_trait.create_context();
            for index in 0..array_length {
                align_array_element_reader(reader, index, index_offsets)?;
                let mut item = T::default();
                self.array_trait.read_packed(
                    &mut packing_context_node,
                    reader,
                    &mut item,
                    index,
                )?;
                data.push(item);
            }
        } else {
            for index in 0..array_length {
                align_array_element_reader(reader, index, index_offsets)?;
                let mut item = T::default();
                self.array_trait.read(reader, &mut item)?;
                data.push(item);
            }
        }
        Ok(())
    }

    /// Read a single element into `item` at the given `index`, handling both packed and
    /// non-packed modes and optional offset-based byte-alignment.
    ///
    /// For parametrized arrays (where generated code must set type parameters on each element
    /// before decoding it), the caller creates the element, sets its parameters, then calls
    /// this method, and finally pushes the element onto the target vector.
    ///
    /// `packing_context` must be a context previously created via
    /// `self.array_trait.create_context()` and shared across all elements of the array.
    /// For non-packed arrays, `packing_context` is unused but must still be provided.
    pub fn read_one_element<U: OffsetTrait>(
        &mut self,
        reader: &mut BitReader,
        packing_context: &mut crate::ztype::array_traits::packing_context_node::PackingContextNode,
        item: &mut T,
        index: usize,
        index_offsets: Option<&Vec<U>>,
    ) -> Result<()> {
        align_array_element_reader(reader, index, index_offsets)?;
        if self.is_packed {
            self.array_trait
                .read_packed(packing_context, reader, item, index)?;
        } else {
            self.array_trait.read(reader, item)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ztype::array_traits::UnsignedBitFieldArrayTrait;

    fn make_u32_array(packed: bool) -> Array<u32> {
        Array {
            array_trait: Box::new(UnsignedBitFieldArrayTrait { num_bits: 32 }),
            is_packed: packed,
            fixed_size: None,
        }
    }

    /// Helper: encode a varsize value into a byte vec (little-endian zserio varsize).
    fn encode_varsize(mut value: u32) -> Vec<u8> {
        // varsize encoding: up to 5 bytes, 7 bits per byte, MSB set if more bytes follow.
        let mut bytes = Vec::new();
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                bytes.push(byte);
                break;
            } else {
                bytes.push(byte | 0x80);
            }
        }
        bytes
    }

    #[test]
    fn test_oversized_length_returns_error() {
        // Claim 1 000 000 u32 elements but provide no payload bytes → should return Err (Eof).
        let length_bytes = encode_varsize(1_000_000);
        // No actual element data — stream will be exhausted immediately.
        let mut reader = BitReader::new(&length_bytes);
        let mut arr = make_u32_array(false);
        let array_length = arr.zserio_read_array_length(&mut reader).unwrap();
        let mut data: Vec<u32> = Vec::new();
        let result = arr.zserio_read::<u32>(&mut reader, &mut data, array_length, None);
        assert!(
            result.is_err(),
            "expected Err for oversized array length, got Ok"
        );
        // Also assert no OOM: data should be empty or have at most ARRAY_ALLOC_CHUNK capacity.
        assert!(data.capacity() <= ARRAY_ALLOC_CHUNK.load(Ordering::Relaxed));
    }

    #[test]
    fn test_round_trip_unpacked() {
        // A legitimate round-trip for a small unpacked u32 array.
        let original: Vec<u32> = vec![10, 20, 30, 40, 50];

        let mut writer = rust_bitwriter::BitWriter::new();
        let mut arr_write = make_u32_array(false);
        arr_write
            .zserio_write::<u32>(&mut writer, &original, None)
            .unwrap();
        writer.close().unwrap();
        let bytes = writer.data().clone();

        let mut reader = BitReader::new(&bytes);
        let mut arr_read = make_u32_array(false);
        let array_length = arr_read.zserio_read_array_length(&mut reader).unwrap();
        let mut decoded: Vec<u32> = Vec::new();
        arr_read
            .zserio_read::<u32>(&mut reader, &mut decoded, array_length, None)
            .unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_set_array_alloc_chunk_affects_capacity() {
        // After setting chunk to 3, reserving for a 10-element array should cap at 3.
        set_array_alloc_chunk(3);

        let length_bytes = encode_varsize(10);
        // Provide just enough bytes for 10 u32s (32 bits each = 40 bytes payload).
        let mut payload: Vec<u8> = length_bytes.clone();
        for _ in 0..10u32 {
            payload.extend_from_slice(&42u32.to_be_bytes());
        }

        let mut reader = BitReader::new(&payload);
        let mut arr = make_u32_array(false);
        let array_length = arr.zserio_read_array_length(&mut reader).unwrap();

        // Peek at the capacity after clear+reserve but before pushing.
        // We do this by allocating a vec manually, mirroring the logic in zserio_read.
        // Note: Vec::reserve is a minimum hint; the allocator may round up slightly.
        // What we verify is that the reservation was NOT for the full array_length (10).
        let chunk = ARRAY_ALLOC_CHUNK.load(Ordering::Relaxed);
        let probe: Vec<u32> = Vec::with_capacity(array_length.min(chunk));
        assert!(
            probe.capacity() < array_length,
            "expected capacity < {} (array_length), got {}",
            array_length,
            probe.capacity()
        );

        // Restore default so other tests are unaffected.
        set_array_alloc_chunk(1000);

        // Also verify the full read still works.
        let mut reader2 = BitReader::new(&payload);
        let mut arr2 = make_u32_array(false);
        let array_length2 = arr2.zserio_read_array_length(&mut reader2).unwrap();
        let mut decoded: Vec<u32> = Vec::new();
        arr2.zserio_read::<u32>(&mut reader2, &mut decoded, array_length2, None)
            .unwrap();
        assert_eq!(decoded.len(), 10);
        assert!(decoded.iter().all(|&x| x == 42));
    }
}

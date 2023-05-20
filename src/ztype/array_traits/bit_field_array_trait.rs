
use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use crate::ztype::array_traits::array_trait;
use crate::ztype::array_traits::packing_context_node::PackingContextNode;
use crate::ztype::read_signed_bits;
use crate::ztype::write_signed_bits;

pub struct BitFieldArrayTrait {
	pub num_bits: u8,
}

impl array_trait::ArrayTrait<i64> for BitFieldArrayTrait {
    fn is_bitsizeof_constract(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        true
    }

    fn bitsize_of(&self) -> u8 {
        self.num_bits
    }

    fn initialize_offsets(&self, bit_position: u64) -> u64 {
        bit_position + self.bitsize_of() as u64
    }
    
    
    fn read(&self, reader: &mut BitReader) -> i64 {
        read_signed_bits(reader, self.bitsize_of())
    }

	fn write(&self, writer: &mut BitWriter, value: &i64) {
        write_signed_bits(writer, *value, self.bitsize_of());
    }

	fn to_u64(&self, value: &i64) -> u64 {
        return *value as u64;
    }
	fn from_u64(&self, value: u64) -> i64 {
        return value as i64;
    }

	fn init_context(&self, context_node: &mut PackingContextNode, element: &i64) {
        context_node.context.init(self, element);
    }

    fn bitsize_of_packed(&self, context_node: &mut PackingContextNode, bit_position: u64, element: &i64) -> u8 {
        context_node.context.bitsize_of(self, bit_position, element) as u8
    }
    fn initialize_offsets_packed(&self, bit_position: u64, element: &i64) -> u64 {
        0
    }

	fn write_packed(&self, context_node: &mut PackingContextNode, writer: &mut BitWriter, element: &i64) {
        context_node.context.write(self, writer, element);
    }


/*
    func (trait BitFieldArrayTraits[T]) InitializeOffsets(bitPosition int, value T) int {
        return bitPosition + trait.BitSizeOf(value, 0) // endBitPosition is ignored
    }
    
    func (trait BitFieldArrayTraits[T]) Read(reader zserio.Reader, endBitPosition int) (T, error) {
        value, err := reader.ReadBits(uint8(trait.NumBits))
        return T(value), err
    }
    
    func (trait BitFieldArrayTraits[T]) Write(writer zserio.Writer, value T) error {
        return writer.WriteBits(uint64(value), trait.NumBits)
    }
    */
}
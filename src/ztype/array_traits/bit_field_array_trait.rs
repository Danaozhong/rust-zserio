
use bitreader::BitReader;
use crate::ztype::array_traits::array_trait;

pub struct BitFieldArrayTrait {
	pub num_bits: u8,
}

impl array_trait::ArrayTrait for BitFieldArrayTrait {
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
    
    /*
    fn read<T>(&self, reader: &mut BitReader) -> T {
        //reader.read_signed_bits(self.num_bits)
        //reader.
    }
    */
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
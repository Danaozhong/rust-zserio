
use bitreader::BitReader;

pub struct BitFieldArrayTrait {
	pub num_bits: u8,
}

impl BitFieldArrayTrait {
    fn is_bitsizeof_constract(&self) -> bool {
        true
    }

    fn needs_bitsizeof_position(&self) -> bool {
        true
    }

    fn bitsize_of(&self) -> u8 {
        self.num_bits
    }

    fn initialize_offsets(&self, bit_position: unsigned) -> unsigned {
        bit_position + self.bitsize_of()
    }
    
    fn read<T>(&self, mut reader: &BitReader) -> T {
        reader.read_signed_bits(self.num_bits)
        //reader.
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
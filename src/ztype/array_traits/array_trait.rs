use bitreader::BitReader;

pub trait ArrayTrait {

    fn is_bitsizeof_constract(&self) -> bool;
    fn needs_bitsizeof_position(&self) -> bool;
    fn bitsize_of(&self) -> u8;
    fn initialize_offsets(&self, bit_position: u64) -> u64;
    //fn read<T>(&self, reader: &mut BitReader) -> T;

/*

// BitSizeOfIsConstant returns true if the bit size is constant for every
	// array element, for example int32.
	BitSizeOfIsConstant() bool

	// NeedsBitsizeOfPosition is true if the array traits need to know the bit
	// size of an array element.
	NeedsBitsizeOfPosition() bool

	// NeedsReadIndex specifies if the traits need the array index of the element.
	NeedsReadIndex() bool

	// PackedTraits will return the array trait as a packed object.
	PackedTraits() IPackedArrayTraits[T]

	// BitSizeOf returns the bit size of array element. If the sizee depends on
	// the position within the bit stream, the endBitPosition parameter is taken
	// into account.
	BitSizeOf(element T, endBitPosition int) int

	// InitializeOffsets returns the end bit position of an array element within
	// a byte stream.
	InitializeOffsets(bitPosition int, value T) int

	// Read reads an array element from a byte stream.
	Read(reader zserio.Reader, endBitPosition int) (T, error)

	// Write writes an array element to a byte stream.
	Write(writer zserio.Writer, value T) error

	// AsUint64 returns the array element as a uint64. This is only needed if the
	// underlying data type supports delta encoding, such as int32, etc.
	// Other datatypes, such as string, float, can implement this empty.
	AsUint64(value T) uint64
	// FromUint64 returns an array element from a uint64. This is only needed
	// if the underlying data type supports delta encoding.
	FromUint64(value uint64) T
     */


}
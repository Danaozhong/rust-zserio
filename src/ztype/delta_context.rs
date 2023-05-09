
use crate::ztype::array_traits::array_trait::ArrayTrait;
use crate::ztype::bits_decode::read_signed_bits;

use bitreader::BitReader;

const MAX_BIT_NUMBER_LIMIT: u8 =  62;
const MAX_BIT_NUMBER_BITS: u8 = 6;

pub struct DeltaContext {
	is_packed: bool,
    max_bit_number: u8,
    unpacked_size: unsigned,
    first_element_size: u64,
    num_elements: unsigned,
    previous_element: u64,
    init_started: bool,
	processing_started: bool,
}

impl DeltaContext {
    fn init<T>(&self, array_traits: &ArrayTrait, element: T) {
        self.num_elements += 1;
        self.unpacked_size += array_trait.bitsize_of();

        if !self.init_started {
            self.init_started = true;
            self.previous_element = element as u64;
			self.first_element_size = self.unpacked_size;

        } else if self.max_bit_number <= MAX_BIT_NUMBER_LIMIT {
			self.is_packed = true;
			// TODO
			/*
			delta := absDiff(arrayTraits.AsUint64(element), *context.previousElement)
			maxBitNumber := bits.Len64(delta)
			if maxBitNumber > context.maxBitNumber {
				context.maxBitNumber = maxBitNumber
				// cannot store using delta packing if the 64bit range is
				// exhausted
				if maxBitNumber > maxBitNumberLimit {
					context.isPacked = false
				}
			}
			*context.previousElement = arrayTraits.AsUint64(element)
			*/
        }
    }

	/*
	// BitSizeOf returns the size of the delta context array in bits.
	func (context *DeltaContext[T]) BitSizeOf(arrayTraits IArrayTraits[T], bitPosition int, element T) (int, error) {
	if !context.processingStarted {
		context.processingStarted = true
		context.finishInit()
		return context.BitSizeOfDescriptor() + bitsizeOfUnpacked(arrayTraits, element), nil
	}
	if !context.isPacked {
		return bitsizeOfUnpacked(arrayTraits, element), nil
	}
	if context.maxBitNumber > 0 {
		return context.maxBitNumber + 1, nil
	}
	return 0, nil
}
 */
	fn bitsize_of<T>(&self,  array_traits: &ArrayTrait, bit_position: unsigned, element: T) -> unsigned {
		if !self.processing_started {
			self.processing_started = true;

			// self.finish_init();
			return self.bitsize_of_descriptor() + self.bitsize_of_unpacked(array_traits, element);
		}
		if !self.is_packed {
			return self.bitsize_of_unpacked(array_traits, element);
		}
		if self.max_bit_number > 0 {
			return self.max_bit_number + 1
		}
		0
	}


	fn read<T>(&self, array_traits: &ArrayTrait, reader: BitReader) -> T {
		if !self.processing_started {
			self.processing_started = true;
			self.read_descriptor(reader);
			return self.read_unpacked(array_traits, reader);
		}
		if !self.is_packed {
			return self.read_unpacked(array_trait, reader);
		}
		if self.max_bit_number > 0 {
			delta = read_signed_bits(reader, self.max_bit_number + 1);
			self.previous_element = self.previous_element + delta;
		}
		self.previous_element as T
	}

}


/*

// DeltaContext is a packing context used when writing data using delta
// packing, i.e. instead of storing all values, only stores the deltas.
type DeltaContext[T any] struct {

	// specifies if delta packing is actually used (it may be skipped if normal
	// packing is more efficient)
	isPacked bool

	// maxBitNumber specifies the number of bits needed per delta element
	maxBitNumber int

	// previousElement is the value of the previously stored element
	previousElement     *uint64
	processingStarted   bool
	unpackedBitSize     int
	firstElementBitSize int
	numElements         int
}

// arrayTraitsBitsizeOf returns the bit size of an array element.
func arrayTraitsBitsizeOf[T any](arrayTraits IArrayTraits[T], bitPosition int, element T) int {
	return arrayTraits.BitSizeOf(element, bitPosition)
}

func absDiff(lhs, rhs uint64) uint64 {
	if lhs > rhs {
		return lhs - rhs
	}
	return rhs - lhs
}



// Write writes an element of an delta context array.
func (context *DeltaContext[T]) Write(arrayTraits IArrayTraits[T], writer zserio.Writer, element T) error {
	if !context.processingStarted {
		context.processingStarted = true
		context.finishInit()
		if err := context.WriteDescriptor(writer); err != nil {
			return err
		}
		return context.writeUnpacked(arrayTraits, writer, element)
	}
	if !context.isPacked {
		return context.writeUnpacked(arrayTraits, writer, element)
	}
	if context.maxBitNumber > 0 {
		delta := arrayTraits.AsUint64(element) - *context.previousElement
		err := writer.WriteBits(delta, uint8(context.maxBitNumber+1))
		if err != nil {
			return err
		}
		*context.previousElement = arrayTraits.AsUint64(element)
	}
	return nil
}

// finishInit decided if the array should be written packed or unpacked,
// depending on which variant is more space-efficient.
func (context *DeltaContext[T]) finishInit() {
	if context.isPacked {
		deltaBitsize := 0
		if context.maxBitNumber > 0 {
			deltaBitsize = context.maxBitNumber + 1
		}
		// decide if this array should be packed or not by comparing the array
		// bit sizes of both methods. Packed is usually more efficient if the
		// the array values are not differing too much from each other.
		packedBitsizeWithDescriptor := 1 + maxBitNumberBits +
			context.firstElementBitSize + (context.numElements-1)*deltaBitsize

		unpackedBitsizeWithDescriptor := 1 + context.unpackedBitSize

		if packedBitsizeWithDescriptor >= unpackedBitsizeWithDescriptor {
			context.isPacked = false
		}
	}
}

// BitSizeOfDescriptor returns the bit size of a delta context array descriptor.
func (context *DeltaContext[T]) BitSizeOfDescriptor() int {
	context.finishInit()
	if context.isPacked {
		return 1 + maxBitNumberBits
	}
	return 1
}

// ReadDescriptor reads the descriptor of a delta packed context.
func (context *DeltaContext[T]) ReadDescriptor(reader zserio.Reader) error {
	var err error
	if context.isPacked, err = reader.ReadBool(); err != nil {
		return err
	}
	if context.isPacked {
		// read how many bits are used for the delta encoding of each element
		var maxBitNumber uint64
		maxBitNumber, err = reader.ReadBits(maxBitNumberBits)
		context.maxBitNumber = int(maxBitNumber)
	}
	return err
}

// readUnpacked reads an unpacked array element from a delta context.
func (context *DeltaContext[T]) readUnpacked(arrayTraits IArrayTraits[T], reader zserio.Reader) (T, error) {
	element, err := arrayTraits.Read(reader, 0)
	if err != nil {
		return arrayTraits.FromUint64(0), err
	}
	elementAsUint64 := arrayTraits.AsUint64(element)
	context.previousElement = &elementAsUint64
	return element, nil
}

// WriteDescriptor writes the descriptor of a delta packed context.
func (context *DeltaContext[T]) WriteDescriptor(writer zserio.Writer) error {
	if err := writer.WriteBool(context.isPacked); err != nil {
		return err
	}
	if context.isPacked {
		return writer.WriteBits(uint64(context.maxBitNumber), maxBitNumberBits)
	}
	return nil
}

// writeUnpacked writes an unpacked array element to a writer.
func (context *DeltaContext[T]) writeUnpacked(arrayTraits IArrayTraits[T], writer zserio.Writer, element T) error {
	elementAsUint64 := arrayTraits.AsUint64(element)
	context.previousElement = &elementAsUint64
	return arrayTraits.Write(writer, element)
}

// bitsizeOfUnpacked returns the unpacked bit size of an array element.
func bitsizeOfUnpacked[T any](arrayTraits IArrayTraits[T], element T) int {
	return arrayTraits.BitSizeOf(element, 0)
}
 */
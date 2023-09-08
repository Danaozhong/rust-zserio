import zserio
from reference_modules.packed_arrays.packed_arrays.packed_array_wrapper import PackedArrayWrapper

def test_packed_arrays() -> None:
    filename = "tests/binaries/packed_array_test_rust.bin"

    reader = zserio.BitStreamReader.from_file(filename)
    packing_context_node = zserio.array.PackingContextNode()
    packed_array_wrapper = PackedArrayWrapper.from_reader_packed(packing_context_node, reader)
    #packed_array_wrapper: PackedArrayWrapper = zserio.deserialize_from_bytes(PackedArrayWrapper, binary_content)
    assert len(packed_array_wrapper.packed_array) == 3


if __name__ == "__main__":
    test_packed_arrays()
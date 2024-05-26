use reference_module_lib::reference_modules::bitmask_test::bitmask_test::{
    bitmask_test::BitmaskTest, some_bit_mask::SomeBitMask,
};

use rust_zserio::ztype::ZserioPackableObject;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub fn test_bitmasks() {
    let mut test_struct = BitmaskTest::new();
    test_struct.selector = SomeBitMask::HasA | SomeBitMask::HasB;

    test_struct.value_a = 123;
    test_struct.value_b = 456;
    test_struct.value_c = 678; // Should be ignored.

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = BitmaskTest::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct.zserio_read(&mut bitreader);

    // expect them to be identical.
    assert!(test_struct.selector == other_test_struct.selector);
    assert!(test_struct.value_a == other_test_struct.value_a);
    assert!(test_struct.value_b == other_test_struct.value_b);

    // The following field should still be 0, because the condition is not used.
    assert!(other_test_struct.value_c == 0);
}

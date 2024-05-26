use reference_module_lib::reference_modules::bitmask_isset::bitmask_isset::{
    bitmask_test::BitmaskTest, some_bit_mask::SomeBitMask,
};

use rust_zserio::ztype::ZserioPackableObject;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub fn test_bitmask_isset_round_trip() {
    let mut test_struct = BitmaskTest::new();
    test_struct.value = SomeBitMask::FlagA | SomeBitMask::FlagB;

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
    assert!(test_struct.value == other_test_struct.value);
}

pub fn test_bitmask_isset_operator() {
    let mut test_struct = BitmaskTest::new();
    test_struct.value = SomeBitMask::FlagA | SomeBitMask::FlagB;

    // Ensure that the isset() operator works correctly.
    assert!(test_struct.has_a());
    assert!(test_struct.has_b());
    assert!(!test_struct.has_c());
    assert!(test_struct.has_a_or_c());

    // Change the bitmask value. The value change should reflect
    // in the output of the functions that use the isset()
    // operator.
    test_struct.value = SomeBitMask::FlagC;

    assert!(!test_struct.has_a());
    assert!(!test_struct.has_b());
    assert!(test_struct.has_c());
    assert!(test_struct.has_a_or_c());

    test_struct.value = SomeBitMask::none();
    assert!(!test_struct.has_a());
    assert!(!test_struct.has_b());
    assert!(!test_struct.has_c());
    assert!(!test_struct.has_a_or_c());
}

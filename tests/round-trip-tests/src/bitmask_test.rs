use reference_module_lib::reference_modules::bitmask_test::bitmask_test::{
    bitmask_test::BitmaskTest, bitmask_with_zero::BitmaskWithZero, some_bit_mask::SomeBitMask,
};

use rust_zserio::ztype::ZserioPackableObject;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub fn test_bitmasks() {
    let test_struct = BitmaskTest {
        selector: SomeBitMask::HasA | SomeBitMask::HasB,
        value_a: 123,
        value_b: 456,
        value_c: 678, // Should be ignored.
    };

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = BitmaskTest::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct.selector == other_test_struct.selector);
    assert!(test_struct.value_a == other_test_struct.value_a);
    assert!(test_struct.value_b == other_test_struct.value_b);

    // The following field should still be 0, because the condition is not used.
    assert!(other_test_struct.value_c == 0);
}

pub fn test_bitmask_values_with_zero() {
    // test the assigned values of the bitmask.
    assert!(BitmaskWithZero::NoValue == 0);
    assert!(BitmaskWithZero::ValueA == 1);
    assert!(BitmaskWithZero::ValueB == 4);
    assert!(BitmaskWithZero::ValueC == 8);
}

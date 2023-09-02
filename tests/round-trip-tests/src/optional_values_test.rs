use crate::reference_modules::optional_values::optional_values::{
    option_enum::OptionEnum,
    optional_values_test::OptionalValuesTest,
};
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableOject;

pub fn test_optional_values() {
    let mut test_struct = OptionalValuesTest::new();
    test_struct.field_selector = OptionEnum::HasB;
    test_struct.field_a = 123; // Should be ignored.
    test_struct.field_b = 456; // Should be serialized.
    test_struct.field_c = 789; // Should be ignored.

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = OptionalValuesTest::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct.zserio_read(&mut bitreader);

    // expect them to be identical.
    assert!(test_struct.field_selector == other_test_struct.field_selector);
    assert!(test_struct.field_b == other_test_struct.field_b);

    // The following fields should still be 0, because the condition is not used.
    assert!(other_test_struct.field_a == 0);
    assert!(other_test_struct.field_c == 0);
}

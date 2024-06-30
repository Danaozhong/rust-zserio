use bitreader::BitReader;
use reference_module_lib::reference_modules::optional_values::optional_values::{
    option_enum::OptionEnum, optional_values_test::OptionalValuesTest,
};
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableObject;

pub fn test_optional_values() {
    let mut test_struct = OptionalValuesTest::new();
    test_struct.field_selector = OptionEnum::HasB;
    test_struct.field_a = 123; // Should be ignored.
    test_struct.field_b = 456; // Should be serialized.
    test_struct.field_c = 789; // Should be ignored.

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = OptionalValuesTest::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct.field_selector == other_test_struct.field_selector);
    assert!(test_struct.field_b == other_test_struct.field_b);

    // The following fields should still be 0, because the condition is not used.
    assert!(other_test_struct.field_a == 0);
    assert!(other_test_struct.field_c == 0);
}

pub fn test_optional_members() {
    let mut test_struct = OptionalValuesTest::new();
    test_struct.option_str_field = Some("Test".to_owned());
    test_struct.option_custom_str_field = Some("OtherTest".to_owned());
    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = OptionalValuesTest::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct.option_str_field == other_test_struct.option_str_field);
    assert!(test_struct.option_custom_str_field == other_test_struct.option_custom_str_field);
}

/// This test case tests optional arrays.
/// It ensures that zserio structures with optional arrays compile, and the arrays can be correctly
/// set and deserialized.
pub fn test_optional_arrays() {
    let mut test_struct = OptionalValuesTest::new();
    test_struct.option_string_array = Some(vec![
        "Hokkien Mee".to_string(),
        "Kaya Toast".to_string(),
        "Char Kway Teow".to_string(),
    ]);
    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = OptionalValuesTest::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct.option_string_array == other_test_struct.option_string_array);
}

use reference_module_lib::reference_modules::subtyped_dot_expression::test::test_struct::TestStruct;
use reference_module_lib::reference_modules::subtyped_dot_expression::subtyped_enum::subtyped_enum::SubtypedEnum;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::ZserioPackableObject;

pub fn test_subtyped_dot_expression() {
    let mut test_struct = TestStruct::new();
    test_struct.value_1 = SubtypedEnum::TestValueB;
    test_struct.opt_value_2 = 20;

    // serialize.
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = TestStruct::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}

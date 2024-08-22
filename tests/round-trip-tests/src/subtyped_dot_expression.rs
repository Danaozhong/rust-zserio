use reference_module_lib::reference_modules::subtyped_dot_expression::subtyped_enum::SubtypedEnum;
use reference_module_lib::reference_modules::subtyped_dot_expression::test::TestStruct;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use zserio::ZserioPackableObject;

pub fn test_subtyped_dot_expression() {
    let test_struct = TestStruct {
        value_1: SubtypedEnum::TestValueB,
        opt_value_2: 20,
        ..Default::default()
    };

    // serialize.
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = TestStruct::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}

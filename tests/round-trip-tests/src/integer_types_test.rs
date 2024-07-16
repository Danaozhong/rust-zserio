use bitreader::BitReader;
use reference_module_lib::reference_modules::integer_types::integer_types::integer_types_test::IntegerTypesTest;
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableObject;

pub fn test_integer_types() {
    let test_struct = IntegerTypesTest {
        i_16_value: -1,
        i_32_value: -2,
        i_64_value: -3,
        i_5_value: 4,
        varidyn_value: -5,
        vari_value: -6,
        vari_32_value: -7,
        vari_64_value: -8,
        u_16_value: 10,
        u_32_value: 11,
        u_64_value: 12,
        u_5_value: 13,
        varudyn_value: 14,
        varu_value: 15,
        ver_size_value: 16,
        varu_32_value: 17,
        varu_64_value: 18,
    };
    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = IntegerTypesTest::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}

use reference_module_lib::reference_modules::constants::constants::constant_test_struct::ConstantTestStruct;

use rust_zserio::ztype::ZserioPackableObject;

pub fn test_constants() {
    // The test structure created in this test generates a function that
    // requires a lot of type casts.
    // The test passes if the generated structure compiles.
    let mut test_struct = ConstantTestStruct::new();
    test_struct.str_value = "StrValue".to_owned();
    test_struct.other_str_value = "Other".to_owned();

    // CONSTANT_STRING + "DummyString" + CONSTANT_STRING;
    let expected_result = "TestDummyStringTest";
    let result = test_struct.test_string_constants(); 
    assert!(result.as_str() == expected_result, "Test Strings don't match expected: {:?}, got: {:?}", &expected_result, &result);

    // Test the function calls on the dynamic string types.
    assert!(test_struct.str_value == test_struct.test_string_constants_2());
    assert!(test_struct.other_str_value == test_struct.test_string_constants_3());
    
}

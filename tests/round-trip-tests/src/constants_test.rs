use crate::reference_modules::constants::constants::constant_test_struct::ConstantTestStruct;

use rust_zserio::ztype::ZserioPackableOject;

pub fn test_constants() {
    // The test structure created in this test generates a function that
    // requires a lot of type casts.
    // The test passes if the generated structure compiles.
    let mut test_struct = ConstantTestStruct::new();
    test_struct.str_value = "StrValue".to_owned();
    test_struct.other_str_value = "Other".to_owned();

    // strValue + CONSTANT_STRING + "DummyString" + otherStrValue + CONSTANT_STRING + strValue;
    let expected_result = "StrValueTestDummyStringOtherTestStrValue";
    let result = test_struct.test_string_constants(); 
    assert!(result.as_str() == expected_result, "Test Strings don't match expected: {:?}, got: {:?}", &expected_result, &result);
}

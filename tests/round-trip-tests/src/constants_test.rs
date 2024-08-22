use reference_module_lib::reference_modules::constants::constants::ConstantTestStruct;

pub fn test_constants() {
    // The test structure created in this test generates a function that
    // requires a lot of type casts.
    // The test passes if the generated structure compiles.
    let test_struct = ConstantTestStruct {
        str_value: String::from("StrValue"),
        other_str_value: String::from("Other"),
    };

    // CONSTANT_STRING + "DummyString" + CONSTANT_STRING;
    let expected_result = "TestDummyStringTest";
    let result = test_struct.test_string_constants();
    assert!(
        result.as_str() == expected_result,
        "Test Strings don't match expected: {:?}, got: {:?}",
        &expected_result,
        &result
    );

    // Test the function calls on the dynamic string types.
    assert!(test_struct.str_value == test_struct.test_string_constants_2());
    assert!(test_struct.other_str_value == test_struct.test_string_constants_3());
}

use reference_module_lib::reference_modules::core::types::color::Color;
use reference_module_lib::reference_modules::core::types::value_wrapper::ValueWrapper;
use rust_zserio::ztype::ZserioPackableObject;

pub fn test_debug_trait() {
    let mut value_wrapper = ValueWrapper::new();
    value_wrapper.parameter = 7;
    value_wrapper.value = 14;
    value_wrapper.enum_value = Color::Red;
    value_wrapper.description = "test".into();
    value_wrapper.fixed_array = vec![100, 101, 102, 103];
    value_wrapper.packed_array = vec![200, 201, 202, 203, 205, 204];

    assert_eq!(
        format!("{value_wrapper:?}"),
        r#"ValueWrapper { parameter: 7, value: 14, other_value: 0, enum_value: Red, description: "test", opt_int_32: None, fixed_array: [100, 101, 102, 103], str_array: [], packed_array: [200, 201, 202, 203, 205, 204] }"#
    );
}

use reference_module_lib::reference_modules::core::types::color::Color;
use reference_module_lib::reference_modules::core::types::value_wrapper::ValueWrapper;

pub fn test_debug_trait() {
    let value_wrapper = ValueWrapper {
        parameter: 7,
        value: 14,
        enum_value: Color::Red,
        description: "test".into(),
        fixed_array: vec![100, 101, 102, 103],
        packed_array: vec![200, 201, 202, 203, 205, 204],
        ..Default::default()
    };

    assert_eq!(
        format!("{value_wrapper:?}"),
        r#"ValueWrapper { parameter: 7, value: 14, other_value: 0, enum_value: Red, description: "test", opt_int_32: None, fixed_array: [100, 101, 102, 103], str_array: [], packed_array: [200, 201, 202, 203, 205, 204] }"#
    );
}

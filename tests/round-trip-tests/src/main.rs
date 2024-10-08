pub mod alignment_test;
pub mod ambiguous_types_test;
pub mod bitmask_isset_test;
pub mod bitmask_test;
pub mod constants_test;
pub mod debug_trait_test;
pub mod expr_numbits_test;
pub mod integer_types_test;
pub mod offsets_test;
pub mod optional_values_test;
pub mod packed_arrays_test;
pub mod parameter_passing_bitmask_test;
pub mod parameter_passing_templates_test;
pub mod parameter_passing_test;
pub mod parameterized_array_length_test;
pub mod subtyped_dot_expression;
pub mod template_instantiation_test;
pub mod type_casts_test;
pub mod valueof_operator_test;

use reference_module_lib::reference_modules::core::types::{
    basic_choice::BasicChoice, color::Color, extern_test_case::ExternTestCase, some_enum::SomeEnum,
    value_wrapper,
};

use reference_module_lib::reference_modules::type_lookup_test::ztype::union_type::{
    UnionType, UnionTypeSelector,
};
use reference_module_lib::reference_modules::type_lookup_test::ztype::z_type_struct::ZTypeStruct;

use crate::alignment_test::{test_alignment, test_alignment_roundtrip};
use crate::ambiguous_types_test::test_ambiguous_types;
use crate::bitmask_isset_test::{test_bitmask_isset_operator, test_bitmask_isset_round_trip};
use crate::bitmask_test::{test_bitmask_values_with_zero, test_bitmasks};
use crate::constants_test::test_constants;
use crate::debug_trait_test::test_debug_trait;
use crate::expr_numbits_test::test_expr_numbits;
use crate::integer_types_test::test_integer_types;
use crate::offsets_test::test_offsets;
use crate::optional_values_test::{
    test_optional_arrays, test_optional_members, test_optional_values,
};
use crate::packed_arrays_test::test_packed_arrays;
use crate::parameter_passing_bitmask_test::test_passing_bitmask_parameter;
use crate::parameter_passing_templates_test::test_parameter_passing_templates;
use crate::parameter_passing_test::{test_index_operator, test_parameter_passing};
use crate::parameterized_array_length_test::test_parameterized_array_length;
use crate::subtyped_dot_expression::test_subtyped_dot_expression;
use crate::type_casts_test::test_type_casts;
use crate::valueof_operator_test::test_valueof_operator;
use bitreader::BitReader;
use reference_module_lib::reference_modules::core::instantiations::instantiated_template_struct;
use rust_bitwriter::BitWriter;
use zserio::{Result, ZserioPackableObject};

fn main() {
    test_structure().expect("structure tests failed");
    test_functions();
    test_choice();
    test_template_instantiation();
    test_functions_in_instantiated_templates();
    test_extern_type();
    test_type_lookup();
    test_union_type();
    test_parameter_passing();
    test_parameter_passing_templates();
    test_passing_bitmask_parameter();
    test_index_operator();
    test_ambiguous_types();
    test_type_casts();
    test_optional_values();
    test_optional_members();
    test_optional_arrays();
    test_bitmasks();
    test_bitmask_values_with_zero();
    test_bitmask_isset_operator();
    test_bitmask_isset_round_trip();
    test_constants();
    test_integer_types();
    test_parameterized_array_length().unwrap();
    test_alignment().unwrap();
    test_alignment_roundtrip().unwrap();
    test_packed_arrays();
    test_subtyped_dot_expression();
    test_expr_numbits();
    test_valueof_operator();
    test_debug_trait();
    test_offsets();
}

fn test_structure() -> Result<()> {
    // This test generates a test structure, serializes it, deserializes it, and ensures
    // that the data is still the same.

    // Instantiate the data
    let value_wrapper = value_wrapper::ValueWrapper {
        parameter: 7,
        value: 14,
        enum_value: Color::Red, // this field only gets serialized, if parameter: 7
        description: "test".into(),
        fixed_array: vec![100, 101, 102, 103],
        packed_array: vec![200, 201, 202, 203, 205, 204],
        ..Default::default()
    };

    // serialize
    let mut bitwriter = BitWriter::new();
    value_wrapper.zserio_write(&mut bitwriter)?;
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_value_wrapper = value_wrapper::ValueWrapper {
        parameter: 7,
        ..Default::default()
    };

    // Deserialize.
    let mut bitreader = BitReader::new(serialized_bytes);
    other_value_wrapper.zserio_read(&mut bitreader)?;

    assert!(other_value_wrapper.value == value_wrapper.value);
    assert!(other_value_wrapper.enum_value == value_wrapper.enum_value);
    assert!(other_value_wrapper.other_value == value_wrapper.other_value);
    assert!(other_value_wrapper.description == value_wrapper.description);
    assert!(other_value_wrapper.fixed_array == value_wrapper.fixed_array);
    assert!(other_value_wrapper.packed_array == value_wrapper.packed_array);

    // serialize the new structure again, and ensure it is binary identical
    let mut other_bitwriter = BitWriter::new();
    other_value_wrapper.zserio_write(&mut other_bitwriter)?;
    other_bitwriter.close().expect("failed to close bit stream");
    let other_serialized_bytes = other_bitwriter.data();
    assert!(other_serialized_bytes == serialized_bytes);
    Ok(())
}

fn test_functions() {
    // This test generates a test structure, puts some values in it, and and ensures
    // that the function is generated correctly.

    // Instantiate the data
    let value_wrapper = value_wrapper::ValueWrapper {
        parameter: 2,
        value: 9,
        ..Default::default()
    };
    // Call the function, and expect it to return the correct value.
    assert!(value_wrapper.get_some_random_value() == 36)
}
fn test_choice() {
    // This test generates a test zserio choice, serializes it, deserializes it, and ensures
    // that the data is identical to what was serialized.
    let choice_param = SomeEnum::AttrC;

    let basic_choice = BasicChoice {
        z_type: choice_param,
        field_c: 42,
        ..Default::default()
    };

    // Serialize to binary.
    let mut bitwriter = BitWriter::new();
    basic_choice
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // Deserialize the binary stream.
    let mut other_basic_choice = BasicChoice {
        z_type: choice_param,
        ..Default::default()
    };

    let mut bitreader = BitReader::new(serialized_bytes);
    other_basic_choice
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    assert!(other_basic_choice.field_c == basic_choice.field_c);

    // serialize the new structure again, and ensure it is binary identical
    let mut other_bitwriter = BitWriter::new();
    other_basic_choice
        .zserio_write(&mut other_bitwriter)
        .expect("can not write zserio data");
    other_bitwriter.close().expect("failed to close bit stream");
    let other_serialized_bytes = other_bitwriter.data();
    assert!(other_serialized_bytes == serialized_bytes);
}

fn test_template_instantiation() {
    // This function tests that templates can be successfully instantiated, and their
    // generated types can be serialized and deserialized.
    let mut z_struct = instantiated_template_struct::InstantiatedTemplateStruct::default();
    z_struct.field.description = "Test Description".into();
    z_struct.field.fixed_array = vec![0, 1, 2, 3];

    // serialize
    let mut bitwriter = BitWriter::new();
    z_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_struct = instantiated_template_struct::InstantiatedTemplateStruct::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    assert!(other_struct.field.description == z_struct.field.description);
    assert!(other_struct.field.fixed_array == z_struct.field.fixed_array);
}

fn test_functions_in_instantiated_templates() {
    // This function tests that templates can be successfully instantiated, functions that rely on
    // template properties (e.g. "T.getValue()") that are unknown before tempalte instantiation.
    let mut z_struct = instantiated_template_struct::InstantiatedTemplateStruct::default();
    z_struct.field.description = "Test Description".into();
    z_struct.parameter = 15;
    z_struct.field.value = 32;
    assert!(z_struct.get_value() == 64);
}

/// This test case tests that extern types (that store bit buffers) and bytes types (that store
/// byte buffers) can be serialized and deserialized without changes.
fn test_extern_type() {
    let mut extern_test_struct = ExternTestCase::default();

    // fill the extern buffer with three bytes and three bits
    extern_test_struct.extern_buffer.data_blob = vec![0xf1, 0xaa, 0x12, 0xe0];
    extern_test_struct.extern_buffer.bit_size = 3 * 8 + 3;

    // fill the bytes buffer with four bytes
    extern_test_struct.byte_buffer.data_blob = vec![0xf1, 0xaa, 0x12, 0xf1];
    extern_test_struct.byte_buffer.byte_size = 4;

    // serialize
    let mut bitwriter = BitWriter::new();
    extern_test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_struct = ExternTestCase::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    assert!(other_struct.byte_buffer == extern_test_struct.byte_buffer);
    assert!(other_struct.extern_buffer == extern_test_struct.extern_buffer);
}

fn test_type_lookup() {
    let mut ztype_struct = ZTypeStruct::default();
    // If this line compiles, the test is already successful. That means that the correct type
    // got looked up (integer).
    ztype_struct.ztype.ztype = 16;

    // To be extra safe, serialize it to make sure it doesn't crash on serialization.
    let mut bitwriter = BitWriter::new();
    ztype_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
}

fn test_union_type() {
    // This test case checks union types, by creating them, assigning a value,
    // and then performing a round-trip test.
    let union_instance = UnionType {
        // fill the bytes buffer with four bytes
        u_16_value: 32,
        union_selector: UnionTypeSelector::U16Value,
        ..Default::default()
    };

    // serialize
    let mut bitwriter = BitWriter::new();
    union_instance
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_union_instance = UnionType::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_union_instance
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    assert!(other_union_instance.union_selector as u32 == union_instance.union_selector as u32);
    assert!(other_union_instance.u_16_value == union_instance.u_16_value);
}

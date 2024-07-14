use reference_module_lib::reference_modules::parameter_passing_templates::parameter_passing_templates::{
    parameter_passing_templates::ParameterPassingTemplates,
    templated_blockstring::TemplatedBlockstring,
    templated_blockuint_64::TemplatedBlockuint64,
    templated_choiceuint_32_string::TemplatedChoiceuint32String,
    templated_unionuint_32_string::TemplatedUnionuint32String,
    templated_unionuint_32_string::TemplatedUnionuint32StringSelector,
};

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::ZserioPackableObject;

pub fn test_parameter_passing_templates() {
    // Create a test structure, which uses parameter passing on template types
    let test_struct: ParameterPassingTemplates = ParameterPassingTemplates {
        num_blocks: 2,
        num_elements: 3,
        templated_block: TemplatedBlockstring {
            num_elements: 3,
            items: vec![
                String::from("foo"),
                String::from("bar"),
                String::from("foobar"),
            ],
            condition_item: 10,
        },
        templated_blocks: vec![
            TemplatedBlockuint64 {
                num_elements: 3,
                items: vec![100, 101, 102],
                condition_item: 10,
            },
            TemplatedBlockuint64 {
                num_elements: 3,
                items: vec![200, 201, 202],
                condition_item: 10,
            },
        ],
        templated_union: TemplatedUnionuint32String {
            num_elements: 3,
            union_selector: TemplatedUnionuint32StringSelector::Value1,
            value_1: 42042,
            value_2: vec![], // ignored
        },
        templated_unions: vec![
            TemplatedUnionuint32String {
                num_elements: 3,
                union_selector: TemplatedUnionuint32StringSelector::Value2,
                value_1: 0, // ignored
                value_2: vec![
                    String::from("芝麻包"),
                    String::from("叉烧包"),
                    String::from("流沙包"),
                ],
            },
            TemplatedUnionuint32String {
                num_elements: 3,
                union_selector: TemplatedUnionuint32StringSelector::Value1,
                value_1: 195373,
                value_2: vec![], // ignored
            },
        ],
        selector: 1,
        templated_choice: TemplatedChoiceuint32String {
            selector: 1,
            value_1: 0, // ignored
            value_2: String::from("干炒牛河"),
        },
        templated_choices: vec![
            TemplatedChoiceuint32String {
                selector: 1,
                value_1: 0, // ignored
                value_2: String::from("炸酱面"),
            },
            TemplatedChoiceuint32String {
                selector: 1,
                value_1: 0, // ignored
                value_2: String::from(""),
            },
        ],
    };

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = ParameterPassingTemplates::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}

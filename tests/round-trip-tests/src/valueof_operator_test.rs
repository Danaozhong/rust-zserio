use reference_module_lib::reference_modules::valueof_operator::valueof_operator::{
    Color, OptionEnum, OtherEnum, ValueOfTest,
};

pub fn test_valueof_operator() {
    let mut zstruct = ValueOfTest {
        color: Color::Red,
        option_enum: OptionEnum::HasA,
        other_enum: OtherEnum::HasC,
    };

    // expect the valueof operators to compile, and return the correct results.
    assert!(zstruct.get_value_of_color() == 1);
    assert!(zstruct.get_value_of_option_enum() == 0);
    assert!(zstruct.get_value_of_other_enum() == 2);

    // Change the bitmask, and assume the value is correctly deduced.
    zstruct.color = Color::Green | Color::Blue;
    assert!(zstruct.get_value_of_color() == 12);

    zstruct.color = Color::none();
    assert!(zstruct.get_value_of_color() == 0);
}

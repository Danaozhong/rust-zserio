use reference_module_lib::reference_modules::ambiguous_types::main::AmbiguousTypesStruct;

pub fn test_ambiguous_types() {
    // Create a test structure, and assign a new instance.
    // If this line compiles, the test passes.
    let mut test_struct = AmbiguousTypesStruct { value: 17 };
    test_struct.value = 18;
}

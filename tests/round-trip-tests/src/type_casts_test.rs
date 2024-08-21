use reference_module_lib::reference_modules::type_casts::type_casts::TypeCastStruct;

pub fn test_type_casts() {
    // The test structure created in this test generates a function that
    // requires a lot of type casts.
    // The test passes if the generated structure compiles.
    let test_struct = TypeCastStruct::default();
    test_struct.test_type_casts();
}

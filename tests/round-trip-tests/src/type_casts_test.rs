use crate::reference_modules::type_casts::type_casts::type_cast_struct::TypeCastStruct;

use rust_zserio::ztype::ZserioPackableOject;

pub fn test_type_casts() {
    // The test structure created in this test generates a function that
    // requires a lot of type casts.
    // The test passes if the generated structure compiles.
    let test_struct = TypeCastStruct::new();
    test_struct.test_type_casts();
}

use crate::reference_modules::ambiguous_types::main::ambiguous_types_struct::AmbiguousTypesStruct;
use rust_zserio::ztype::ZserioPackableOject;

pub fn test_ambiguous_types() {
    // Create a test structure, and assign a new instance.
    // If this line compiles, the test passes.
    let mut test_struct = AmbiguousTypesStruct::new();
    test_struct.value = 17;
}

use reference_module_lib::reference_modules::template_instantiation::template_instantiation::{
    InstantiatedStruct1, InstantiatedStruct2,
};
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableObject;

pub fn test_ambiguous_types() {
    // Create a test structure, and assign a new instance.
    // If this line compiles, the test passes.
    let test_struct1 = InstantiatedStruct1 {
        value_1: 16,
        value_2: "test".into(),
    };

    let test_struct2 = InstantiatedStruct2 {
        value_1: "test".into(),
        value_2: 22,
    };

    let mut bitwriter = BitWriter::new();
    test_struct1
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    test_struct2
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
}

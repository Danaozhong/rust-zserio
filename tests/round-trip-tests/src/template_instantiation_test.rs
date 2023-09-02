use crate::reference_modules::template_instantiation::template_instantiation::
{   
    instantiated_struct_1::InstantiatedStruct1,
    instantiated_struct_2::InstantiatedStruct2,
};
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableOject;

pub fn test_ambiguous_types() {
    // Create a test structure, and assign a new instance.
    // If this line compiles, the test passes.
    let mut test_struct1 = InstantiatedStruct1::new();
    test_struct1.value_1 = 16;
    test_struct1.value_2 = "test".into();

    let mut test_struct2 = InstantiatedStruct2::new();
    test_struct2.value_1 = "test".into();
    test_struct2.value_2 = 22;

    let mut bitwriter = BitWriter::new();
    test_struct1.zserio_write(&mut bitwriter);
    test_struct2.zserio_write(&mut bitwriter);
}

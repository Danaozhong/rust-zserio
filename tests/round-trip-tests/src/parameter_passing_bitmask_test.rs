use reference_module_lib::reference_modules::parameter_passing_bitmask::parameter_passing_bitmask::{
    Item, SomeBitMask, ParameterPassingBitmask
};

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use zserio::ztype::ZserioPackableObject;

pub fn test_passing_bitmask_parameter() {
    // Create a test structure, which uses parameter passing
    let mut test_struct = ParameterPassingBitmask {
        some_mask: SomeBitMask::HasA,
        block: Item::default(),
    };

    // We will assign a random value to the optional field, then
    // serialize and deserialize the data. If parameter passing
    // works correctly, the conditional item should only be serialized
    // if the condition is correct.
    // the conditional item will only be serialized if the condition
    // `isset(someMask, SomeBitMask.HAS_A)` is true.
    test_struct.block.condition_item = 10;
    let other_test_struct = serialize_and_deserialize(&test_struct);

    // some_mask is set to `HasA`, as such the value should have been serialized/deserialized.
    assert!(other_test_struct.block.condition_item == 10);

    // do the same again, but change the parameter, so that the
    // conditional item doesn't get serialized.
    test_struct.some_mask = SomeBitMask::HasB | SomeBitMask::HasC;
    // parameters still need to be updated manually
    test_struct.block.some_mask = test_struct.some_mask;
    let yet_another_test_struct = serialize_and_deserialize(&test_struct);
    assert!(yet_another_test_struct.block.condition_item == 0);
}

fn serialize_and_deserialize(test_obj: &ParameterPassingBitmask) -> ParameterPassingBitmask {
    let mut bitwriter = BitWriter::new();
    test_obj
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    let serialized_bytes = bitwriter.data();

    let mut other_test_struct = ParameterPassingBitmask::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");
    other_test_struct
}

use bitreader::BitReader;
use reference_module_lib::reference_modules::packed_arrays::packed_arrays::{
    BubbleTeaAddons, BubbleTeaSize, DataStruct, PackedArrayWrapper,
};
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::array_traits::packing_context_node::PackingContextNode;

use rust_zserio::ztype::ZserioPackableObject;
use std::{fs::File, io::Write};

fn get_test_data() -> PackedArrayWrapper {
    let mut test_struct = PackedArrayWrapper {
        packed_array: vec![
            DataStruct::default(),
            DataStruct::default(),
            DataStruct::default(),
        ],
        standard_array: vec![DataStruct::default(), DataStruct::default()],
    };

    test_struct.packed_array[0].u_32_packed_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    test_struct.packed_array[1].u_32_packed_array =
        vec![325236346, 0, 563463, 1209341892, 11, 21424];
    test_struct.packed_array[2].u_32_packed_array = vec![0, 0, 0, 0, 0];

    test_struct.packed_array[0].u_32_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    test_struct.packed_array[1].u_32_array = vec![325236346, 0, 563463, 1209341892, 11, 21424];
    test_struct.packed_array[2].u_32_array = vec![0, 0, 0, 0, 0];

    test_struct.packed_array[0].size = BubbleTeaSize::HotMedium;
    test_struct.packed_array[1].size = BubbleTeaSize::ColdMedium;
    test_struct.packed_array[2].size = BubbleTeaSize::ColdLarge;

    test_struct.packed_array[0].tea_addons =
        BubbleTeaAddons::HasBlackSugar | BubbleTeaAddons::HasIce;
    test_struct.packed_array[1].tea_addons = BubbleTeaAddons::HasCreamCheese;

    test_struct.packed_array[2].bo_value_2 = true;
    test_struct.packed_array[2].i_16_value_3 = 453;
    test_struct.packed_array[2].str_value_4 = "Milo Dinosaur".into();

    test_struct.standard_array[0].u_32_packed_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    test_struct.standard_array[1].u_32_packed_array =
        vec![325236346, 0, 563463, 1209341892, 11, 21424];

    test_struct.standard_array[0].u_32_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    test_struct.standard_array[1].u_32_array = vec![325236346, 0, 563463, 1209341892, 11, 21424];

    test_struct
}

pub fn test_packed_arrays() {
    let test_struct = get_test_data();

    // serialize packed
    let mut bitwriter = BitWriter::new();
    let mut packing_context_node = PackingContextNode::new();
    PackedArrayWrapper::zserio_create_packing_context(&mut packing_context_node);

    test_struct
        .zserio_write_packed(&mut packing_context_node, &mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = PackedArrayWrapper::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    let mut new_context_node = PackingContextNode::new();
    PackedArrayWrapper::zserio_create_packing_context(&mut new_context_node);
    other_test_struct
        .zserio_read_packed(&mut new_context_node, &mut bitreader)
        .expect("can not read zserio data");

    assert!(test_struct == other_test_struct);
}

pub fn packed_arrays_serialize_to_file() {
    let filename = "tests/binaries/packed_array_test_rust.bin";
    let test_struct = get_test_data();

    // serialize packed
    let mut bitwriter = BitWriter::new();
    let mut packing_context_node = PackingContextNode::new();
    PackedArrayWrapper::zserio_create_packing_context(&mut packing_context_node);
    test_struct
        .zserio_write_packed(&mut packing_context_node, &mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    {
        let mut file = File::create(filename).expect("failed to open file");
        file.write_all(serialized_bytes.as_slice())
            .expect("failed to write to file");
    }
}

use reference_module_lib::reference_modules::parameter_passing::parameter_passing::{
    block::Block, parameter_passing::ParameterPassing,
};

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::ZserioPackableObject;

pub fn test_parameter_passing() {
    // Create a test structure, which uses parameter passing
    let test_struct = ParameterPassing {
        num_blocks: 2,
        num_elements: 1, // This is the only true value for num_elements - will be passed over to all others.
        block: Block {
            num_elements: 1,
            items: vec![10],    // should have one element, because num_elements = 1
            condition_item: 15, // should be serialize, because num_elements > 0
        },
        blocks: vec![
            Block {
                num_elements: 1, // taken from the root struct, but still needs to be set
                items: vec![11],
                condition_item: 16,
            },
            Block {
                num_elements: 1, // taken from the root struct, but still needs to be set
                items: vec![15],
                condition_item: 17,
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
    let mut other_test_struct = ParameterPassing::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct.num_elements == other_test_struct.num_elements);
    assert!(test_struct.num_blocks == other_test_struct.num_blocks);
    assert!(test_struct.block.condition_item == other_test_struct.block.condition_item);
    assert!(test_struct.block.items == other_test_struct.block.items);

    for (block_index, src_block) in test_struct.blocks.iter().enumerate() {
        let dst_block = &other_test_struct.blocks[block_index];
        assert!(src_block.num_elements == dst_block.num_elements);
        assert!(src_block.condition_item == dst_block.condition_item);
        assert!(src_block.items == dst_block.items);
    }
}

pub fn test_index_operator() {}

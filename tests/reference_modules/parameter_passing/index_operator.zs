package reference_modules.parameter_passing.index_operator;

// This is a simple test case to test the @index operator.
struct IndexOperator
{
    uint16                  numBlocks;
    BlockHeader             headers[numBlocks];

    // Note: the headers are passed as element-by-element parameters to the blocks array.
    Block(headers[@index])  blocks[numBlocks];
};

struct BlockHeader
{
    uint16 numItems;
};

struct Block(BlockHeader header)
{
    int64 items[header.numItems];

    // Add a conditional item, that depends on the parameter passed in the index.
    // This ensures that during serialization / deserialization, the item is correctly passed.
    uint8 conditionItem if lengthof(items) > 0;
};
package reference_modules.parameter_passing.parameter_passing;

// This is a simple test case to test parameter passing.
struct ParameterPassing
{
    uint16 numBlocks;
    uint32 numElements;

    // Test passing of parameters to a single instance.
    Block(numElements)  block;

    // Test passing of parameters to an array.
    Block(numElements) blocks[2];
};

struct Block(uint32 numElements)
{
    int64 items[numElements];

    // Add a conditional item, that depends on the parameter passed in the index.
    // This ensures that during serialization / deserialization, the item is correctly passed.
    uint8 conditionItem if lengthof(numElements) > 0;
};
package reference_modules.parameter_passing_templates.parameter_passing_templates;

// This is a variant of the parameterized test, using
// templated structs, union and choices to stress
// the parameter passing.
struct ParameterPassingTemplates
{
    uint16 numBlocks;
    uint32 numElements;

    // Test passing of parameters to a single instance.
    TemplatedBlock<string>(numElements)  templated_block;

    // Test passing of parameters to an array.
    TemplatedBlock<uint64>(numElements) templated_blocks[2];
};

struct TemplatedBlock<ITEM_T>(uint32 numElements)
{
    ITEM_T items[numElements];

    // Add a conditional item, that depends on the parameter passed in the index.
    // This ensures that during serialization / deserialization, the item is correctly passed.
    uint8 conditionItem if numElements > 0;
};

union TemplatedUnion<ITEM_T, ITEM_U>(uint32 numElements)
{
    ITEM_T value1;
    ITEM_U value2;

};

choice TemplatedChoice<ITEM_T, ITEM_U>(uint32 selector) on selector
{
    case 0:
        ITEM_T value1;
    case 1:
        ITEM_U value2;

};
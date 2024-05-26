package reference_modules.parameter_passing_bitmask.parameter_passing_bitmask;


bitmask uint8 SomeBitMask
{
    HAS_A,
    HAS_B = 0x04,
    HAS_C,
};

// This is a simple test case to passing a bitmask as a parameter.
struct ParameterPassingBitmask
{
    SomeBitMask someMask;

    // Test passing of parameters to a single instance.
    Item(someMask)  block;
};

struct Item(SomeBitMask someMask)
{
    // Add a conditional item, that depends on the parameter passed in the index.
    // This ensures that during serialization / deserialization, the item is correctly passed.
    uint8 conditionItem if isset(someMask, SomeBitMask.HAS_A);
};
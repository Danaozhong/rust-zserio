package reference_modules.optional_values.optional_values;

enum int32 OptionEnum {
    HAS_A = 0,
    HAS_B = 1,
    HAS_C = 2,
};

// This is a simple test case to test parameter passing.
struct OptionalValuesTest
{
    OptionEnum fieldSelector;
    uint16 fieldA if fieldSelector == OptionEnum.HAS_A;
    uint16 fieldB if fieldSelector == OptionEnum.HAS_B;
    uint16 fieldC if fieldSelector == OptionEnum.HAS_C;
};

package reference_modules.optional_array.optional_array;

subtype string CustomString;

enum int32 OptionEnum {
    HAS_A = 0,
    HAS_B = 1,
    HAS_C = 2,
};

struct ArrayElement(OptionEnum param) {
    uint32 field if param == OptionEnum.HAS_A;
};

struct OptionalArray
{
    OptionEnum param;
    optional ArrayElement(param) optional_array[];
    optional int32 int_opt_array[];
};

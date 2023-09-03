package reference_modules.optional_values.optional_values;

subtype string CustomString;

enum int32 OptionEnum {
    HAS_A = 0,
    HAS_B = 1,
    HAS_C = 2,
};

struct OptionalValuesTest
{
    // These fields test the option condition, i.e. fields that
    // are only serialized if the condition is fulfilled.
    OptionEnum fieldSelector;
    uint16 fieldA if fieldSelector == OptionEnum.HAS_A;
    uint16 fieldB if fieldSelector == OptionEnum.HAS_B;
    uint16 fieldC if fieldSelector == OptionEnum.HAS_C;

    // Below fields do not use a condition, but can be optionally
    // set, if the user wishes to.
    optional uint16 optionU16Field;
    optional string optionStrField;
    optional CustomString optionCustomStrField;
    optional CustomString optionStringArray[3];

};

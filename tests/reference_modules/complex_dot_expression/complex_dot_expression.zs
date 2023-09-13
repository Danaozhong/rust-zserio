package reference_modules.complex_dot_expression.complex_dot_expression;

bitmask uint8 SomeBitMask
{
    HAS_A,
    HAS_B = 0x04,
    HAS_C,
};

struct InnerStruct {
    SomeBitMask selector_bitmask;
};

struct MiddleStruct {
    InnerStruct value;
};

struct OuterStruct {
    MiddleStruct value;
};

struct TestStruct {
    OuterStruct value;
    int32 optValue if value.value.value.selector_bitmask == SomeBitMask.HAS_A;
};

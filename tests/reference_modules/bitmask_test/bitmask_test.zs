package reference_modules.bitmask_test.bitmask_test;

bitmask uint8 SomeBitMask
{
    HAS_A,
    HAS_B = 0x04,
    HAS_C,
};

struct BitmaskTest {
    SomeBitMask selector;
    int32 valueA if (selector & SomeBitMask.HAS_A) == SomeBitMask.HAS_A;
    int32 valueB if (selector & SomeBitMask.HAS_B) == SomeBitMask.HAS_B;
    int32 valueC if (selector & SomeBitMask.HAS_C) == SomeBitMask.HAS_C;
};
package reference_modules.packed_arrays.packed_arrays;


enum uint16 BubbleTeaSize {
    COLD_SMALL = 0,
    COLD_MEDIUM = 1,
    COLD_LARGE = 2,
    HOT_MEDIUM = 3,
    HOT_LARGE,
};

bitmask uint32 BubbleTeaAddons {
    HAS_ICE,
    HAS_PEARLS,
    HAS_BLACK_SUGAR,
    HAS_CREAM_CHEESE,
};

struct DataStruct
{
    packed uint32 u32PackedArray[];
    uint32 u32Array[];
    BubbleTeaSize size;
    BubbleTeaAddons teaAddons;
    varint32 vi32Value1;
    bool boValue2;
    int16 i16Value3;
    string strValue4;
};

struct PackedArrayWrapper {
    packed DataStruct packedArray[];
    DataStruct standardArray[];
};
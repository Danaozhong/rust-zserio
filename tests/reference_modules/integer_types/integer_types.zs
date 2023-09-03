package reference_modules.integer_types.integer_types;

struct IntegerTypesTest
{
    // Test reading/writing all integer types
    int16 i16Value;
    int32 i32Value;
    int64 i64Value;
    int<5> i5Value;
    int<i5Value> varidynValue;
    varint variValue;
    varint32 vari32Value;
    varint64 vari64Value;

    uint16 u16Value;
    uint32 u32Value;
    uint64 u64Value;
    bit<5> u5Value;
    bit<u5Value> varudynValue;
    varuint varuValue;
    varsize verSizeValue;
    varuint32 varu32Value;
    varuint64 varu64Value;

};

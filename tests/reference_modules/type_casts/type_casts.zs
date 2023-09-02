package reference_modules.type_casts.type_casts;

struct TypeCastStruct {
    int32 i32Value;
    uint32 u32Value;
    float16 f16Value;
    varint64 vi64Value;
    int32 i32Array[3];


    function float32 testTypeCasts()
    {
        // Test mixing different integer types, and make
        // sure the generated code handles type casts correctly.
        return i32Value + u32Value + vi64Value + lengthof(i32Array);   
    }
};
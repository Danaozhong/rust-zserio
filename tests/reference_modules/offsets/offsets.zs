package reference_modules.offsets.offsets;

struct Offsets {
    uint32 u32Offset;

    varint32 vi32Array[];

u32Offset:
    varint16 vi16OffsetArray[];

    uint32 u32ArrayOffset[];

u32ArrayOffset[@index]:
    varint64 vi64IndexOffsetArray[];

    uint8 u8Check;

u8Check:
    uint16 u16FinalCheck;

    bool hasFlag;
    uint16 u16Offset;
u16Offset:
    uint32 u32Value if hasFlag;

    uint16 u16YetFinalCheck;

};

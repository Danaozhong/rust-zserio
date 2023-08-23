pub const MAX_INT8: i8 = i8::MAX;
pub const MIN_INT8: i8 = -1 << 7;
pub const MAX_INT16: i16 = i16::MAX;
pub const MIN_INT16: i16 = -1 << 15;
pub const MAX_INT32: i32 = i32::MAX;
pub const MIN_INT32: i32 = -1 << 31;
pub const MAX_INT64: i64 = i64::MAX;
pub const MIN_INT64: i64 = -1 << 63;

pub const MAX_UINT8: u8 = 0xFF;
pub const MIN_UINT8: u8 = 0;
pub const MAX_UINT16: u16 = 0xFFFF;
pub const MIN_UINT16: u16 = 0;
pub const MAX_UINT32: u32 = 0xFFFFFFFF;
pub const MIN_UINT32: u32 = 0;
pub const MAX_UINT64: u64 = 0xFFFFFFFFFFFFFFFF;
pub const MIN_UINT64: u64 = 0;

pub const MAX_VARINT16: i16 = (1 << (6 + 8)) - 1;
pub const MIN_VARINT16: i16 = -MAX_VARINT16;
pub const MAX_VARINT32: i32 = (1 << (6 + 7 + 7 + 8)) - 1;
pub const MIN_VARINT32: i32 = -MAX_VARINT32;
pub const MAX_VARINT64: i64 = (1 << (6 + 7 + 7 + 7 + 7 + 7 + 7 + 8)) - 1;
pub const MIN_VARINT64: i64 = -MAX_VARINT64;
pub const MAX_VARINT: i64 = MAX_INT64;
pub const MIN_VARINT: i64 = MIN_INT64;

pub const MAX_VARUINT16: u16 = (1 << (7 + 8)) - 1;
pub const MIN_VARUINT16: u16 = 0;
pub const MAX_VARUINT32: u32 = (1 << (7 + 7 + 7 + 8)) - 1;
pub const MIN_VARUINT32: u32 = 0;
pub const MAX_VARUINT64: u64 = (1 << (7 + 7 + 7 + 7 + 7 + 7 + 7 + 8)) - 1;
pub const MIN_VARUINT64: u64 = 0;
pub const MAX_VARUINT: u64 = MAX_UINT64;
pub const MIN_VARUINT: u64 = 0;

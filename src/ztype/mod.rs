pub use self::string_encode::bitsize_string;
pub use self::string_encode::write_string;

pub use self::string_decode::read_string;

pub use self::bits_encode::write_i16;
pub use self::bits_encode::write_i32;
pub use self::bits_encode::write_i64;
pub use self::bits_encode::write_i8;

pub use self::bits_encode::write_u16;
pub use self::bits_encode::write_u32;
pub use self::bits_encode::write_u64;
pub use self::bits_encode::write_u8;

pub use self::bits_encode::write_signed_bits;
pub use self::bits_encode::write_unsigned_bits;

pub use self::bits_decode::read_i16;
pub use self::bits_decode::read_i32;
pub use self::bits_decode::read_i64;
pub use self::bits_decode::read_i8;

pub use self::bits_decode::read_u16;
pub use self::bits_decode::read_u32;
pub use self::bits_decode::read_u64;
pub use self::bits_decode::read_u8;

pub use self::bits_decode::read_signed_bits;
pub use self::bits_decode::read_unsigned_bits;

pub use self::varuint_bitsize::unsigned_bitsize;
pub use self::varuint_bitsize::varsize_bitsize;

pub use self::array::Array;

pub use self::bit_alignment::align_to;
pub use self::traits::ZserioPackableOject;

pub mod bits_decode;
pub mod bits_encode;
mod string_decode;
mod string_encode;
mod varuint_bitsize;
pub mod varuint_decode;
pub mod varuint_encode;

// all the modules required for arrays
mod array;
pub mod array_traits;
mod bit_alignment;

mod traits;

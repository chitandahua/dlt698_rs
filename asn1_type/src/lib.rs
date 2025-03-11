use octetstring::FixedOctetString;

mod bitstring;
pub use bitstring::BitString;

mod boolean;
pub use boolean::Boolean;

mod enumerated;
pub use enumerated::Enumerated;

mod integer;
pub use integer::Integer;

mod null;
pub use null::Null;

mod octetstring;
pub use octetstring::OctetString;

mod optional;

mod strings;
pub use strings::*;

pub mod traits;
mod unsigned_integer;

mod sequence;
pub use sequence::SequenceOf;

#[inline]
pub fn is_highest_bit_set(bytes: &[u8]) -> bool {
    bytes
        .first()
        .map(|byte| byte & 0b10000000 != 0)
        .unwrap_or(false)
}

pub type DoubleLong = i32;
pub type DoubleLongUnsigned = u32;
pub type Int = i8;
pub type Long = i16;
pub type Unsigned = u8;
pub type LongUnsigned = u16;
pub type Long64 = i64;
pub type Long64Unsigned = u64;

pub type Float32 = FixedOctetString<4>;
pub type Float64 = FixedOctetString<8>;
//type DateTime = FixedOctetString<10>;
//type Date = FixedOctetString<5>;
//type Time = FixedOctetString<3>;
//type DateTimes = FixedOctetString<7>;

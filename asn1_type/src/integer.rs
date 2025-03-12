use super::is_highest_bit_set;
use super::traits::{FromAxdr, ToAxdr};
use crate::{Error, ParseResult, Result, SerializeResult};
use std::borrow::Cow;
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
pub struct Integer<'a> {
    pub(crate) data: Cow<'a, [u8]>,
}

impl<'a> Integer<'a> {
    #[inline]
    pub const fn new(s: &'a [u8]) -> Self {
        Integer {
            data: Cow::Borrowed(s),
        }
    }

    pub fn from_const_array<const N: usize>(b: [u8; N]) -> Self {
        if is_highest_bit_set(&b) {
            let mut bytes = vec![0];
            bytes.extend_from_slice(&b);

            Integer {
                data: Cow::Owned(bytes),
            }
        } else {
            let mut idx = 0;

            while idx < b.len() - 1 {
                if b[idx] == 0 && b[idx + 1] < 0x80 {
                    idx += 1;
                    continue;
                }
                break;
            }

            Integer {
                data: Cow::Owned(b[idx..].to_vec()),
            }
        }
    }

    pub fn from_const_array_negative<const N: usize>(b: [u8; N]) -> Self {
        let mut idx = 0;
        while idx < b.len() - 1 {
            if b[idx] == 0xff && b[idx + 1] >= 0x80 {
                idx += 1;
                continue;
            }
            break;
        }

        if idx == b.len() {
            Integer {
                data: Cow::Borrowed(&[0]),
            }
        } else {
            Integer {
                data: Cow::Owned(b[idx..].to_vec()),
            }
        }
    }

    #[inline]
    fn need_length(&self) -> bool {
        match self.as_ref().len() {
            0 | 1 => is_highest_bit_set(self.as_ref()),
            _ => true,
        }
    }
}

macro_rules! impl_from_to {
    ($ty:ty, $from:ident, $to:ident) => {
        impl From<$ty> for Integer<'_> {
            fn from(i: $ty) -> Self {
                Self::$from(i)
            }
        }

        impl TryFrom<Integer<'_>> for $ty {
            type Error = Error;

            fn try_from(value: Integer<'_>) -> Result<Self> {
                value.$to()
            }
        }
    };
    (IMPL SIGNED $ty:ty, $from:ident, $to:ident) => {
        impl_from_to!($ty, $from, $to);

        impl Integer<'_> {
            pub fn $to(&self) -> Result<$ty> {
                let size = std::mem::size_of::<$ty>();
                let bytes = self.as_ref();
                let is_negative = is_highest_bit_set(bytes);

                let mut extended = vec![0u8; size];
                if is_negative {
                    extended.fill(0xFF);
                }
                if size > bytes.len() {
                    extended[size - bytes.len()..].copy_from_slice(bytes);
                } else {
                    extended.copy_from_slice(&bytes[bytes.len() - size..]);
                };

                Ok(<$ty>::from_be_bytes(extended.try_into().unwrap()))
            }

            pub fn $from(i: $ty) -> Self {
                let b = i.to_be_bytes();
                if i >= 0 {
                    Self::from_const_array(b)
                } else {
                    Self::from_const_array_negative(b)
                }
            }
        }
    };
    (IMPL UNSIGNED $ty:ty, $from:ident, $to:ident) => {
        impl_from_to!($ty, $from, $to);

        impl Integer<'_> {
            pub fn $to(&self) -> Result<$ty> {
                let size = std::mem::size_of::<$ty>();
                let bytes = self.as_ref();
                let mut extended = vec![0u8; size];

                if size > bytes.len() {
                    extended[size - bytes.len()..].copy_from_slice(bytes);
                } else {
                    extended.copy_from_slice(&bytes[bytes.len() - size..]);
                };

                Ok(<$ty>::from_be_bytes(extended.try_into().unwrap()))
            }

            pub fn $from(i: $ty) -> Self {
                Self::from_const_array(i.to_be_bytes())
            }
        }
    };
    (SIGNED $ty:ty, $from:ident, $to:ident) => {
        impl_from_to!(IMPL SIGNED $ty, $from, $to);
    };
    (UNSIGNED $ty:ty, $from:ident, $to:ident) => {
        impl_from_to!(IMPL UNSIGNED $ty, $from, $to);
    };
}

impl_from_to!(SIGNED i8, from_i8, as_i8);
impl_from_to!(SIGNED i16, from_i16, as_i16);
impl_from_to!(SIGNED i32, from_i32, as_i32);
impl_from_to!(SIGNED i64, from_i64, as_i64);
impl_from_to!(SIGNED i128, from_i128, as_i128);

impl_from_to!(UNSIGNED u8, from_u8, as_u8);
impl_from_to!(UNSIGNED u16, from_u16, as_u16);
impl_from_to!(UNSIGNED u32, from_u32, as_u32);
impl_from_to!(UNSIGNED u64, from_u64, as_u64);
impl_from_to!(UNSIGNED u128, from_u128, as_u128);

impl AsRef<[u8]> for Integer<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<'a> FromAxdr<'a> for Integer<'a> {
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
        // 判断是否有长度区
        if !is_highest_bit_set(bytes) {
            if bytes.len() < 1 {
                return Err(asn1_rs::Err::Error(Error::InvalidLength));
            }
            Ok((&bytes[1..], bytes[0].into()))
        } else {
            let len = (bytes[0] & 0x7f) as usize;
            if bytes.len() < 1 + len {
                return Err(asn1_rs::Err::Error(Error::InvalidLength));
            }
            Ok((&bytes[1 + len..], Integer::new(&bytes[1..1 + len])))
        }
    }
}

impl ToAxdr for Integer<'_> {
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(if self.need_length() { 1 } else { 0 } + self.as_ref().len())
    }

    fn write_axdr_header(&self, writer: &mut dyn Write) -> SerializeResult<usize> {
        if self.need_length() {
            writer
                .write(&[self.as_ref().len() as u8 | 0b10000000])
                .map_err(Into::into)
        } else {
            Ok(0)
        }
    }

    fn write_axdr_content(&self, writer: &mut dyn Write) -> SerializeResult<usize> {
        writer.write(self.as_ref()).map_err(Into::into)
    }
}

macro_rules! impl_axdr {
    ($ty:ty) => {
        impl<'a> FromAxdr<'a> for $ty {
            fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
                let size = std::mem::size_of::<$ty>();
                if bytes.len() < size {
                    return Err(asn1_rs::Err::Error(Error::InvalidLength));
                }

                Ok((
                    &bytes[size..],
                    <$ty>::from_be_bytes(bytes[0..size].try_into().unwrap()),
                ))
            }
        }

        impl ToAxdr for $ty {
            fn to_axdr_len(&self) -> Result<usize> {
                Ok(std::mem::size_of::<$ty>())
            }

            fn write_axdr_header(&self, _writer: &mut dyn Write) -> SerializeResult<usize> {
                Ok(0)
            }

            fn write_axdr_content(&self, writer: &mut dyn Write) -> SerializeResult<usize> {
                writer.write(&self.to_be_bytes()).map_err(Into::into)
            }
        }
    };
}

impl_axdr!(u8);
impl_axdr!(u16);
impl_axdr!(u32);
impl_axdr!(u64);
impl_axdr!(u128);
impl_axdr!(i8);
impl_axdr!(i16);
impl_axdr!(i32);
impl_axdr!(i64);
impl_axdr!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO: &[u8] = &[0];
    const MINUS_ONE: &[u8] = &[0x81, 0xff];
    const I128_BYTES: &[u8] = &[0x82, 0x00, 0x80];
    const INEG128_BYTES: &[u8] = &[0x81, 0x80];
    const MAX_I8: &[u8] = &[0x7f]; // 127
    const MIN_I8: &[u8] = &[0x81, 0x80]; // -128
    const MAX_U8: &[u8] = &[0x82, 0x00, 0xff]; // 255
    const MAX_I16: &[u8] = &[0x82, 0x7f, 0xff]; // 32767
    const MIN_I16: &[u8] = &[0x82, 0x80, 0x00]; // -32768
    const MAX_U16: &[u8] = &[0x83, 0x00, 0xff, 0xff]; // 65535
    const LARGE_POSITIVE: &[u8] = &[0x83, 0x00, 0x80, 0x00]; // 32768

    #[test]
    fn decode() {
        assert_eq!(0, Integer::from_axdr(ZERO).unwrap().1.as_u8().unwrap());

        assert_eq!(
            -1,
            Integer::from_axdr(MINUS_ONE).unwrap().1.as_i8().unwrap()
        );

        assert_eq!(
            128,
            Integer::from_axdr(I128_BYTES).unwrap().1.as_u16().unwrap()
        );

        assert_eq!(
            -128,
            Integer::from_axdr(INEG128_BYTES)
                .unwrap()
                .1
                .as_i16()
                .unwrap()
        );

        assert_eq!(127, Integer::from_axdr(MAX_I8).unwrap().1.as_i8().unwrap());

        assert_eq!(-128, Integer::from_axdr(MIN_I8).unwrap().1.as_i8().unwrap());

        assert_eq!(255, Integer::from_axdr(MAX_U8).unwrap().1.as_u8().unwrap());

        assert_eq!(
            32767,
            Integer::from_axdr(MAX_I16).unwrap().1.as_i16().unwrap()
        );

        assert_eq!(
            -32768,
            Integer::from_axdr(MIN_I16).unwrap().1.as_i16().unwrap()
        );

        assert_eq!(
            65535,
            Integer::from_axdr(MAX_U16).unwrap().1.as_u16().unwrap()
        );

        assert_eq!(
            32768,
            Integer::from_axdr(LARGE_POSITIVE)
                .unwrap()
                .1
                .as_u32()
                .unwrap()
        );
    }

    #[test]
    fn encode() {
        assert_eq!(ZERO, Integer::from(0u8).to_axdr_vec().unwrap());

        assert_eq!(MINUS_ONE, Integer::from(-1i8).to_axdr_vec().unwrap());

        assert_eq!(I128_BYTES, Integer::from(128u16).to_axdr_vec().unwrap());

        assert_eq!(INEG128_BYTES, Integer::from(-128i16).to_axdr_vec().unwrap());

        assert_eq!(MAX_I8, Integer::from(127i8).to_axdr_vec().unwrap());

        assert_eq!(MIN_I8, Integer::from(-128i8).to_axdr_vec().unwrap());

        assert_eq!(MAX_U8, Integer::from(255u8).to_axdr_vec().unwrap());

        assert_eq!(MAX_I16, Integer::from(32767i16).to_axdr_vec().unwrap());

        assert_eq!(MIN_I16, Integer::from(-32768i16).to_axdr_vec().unwrap());

        assert_eq!(MAX_U16, Integer::from(65535u16).to_axdr_vec().unwrap());

        assert_eq!(
            LARGE_POSITIVE,
            Integer::from(32768u32).to_axdr_vec().unwrap()
        );
    }

    #[test]
    fn encode_i8() {
        assert_eq!(&[0xff], &(-1i8).to_axdr_vec().unwrap()[..]);
    }

    #[test]
    fn decode_i8() {
        assert_eq!(-1, i8::from_axdr(&[0xff]).unwrap().1);
    }
}

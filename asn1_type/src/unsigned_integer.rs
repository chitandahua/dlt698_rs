use super::is_highest_bit_set;
use super::traits::{FromAxdr, ToAxdr};
use crate::{Error, ParseResult, Result, SerializeResult};
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
pub struct UnsignedInteger {
    pub(crate) data: Vec<u8>,
}

impl UnsignedInteger {
    #[inline]
    pub fn new(s: &[u8]) -> Self {
        UnsignedInteger { data: s.to_vec() }
    }

    pub fn from_const_array<const N: usize>(b: [u8; N]) -> Self {
        let mut idx = 0;

        while idx < b.len() - 1 {
            if b[idx] == 0 {
                // 去除前面的0
                idx += 1;
                continue;
            }
            break;
        }

        UnsignedInteger {
            data: b[idx..].to_vec(),
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
        impl From<$ty> for UnsignedInteger {
            fn from(i: $ty) -> Self {
                Self::$from(i)
            }
        }

        impl TryFrom<UnsignedInteger> for $ty {
            type Error = Error;

            fn try_from(value: UnsignedInteger) -> Result<Self> {
                value.$to()
            }
        }

        impl UnsignedInteger {
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
}

impl_from_to!(u8, from_u8, as_u8);
impl_from_to!(u16, from_u16, as_u16);
impl_from_to!(u32, from_u32, as_u32);
impl_from_to!(u64, from_u64, as_u64);
impl_from_to!(u128, from_u128, as_u128);
impl_from_to!(usize, from_usize, as_usize);

impl AsRef<[u8]> for UnsignedInteger {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl FromAxdr<'_> for UnsignedInteger {
    fn from_axdr(bytes: &[u8]) -> ParseResult<'_, Self> {
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
            Ok((&bytes[1 + len..], UnsignedInteger::new(&bytes[1..1 + len])))
        }
    }
}

impl ToAxdr for UnsignedInteger {
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

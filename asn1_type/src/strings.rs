mod str;
mod string;
mod utf8string;
mod visiblestring;

pub use utf8string::*;
pub use visiblestring::*;

use crate::Result;

pub trait TestValidCharset {
    /// Check character set for this object type.
    fn test_valid_charset(i: &[u8]) -> Result<()>;
}

#[macro_export]
macro_rules! asn1_axdr_string {
    (IMPL $name:ident, $sname:expr) => {
        use crate::{Error, ParseResult, Result, SerializeResult};
        use crate::traits::{FromAxdr, ToAxdr};
        use crate::unsigned_integer::UnsignedInteger;

        #[derive(Debug, PartialEq, Eq)]
        pub struct $name<'a> {
            pub(crate) data: std::borrow::Cow<'a, str>,
        }

        impl<'a> $name<'a> {
            pub const fn new(s: &'a str) -> Self {
                $name {
                    data: std::borrow::Cow::Borrowed(s),
                }
            }

            pub fn string(&self) -> String {
                self.data.to_string()
            }
        }

        impl<'a> AsRef<str> for $name<'a> {
            fn as_ref(&self) -> &str {
                &self.data
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            fn from(s: &'a str) -> Self {
                Self::new(s)
            }
        }

        impl From<String> for $name<'_> {
            fn from(s: String) -> Self {
                Self {
                    data: std::borrow::Cow::Owned(s),
                }
            }
        }

        impl<'a> FromAxdr<'a> for $name<'a> {
            fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
                let (bytes, len) = UnsignedInteger::from_axdr(bytes)?;
                let len = len.as_u64()? as usize; // TODO

                if bytes.len() < len {
                    return Err(asn1_rs::Err::Error(Error::InvalidLength));
                }

                <$name>::test_valid_charset(&bytes[0..len])?;

                Ok((&bytes[len..], $name::new(std::str::from_utf8(&bytes[0..len]).map_err(|_| asn1_rs::Err::Error(Error::StringInvalidCharset))?)))
            }
        }

        impl ToAxdr for $name<'_> {
            fn to_axdr_len(&self) -> Result<usize> {
                Ok(
                    UnsignedInteger::from_u64(self.data.as_bytes().len() as u64).to_axdr_len()?
                        + self.data.as_bytes().len(),
                )
            }

            fn write_axdr_header(
                &self,
                writer: &mut dyn std::io::Write,
            ) -> SerializeResult<usize> {
                UnsignedInteger::from_u64(self.data.as_bytes().len() as u64).write_axdr(writer)
            }

            fn write_axdr_content(
                &self,
                writer: &mut dyn std::io::Write,
            ) -> SerializeResult<usize> {
                writer.write(self.data.as_bytes()).map_err(Into::into)
            }
        }
    };
    ($name:ident) => {
        asn1_axdr_string!(IMPL $name, stringify!($name));
    };
}

#[macro_export]
macro_rules! asn1_axdr_fixed_string {
    (IMPL $name:ident, $sname:expr) => {

        #[derive(Debug, PartialEq, Eq)]
        pub struct $name<'a, const N: usize> {
            pub(crate) data: std::borrow::Cow<'a, str>,
        }

        impl<'a, const N: usize> $name<'a, N> {
            const fn new(s: &'a str) -> Self {
                Self {
                    data: std::borrow::Cow::Borrowed(s),
                }
            }

            pub fn string(&self) -> String {
                self.data.to_string()
            }
        }

        impl<'a, const N: usize> AsRef<str> for $name<'a, N> {
            fn as_ref(&self) -> &str {
                &self.data
            }
        }

        impl<'a, const N: usize> TryFrom<&'a str> for $name<'a, N> {
            type Error = asn1_rs::Error;
            fn try_from(s: &'a str) -> Result<Self> {
                if s.len() != N {
                    return Err(asn1_rs::Error::InvalidLength);
                }

                Ok(Self {
                    data: std::borrow::Cow::Borrowed(s),
                })
            }
        }

        impl<const N: usize> TryFrom<String> for $name<'_, N> {
            type Error = asn1_rs::Error;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                if s.len() != N {
                    return Err(asn1_rs::Error::InvalidLength);
                }

                Ok(Self {
                    data: std::borrow::Cow::Owned(s),
                })
            }
        }

        impl<'a, const N: usize> FromAxdr<'a> for $name<'a, N> {
            fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
                if bytes.len() < N {
                    return Err(asn1_rs::Err::Error(Error::InvalidLength));
                }

                <$name<'a, N>>::test_valid_charset(&bytes[0..N])?;

                Ok((&bytes[N..], <$name<'a, N>>::new(std::str::from_utf8(&bytes[0..N]).map_err(|_| asn1_rs::Err::Error(Error::StringInvalidCharset))?)))
            }
        }

        impl<const N: usize> ToAxdr for $name<'_, N> {
            fn to_axdr_len(&self) -> Result<usize> {
                Ok(self.data.as_bytes().len())
            }

            fn write_axdr_header(
                &self,
                _writer: &mut dyn std::io::Write,
            ) -> SerializeResult<usize> {
                Ok(0)
            }

            fn write_axdr_content(
                &self,
                writer: &mut dyn std::io::Write,
            ) -> SerializeResult<usize> {
                writer.write(self.data.as_bytes()).map_err(Into::into)
            }
        }
    };
    ($name:ident) => {
        asn1_axdr_fixed_string!(IMPL $name, stringify!($name));
    };
}

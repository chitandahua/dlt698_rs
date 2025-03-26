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
        pub struct $name {
            pub(crate) data: String,
        }

        impl $name {
            pub fn new(s: &str) -> Self {
                $name {
                    data: s.to_owned(),
                }
            }

            pub fn string(&self) -> String {
                self.data.to_owned()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.data
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self::new(s)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self {
                    data: s,
                }
            }
        }

        impl FromAxdr<'_> for $name {
            fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
                let (bytes, len) = UnsignedInteger::from_axdr(bytes)?;
                let len = len.as_usize()?;

                if bytes.len() < len {
                    return Err(asn1_rs::Err::Error(Error::InvalidLength));
                }

                <$name>::test_valid_charset(&bytes[0..len])?;

                Ok((&bytes[len..], $name::new(std::str::from_utf8(&bytes[0..len]).map_err(|_| asn1_rs::Err::Error(Error::StringInvalidCharset))?)))
            }
        }

        impl ToAxdr for $name {
            fn to_axdr_len(&self) -> Result<usize> {
                Ok(
                    UnsignedInteger::from_usize(self.data.as_bytes().len()).to_axdr_len()?
                        + self.data.as_bytes().len(),
                )
            }

            fn write_axdr_header(
                &self,
                writer: &mut dyn std::io::Write,
            ) -> SerializeResult<usize> {
                UnsignedInteger::from_usize(self.data.as_bytes().len()).write_axdr(writer)
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
        pub struct $name<const N: usize> {
            pub(crate) data: String,
        }

        impl<const N: usize> $name<N> {
            fn new(s: &str) -> Self {
                Self {
                    data: s.to_owned(),
                }
            }

            pub fn string(&self) -> String {
                self.data.to_owned()
            }
        }

        impl<const N: usize> AsRef<str> for $name<N> {
            fn as_ref(&self) -> &str {
                &self.data
            }
        }

        impl<const N: usize> TryFrom<&str> for $name<N> {
            type Error = asn1_rs::Error;
            fn try_from(s: &str) -> Result<Self> {
                if s.len() != N {
                    return Err(asn1_rs::Error::InvalidLength);
                }

                Ok(Self {
                    data: s.to_string(),
                })
            }
        }

        impl<const N: usize> TryFrom<String> for $name<N> {
            type Error = asn1_rs::Error;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                if s.len() != N {
                    return Err(asn1_rs::Error::InvalidLength);
                }

                Ok(Self {
                    data: s,
                })
            }
        }

        impl<const N: usize> FromAxdr<'_> for $name<N> {
            fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
                if bytes.len() < N {
                    return Err(asn1_rs::Err::Error(Error::InvalidLength));
                }

                <$name<N>>::test_valid_charset(&bytes[0..N])?;

                Ok((&bytes[N..], <$name<N>>::new(std::str::from_utf8(&bytes[0..N]).map_err(|_| asn1_rs::Err::Error(Error::StringInvalidCharset))?)))
            }
        }

        impl<const N: usize> ToAxdr for $name<N> {
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

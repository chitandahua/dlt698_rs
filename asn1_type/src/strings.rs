mod str;
mod string;
mod utf8string;
mod visiblestring;

pub use utf8string::*;
//pub use visiblestring::*;

use asn1_rs::Result;

pub trait TestValidCharset {
    /// Check character set for this object type.
    fn test_valid_charset(i: &[u8]) -> Result<()>;
}

#[macro_export]
macro_rules! asn1_axdr_string {
    (IMPL $name:ident, $sname:expr) => {
        use asn1_rs::{Error, ParseResult, Result, SerializeResult};
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
        asn1_axdr_string!(IMPL $name, stringify!($name));
    };
}

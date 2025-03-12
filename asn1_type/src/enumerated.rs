use super::traits::{FromAxdr, ToAxdr};
use crate::{ParseResult, Result, SerializeResult};

#[derive(Debug, PartialEq, Eq)]
pub struct Enumerated(pub u8); // 0-255

impl Enumerated {
    pub const fn new(value: u8) -> Self {
        Enumerated(value)
    }
}

impl FromAxdr<'_> for Enumerated {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        bytes.first().map_or(
            Err(asn1_rs::Err::Error(asn1_rs::Error::InvalidLength)),
            |b| Ok((&bytes[1..], Enumerated::new(*b))),
        )
    }
}

impl ToAxdr for Enumerated {
    fn to_axdr_len(&self) -> Result<usize> {
        self.0.to_axdr_len()
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        self.0.write_axdr_content(writer)
    }
}

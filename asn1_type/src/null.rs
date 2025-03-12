use super::traits::{FromAxdr, ToAxdr};
use asn1_rs::{ParseResult, Result, SerializeResult};

pub type Null = asn1_rs::Null;

impl FromAxdr<'_> for Null {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        Ok((bytes, Self::default()))
    }
}

impl ToAxdr for Null {
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(0)
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }
}
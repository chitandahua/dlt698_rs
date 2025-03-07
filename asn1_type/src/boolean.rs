use super::traits::{FromAxdr, ToAxdr};
use asn1_rs::Boolean;
use asn1_rs::{ParseResult, Result, SerializeResult};

impl FromAxdr<'_> for Boolean {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        bytes.first().map_or(
            Err(asn1_rs::Err::Error(asn1_rs::Error::InvalidLength)),
            |b| Ok((&bytes[1..], Boolean::new(*b))),
        )
    }
}

impl ToAxdr for Boolean {
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(1)
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        let b = if self.value != 0 { 0xff } else { 0x00 };
        writer.write(&[b]).map_err(Into::into)
    }
}

impl FromAxdr<'_> for bool {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        Ok((&bytes[1..], bytes[0] != 0))
    }
}

impl ToAxdr for bool {
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(1)
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        let b = if *self { 0xff } else { 0x00 };
        writer.write(&[b]).map_err(Into::into)
    }
}

use crate::traits::{FromAxdr, ToAxdr};
use crate::unsigned_integer::UnsignedInteger;
use crate::{Error, ParseResult, Result, SerializeResult};

impl<'a> FromAxdr<'a> for &'a str {
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
        // TODO
        //let (bytes, s) = Utf8String::<'a>::from_axdr(bytes)?;
        //Ok((bytes, s.as_ref()))
        //Ok((bytes, s.data.as_ref()))
        let (bytes, int) = UnsignedInteger::from_axdr(bytes)?;
        let len = int.as_usize()?;

        if bytes.len() < len {
            return Err(asn1_rs::Err::Error(Error::InvalidLength));
        }
        let s = std::str::from_utf8(&bytes[0..len])
            .map_err(|_| asn1_rs::Err::Error(Error::StringInvalidCharset))?;

        Ok((&bytes[len..], s))
    }
}

impl ToAxdr for &'_ str {
    fn to_axdr_len(&self) -> Result<usize> {
        UnsignedInteger::from_usize(self.len()).to_axdr_len()
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        UnsignedInteger::from_usize(self.len()).write_axdr(writer)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        writer.write(self.as_bytes()).map_err(Into::into)
    }
}

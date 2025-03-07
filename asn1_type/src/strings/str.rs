use crate::traits::{FromAxdr, ToAxdr};
use crate::unsigned_integer::UnsignedInteger;
use asn1_rs::{Error, ParseResult, Result, SerializeResult};

impl<'a> FromAxdr<'a> for &'a str {
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
        // TODO
        //let (bytes, s) = Utf8String::<'a>::from_axdr(bytes)?;
        //Ok((bytes, s.as_ref()))
        //Ok((bytes, s.data.as_ref()))
        let (bytes, int) = UnsignedInteger::from_axdr(bytes)?;
        let len = int.as_u64()? as usize;

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
        UnsignedInteger::from_u64(self.len() as u64).to_axdr_len()
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        UnsignedInteger::from_u64(self.len() as u64).write_axdr(writer)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        writer.write(self.as_bytes()).map_err(Into::into)
    }
}

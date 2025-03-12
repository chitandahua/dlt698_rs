use crate::{Error, ParseResult, Result, SerializeResult};
use std::io::Write;

//pub trait AxdrTag {
//    fn tag(&self) -> u8;
//}

pub trait FromAxdr<'a, E = Error>: Sized {
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self, E>;
}

pub trait ToAxdr {
    fn to_axdr_len(&self) -> Result<usize>;

    fn to_axdr_vec(&self) -> SerializeResult<Vec<u8>> {
        let mut v = Vec::new();
        let _ = self.write_axdr(&mut v)?;
        Ok(v)
    }

    fn to_axdr_vec_raw(&self) -> SerializeResult<Vec<u8>> {
        let mut v = Vec::new();
        let _ = self.write_axdr_raw(&mut v)?;
        Ok(v)
    }

    fn write_axdr(&self, writer: &mut dyn Write) -> SerializeResult<usize> {
        let sz = self.write_axdr_header(writer)?;
        let sz = sz + self.write_axdr_content(writer)?;
        Ok(sz)
    }

    fn write_axdr_header(&self, writer: &mut dyn Write) -> SerializeResult<usize>;

    fn write_axdr_content(&self, writer: &mut dyn Write) -> SerializeResult<usize>;

    fn write_axdr_raw(&self, writer: &mut dyn Write) -> SerializeResult<usize> {
        self.write_axdr(writer)
    }
}

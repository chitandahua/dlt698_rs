use crate::strings::Utf8String;
use crate::traits::{FromAxdr, ToAxdr};
use crate::unsigned_integer::UnsignedInteger;
use crate::{ParseResult, Result, SerializeResult};

impl FromAxdr<'_> for String {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        let (bytes, s) = Utf8String::from_axdr(bytes)?;
        Ok((bytes, s.string()))
    }
}

impl ToAxdr for String {
    fn to_axdr_len(&self) -> Result<usize> {
        UnsignedInteger::from_u64(self.len() as u64).to_axdr_len().map(|len| len + self.len())
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        UnsignedInteger::from_u64(self.len() as u64).write_axdr(writer)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        writer.write(self.as_ref()).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_string_to_axdr() {
        let s = String::from("test");
        let bytes = s.to_axdr_vec().unwrap();

        assert_eq!(bytes, [0x04, 0x74, 0x65, 0x73, 0x74]);
    }

    #[test]
    fn test_string_from_axdr() {
        let bytes = [0x04, 0x74, 0x65, 0x73, 0x74];
        let s = String::from("test");
        assert_eq!(s, String::from_axdr(&bytes).unwrap().1);
    }
}

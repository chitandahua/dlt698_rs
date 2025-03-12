use super::traits::{FromAxdr, ToAxdr};
use asn1_rs::{ParseResult, Result, SerializeResult};
use super::Boolean;

impl<'a, T> FromAxdr<'a> for Option<T>
where
    T: FromAxdr<'a>,
{
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
        let (bytes, exist) = Boolean::from_axdr(bytes)?;

        if exist.bool() {
            let (bytes, t) = T::from_axdr(bytes)?;
            Ok((bytes, Some(t)))
        } else {
            Ok((bytes, None))
        }
    }
}

impl<T> ToAxdr for Option<T>
where
    T: ToAxdr,
{
    fn to_axdr_len(&self) -> Result<usize> {
        self.as_ref().map(|t| t.to_axdr_len()).unwrap_or(Ok(0)).map(|len| len + 1)
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        self.is_some().write_axdr(writer)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        self.as_ref().map(|t| t.write_axdr(writer)).unwrap_or(Ok(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_optional_to_axdr() {
        let opt: Option<String> = None;
        let bytes = opt.to_axdr_vec().unwrap();
        assert_eq!(bytes, [0x00]);

        let opt: Option<String> = Some("test".to_string());
        let bytes = opt.to_axdr_vec().unwrap();
        assert_eq!(bytes, [0xff, 0x04, 0x74, 0x65, 0x73, 0x74]);
    }

    #[test]
    fn test_optional_from_axdr() {
        let bytes = [0x00];
        let (_, opt) = Option::<String>::from_axdr(&bytes).unwrap();
        assert_eq!(opt, None);

        let bytes = [0xff, 0x04, 0x74, 0x65, 0x73, 0x74];
        let (_, opt) = Option::<String>::from_axdr(&bytes).unwrap();
        assert_eq!(opt, Some("test".to_string()));
    }
}

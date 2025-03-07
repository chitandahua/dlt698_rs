use super::TestValidCharset;
use crate::asn1_axdr_string;

asn1_axdr_string!(Utf8String);

impl TestValidCharset for Utf8String<'_> {
    fn test_valid_charset(i: &[u8]) -> Result<()> {
        let _ = core::str::from_utf8(i)?;
        Ok(())
    }
}

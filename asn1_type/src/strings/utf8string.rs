use super::TestValidCharset;
use crate::{asn1_axdr_fixed_string, asn1_axdr_string};

asn1_axdr_string!(Utf8String);
asn1_axdr_fixed_string!(FixedUtf8String);

impl<const N: usize> From<FixedUtf8String<N>> for Utf8String {
    fn from(value: FixedUtf8String<N>) -> Self {
        Utf8String::from(value.data)
    }
}

fn test_uft8_valid_charset(i: &[u8]) -> Result<()> {
    let _ = core::str::from_utf8(i)?;
    Ok(())
}

impl TestValidCharset for Utf8String {
    fn test_valid_charset(i: &[u8]) -> Result<()> {
        test_uft8_valid_charset(i)
    }
}

impl<const N: usize> TestValidCharset for FixedUtf8String<N> {
    fn test_valid_charset(i: &[u8]) -> Result<()> {
        test_uft8_valid_charset(i)
    }
}

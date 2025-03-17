use super::TestValidCharset;
use crate::{asn1_axdr_fixed_string, asn1_axdr_string};

asn1_axdr_string!(VisibleString);
asn1_axdr_fixed_string!(FixedVisibleString);

fn test_visible_valid_charset(i: &[u8]) -> Result<()> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_visible(b: &u8) -> bool {
        0x20 <= *b && *b <= 0x7f
    }
    if !i.iter().all(is_visible) {
        return Err(Error::StringInvalidCharset);
    }
    Ok(())
}

impl TestValidCharset for VisibleString<'_> {
    fn test_valid_charset(i: &[u8]) -> Result<()> {
        test_visible_valid_charset(i)
    }
}

impl<const N: usize> TestValidCharset for FixedVisibleString<'_, N> {
    fn test_valid_charset(i: &[u8]) -> Result<()> {
        test_visible_valid_charset(i)
    }
}

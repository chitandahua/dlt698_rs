use asn1_type::{DoubleLongUnsigned, OctetString};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

pub type MAC<'a> = OctetString<'a>;

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct SID<'a> {
    id: DoubleLongUnsigned,
    extra_data: OctetString<'a>,
}

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct SIDMAC<'a> {
    sid: SID<'a>,
    mac: MAC<'a>,
}

pub type RN<'a> = OctetString<'a>;

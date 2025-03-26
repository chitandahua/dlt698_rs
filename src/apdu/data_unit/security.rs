use asn1_type::{DoubleLongUnsigned, OctetString};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

pub type MAC = OctetString;

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct SID {
    id: DoubleLongUnsigned,
    extra_data: OctetString,
}

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct SIDMAC {
    sid: SID,
    mac: MAC,
}

pub type RN = OctetString;

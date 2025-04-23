use asn1_type::{DoubleLongUnsigned, OctetString};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct MAC(pub OctetString);

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

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct RN(pub OctetString);

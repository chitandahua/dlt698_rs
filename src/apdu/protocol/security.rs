use crate::apdu::data_unit::{DAR, MAC, RN, SID, SIDMAC};
use asn1_type::OctetString;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// SecurityRequest
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SecurityRequest<'a> {
    application_data_unit: RequestApplicationDataUnit<'a>,
    data_validation_info: RequestDataValidationInfo<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RequestApplicationDataUnit<'a> {
    #[tag(0)]
    Plain(OctetString<'a>),
    #[tag(1)]
    Encrypted(OctetString<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RequestDataValidationInfo<'a> {
    #[tag(0)]
    SidMac(SIDMAC<'a>),
    #[tag(1)]
    Rn(RN<'a>),
    #[tag(2)]
    RnMac(RNMAC<'a>),
    #[tag(3)]
    Sid(SID<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct RNMAC<'a> {
    rn: RN<'a>,
    mac: MAC<'a>,
}

// SecurityResponse
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SecurityResponse<'a> {
    application_data_unit: ResponseApplicationDataUnit<'a>,
    data_validation_info: Option<ResponseDataValidationInfo<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ResponseApplicationDataUnit<'a> {
    #[tag(0)]
    Plain(OctetString<'a>),
    #[tag(1)]
    Encrypted(OctetString<'a>),
    #[tag(2)]
    Dar(DAR),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ResponseDataValidationInfo<'a> {
    #[tag(0)]
    Mac(MAC<'a>),
}

use crate::apdu::data_unit::{DAR, MAC, RN, SID, SIDMAC};
use asn1_type::OctetString;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// SecurityRequest
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SecurityRequest {
    application_data_unit: RequestApplicationDataUnit,
    data_validation_info: RequestDataValidationInfo,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RequestApplicationDataUnit {
    #[tag(0)]
    Plain(OctetString),
    #[tag(1)]
    Encrypted(OctetString),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RequestDataValidationInfo {
    #[tag(0)]
    SidMac(SIDMAC),
    #[tag(1)]
    Rn(RN),
    #[tag(2)]
    RnMac(RNMAC),
    #[tag(3)]
    Sid(SID),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct RNMAC {
    rn: RN,
    mac: MAC,
}

// SecurityResponse
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SecurityResponse {
    application_data_unit: ResponseApplicationDataUnit,
    data_validation_info: Option<ResponseDataValidationInfo>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ResponseApplicationDataUnit {
    #[tag(0)]
    Plain(OctetString),
    #[tag(1)]
    Encrypted(OctetString),
    #[tag(2)]
    Dar(DAR),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ResponseDataValidationInfo {
    #[tag(0)]
    Mac(MAC),
}

use crate::apdu::data_unit::{OAD, PIID};
use asn1_type::{OctetString, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ReportResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportResponseList {
    piid: PIID,
    oads: SequenceOf<OAD>,
}

// ReportResponseRecordList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportResponseRecordList {
    piid: PIID,
    oads: SequenceOf<OAD>,
}

// ReportResponseTransData
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportResponseTransData {
    piid: PIID,
}

// ReportResponseClientService
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportResponseClientService {
    piid: PIID,
    service_data: OctetString,
}

use crate::apdu::data_unit::{OAD, PIID_ACD};
use crate::apdu::protocol::get::{AResultNormal, AResultRecord};
use asn1_type::{OctetString, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ReportNotificationList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportNotificationList<'a> {
    piid_acd: PIID_ACD,
    a_result_normal: SequenceOf<AResultNormal<'a>>,
}

// ReportNotificationRecordList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportNotificationRecordList<'a> {
    piid_acd: PIID_ACD,
    a_result_record: SequenceOf<AResultRecord<'a>>,
}

// ReportNotificationTransData
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportNotificationTransData<'a> {
    piid_acd: PIID_ACD,
    oad: OAD,
    trans_data: SequenceOf<OctetString<'a>>,
}

// ReportRequestClientService
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReportRequestClientService<'a> {
    piid_acd: PIID_ACD,
    service_data: OctetString<'a>,
}

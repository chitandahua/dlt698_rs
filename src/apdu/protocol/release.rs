use crate::apdu::data_unit::{DateTimeS, PIID, PIID_ACD};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReleaseRequest {
    piid: PIID,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum Result {
    #[tag(0)]
    Success,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReleaseResponse {
    piid_acd: PIID_ACD,
    result: Result,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ReleaseNotification {
    piid_acd: PIID_ACD,
    connect_time: DateTimeS,
    server_time: DateTimeS,
}

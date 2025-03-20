use crate::apdu::data_unit::{DateTime, PIID_ACD};
use asn1_type::LongUnsigned;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RequestType {
    #[tag(0)]
    Login,
    #[tag(1)]
    Heartbeat,
    #[tag(2)]
    Logout,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct LinkRequest {
    piid_acd: PIID_ACD,
    request_type: RequestType,
    heartbeat_interval: LongUnsigned,
    request_time: DateTime,
}

impl LinkRequest {
    pub fn new(
        piid_acd: PIID_ACD,
        request_type: RequestType,
        heartbeat_interval: LongUnsigned,
        request_time: DateTime,
    ) -> Self {
        Self {
            piid_acd,
            request_type,
            heartbeat_interval,
            request_time,
        }
    }
}

use crate::apdu::data_unit::{DateTime, PIID};
use asn1_type::FixedBitString;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

type Result = FixedBitString<8, 1>;

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct LinkResponse {
    piid: PIID,
    result: Result,
    request_time: DateTime,
    received_time: DateTime,
    response_time: DateTime,
}

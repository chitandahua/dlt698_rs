use crate::apdu::data_unit::{DateTime, PIID};
use asn1_type::FixedBitString;
use axdr_macro::{AxdrSequence, ToAxdrSequence};
use num_enum::TryFromPrimitive;

type LinkResponseResult = FixedBitString<8, 1>;

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct LinkResponse {
    piid: PIID,
    result: LinkResponseResult,
    request_time: DateTime,
    received_time: DateTime,
    response_time: DateTime,
}

struct ResponseResult {
    _flag: ClockFlag,
    _result: Result,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum ClockFlag {
    NotTrust = 0,
    Trust = 1,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum Result {
    Success = 0,
    AddressRepeat = 1,
    IllegalDevice = 2,
    InsufficientCapacity = 3,
}

impl From<LinkResponseResult> for ResponseResult {
    fn from(value: LinkResponseResult) -> Self {
        Self {
            _flag: (value.as_slice()[0] & 0x01).try_into().unwrap(),
            _result: ((value.as_slice()[0] & 0xd0) >> 5).try_into().unwrap(),
        }
    }
}

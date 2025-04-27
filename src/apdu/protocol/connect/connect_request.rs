use super::{FunctionConformance, ProtocolConformance};
use crate::apdu::data_unit::{ConnectMechanismInfo, PIID};
use asn1_type::{DoubleLongUnsigned, LongUnsigned, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ConnectRequest {
    pub piid: PIID,
    pub version: LongUnsigned,
    pub protocol_conformance: ProtocolConformance,
    pub function_conformance: FunctionConformance,
    pub max_send_size: LongUnsigned,
    pub max_receive_size: LongUnsigned,
    pub max_receive_window_size: Unsigned,
    pub max_apdu_size: LongUnsigned,
    pub timeout: DoubleLongUnsigned,
    pub connect_mechanism_info: ConnectMechanismInfo,
}

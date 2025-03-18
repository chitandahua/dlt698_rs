use super::{FunctionConformance, ProtocolConformance};
use crate::apdu::data_unit::{ConnectMechanismInfo, PIID};
use asn1_type::{DoubleLongUnsigned, LongUnsigned, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ConnectRequest<'a> {
    piid: PIID,
    version: LongUnsigned,
    protocol_conformance: ProtocolConformance,
    function_conformance: FunctionConformance,
    max_send_size: LongUnsigned,
    max_receive_size: LongUnsigned,
    max_receive_window_size: Unsigned,
    max_apdu_size: LongUnsigned,
    timeout: DoubleLongUnsigned,
    connect_mechanism_info: ConnectMechanismInfo<'a>,
}

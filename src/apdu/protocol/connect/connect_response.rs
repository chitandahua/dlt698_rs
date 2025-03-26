use super::{FunctionConformance, ProtocolConformance};
use crate::apdu::data_unit::{ConnectResponseInfo, PIID_ACD};
use asn1_type::{DoubleLongUnsigned, FixedVisibleString, LongUnsigned, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ConnectResponse {
    piid_acd: PIID_ACD,
    factory_version: FactoryVersion,
    protocol_version: LongUnsigned,
    protocol_conformance: ProtocolConformance,
    function_conformance: FunctionConformance,
    max_frame_size: LongUnsigned,
    max_segment_size: LongUnsigned,
    max_segment_window: Unsigned,
    max_apdu_size: LongUnsigned,
    connection_timeout: DoubleLongUnsigned,
    connect_response_info: ConnectResponseInfo,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct FactoryVersion {
    manufacturer_code: FixedVisibleString<4>,
    software_version: FixedVisibleString<4>,
    software_date: FixedVisibleString<6>,
    hardware_version: FixedVisibleString<4>,
    hardware_date: FixedVisibleString<6>,
    manufacturer_extension: FixedVisibleString<8>,
}

use super::{FunctionConformance, ProtocolConformance};
use crate::apdu::data_unit::{ConnectResponseInfo, PIID_ACD};
use asn1_type::{DoubleLongUnsigned, FixedVisibleString, LongUnsigned, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ConnectResponse {
    pub piid_acd: PIID_ACD,
    pub factory_version: FactoryVersion,
    pub protocol_version: LongUnsigned,
    pub protocol_conformance: ProtocolConformance,
    pub function_conformance: FunctionConformance,
    pub max_frame_size: LongUnsigned,
    pub max_segment_size: LongUnsigned,
    pub max_segment_window: Unsigned,
    pub max_apdu_size: LongUnsigned,
    pub connection_timeout: DoubleLongUnsigned,
    pub connect_response_info: ConnectResponseInfo,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct FactoryVersion {
    pub manufacturer_code: FixedVisibleString<4>,
    pub software_version: FixedVisibleString<4>,
    pub software_date: FixedVisibleString<6>,
    pub hardware_version: FixedVisibleString<4>,
    pub hardware_date: FixedVisibleString<6>,
    pub manufacturer_extension: FixedVisibleString<8>,
}

use crate::apdu::data_unit::{DAR, OAD, PIID_ACD, TSA};
use crate::apdu::protocol::action::{
    ActionResponseNormalResult, ActionThenGetResponseNormalResult,
};
use crate::apdu::protocol::get::{AResultNormal, AResultRecord};
use crate::apdu::protocol::set::{SetResponseNormalResult, SetThenGetResponseNormalResult};
use asn1_type::{OctetString, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ProxyGetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetResponseList {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxyGetResponseResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetResponseResult {
    server_addr: TSA,
    results: SequenceOf<AResultNormal>,
}

// ProxyGetResponseRecord
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetResponseRecord {
    piid_acd: PIID_ACD,
    server_addr: TSA,
    a_result_record: AResultRecord,
}

// ProxySetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetResponseList {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxySetResponseResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetResponseResult {
    server_addr: TSA,
    results: SequenceOf<SetResponseNormalResult>,
}

// ProxySetThenGetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetThenGetResponseList {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxySetThenGetResponseResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxySetThenGetResponseResult {
    server_addr: TSA,
    results: SequenceOf<SetThenGetResponseNormalResult>,
}

// ProxyActionResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionResponseList {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxyActionResponseResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionResponseResult {
    server_addr: TSA,
    results: SequenceOf<ActionResponseNormalResult>,
}

// ProxyActionThenGetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionThenGetResponseList {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxyActionThenGetResponseResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionThenGetResponseResult {
    server_addr: TSA,
    results: SequenceOf<ActionThenGetResponseNormalResult>,
}

// ProxyTransCommandResponse
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyTransCommandResponse {
    piid_acd: PIID_ACD,
    oad: OAD,
    trans_result: ProxyTransResult,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum ProxyTransResult {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    Data(OctetString),
}

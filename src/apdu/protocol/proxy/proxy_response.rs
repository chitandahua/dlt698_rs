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
pub struct ProxyGetResponseList<'a> {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxyGetResponseResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetResponseResult<'a> {
    server_addr: TSA<'a>,
    results: SequenceOf<AResultNormal<'a>>,
}

// ProxyGetResponseRecord
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetResponseRecord<'a> {
    piid_acd: PIID_ACD,
    server_addr: TSA<'a>,
    a_result_record: AResultRecord<'a>,
}

// ProxySetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetResponseList<'a> {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxySetResponseResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetResponseResult<'a> {
    server_addr: TSA<'a>,
    results: SequenceOf<SetResponseNormalResult>,
}

// ProxySetThenGetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetThenGetResponseList<'a> {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxySetThenGetResponseResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxySetThenGetResponseResult<'a> {
    server_addr: TSA<'a>,
    results: SequenceOf<SetThenGetResponseNormalResult<'a>>,
}

// ProxyActionResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionResponseList<'a> {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxyActionResponseResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionResponseResult<'a> {
    server_addr: TSA<'a>,
    results: SequenceOf<ActionResponseNormalResult<'a>>,
}

// ProxyActionThenGetResponseList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionThenGetResponseList<'a> {
    piid_acd: PIID_ACD,
    server_results: SequenceOf<ProxyActionThenGetResponseResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionThenGetResponseResult<'a> {
    server_addr: TSA<'a>,
    results: SequenceOf<ActionThenGetResponseNormalResult<'a>>,
}

// ProxyTransCommandResponse
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyTransCommandResponse<'a> {
    piid_acd: PIID_ACD,
    oad: OAD,
    trans_result: ProxyTransResult<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum ProxyTransResult<'a> {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    Data(OctetString<'a>),
}

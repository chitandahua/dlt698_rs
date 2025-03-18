use crate::apdu::data_unit::{Data, DAR, OMD, PIID_ACD};
use crate::apdu::protocol::get::AResultNormal;
use asn1_type::SequenceOf;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ActionResponseNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionResponseNormal<'a> {
    piid_acd: PIID_ACD,
    omd: OMD,
    result: DAR,
    response_data: Option<Data<'a>>,
}

// ActionResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionResponseNormalList<'a> {
    piid_acd: PIID_ACD,
    action_results: SequenceOf<ActionResponseNormalResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ActionResponseNormalResult<'a> {
    omd: OMD,
    result: DAR,
    response_data: Option<Data<'a>>,
}

// ActionThenGetResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionThenGetResponseNormalList<'a> {
    piid_acd: PIID_ACD,
    action_results: SequenceOf<ActionThenGetResponseNormalResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ActionThenGetResponseNormalResult<'a> {
    omd: OMD,
    result: DAR,
    response_data: Option<Data<'a>>,
    a_result_normal: AResultNormal<'a>,
}

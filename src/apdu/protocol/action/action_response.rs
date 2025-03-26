use crate::apdu::data_unit::{Data, DAR, OMD, PIID_ACD};
use crate::apdu::protocol::get::AResultNormal;
use asn1_type::SequenceOf;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ActionResponseNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionResponseNormal {
    piid_acd: PIID_ACD,
    omd: OMD,
    result: DAR,
    response_data: Option<Data>,
}

// ActionResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionResponseNormalList {
    piid_acd: PIID_ACD,
    action_results: SequenceOf<ActionResponseNormalResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionResponseNormalResult {
    omd: OMD,
    result: DAR,
    response_data: Option<Data>,
}

// ActionThenGetResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionThenGetResponseNormalList {
    piid_acd: PIID_ACD,
    action_results: SequenceOf<ActionThenGetResponseNormalResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionThenGetResponseNormalResult {
    omd: OMD,
    result: DAR,
    response_data: Option<Data>,
    a_result_normal: AResultNormal,
}

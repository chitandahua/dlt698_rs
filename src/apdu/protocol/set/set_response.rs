use crate::apdu::data_unit::{DAR, OAD, PIID_ACD};
use crate::apdu::protocol::get::AResultNormal;
use asn1_type::SequenceOf;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// SetResponseNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetResponseNormal {
    pub piid_acd: PIID_ACD,
    pub oad: OAD,
    pub result: DAR,
}

// SetResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetResponseNormalList {
    piid_acd: PIID_ACD,
    result_list: SequenceOf<SetResponseNormalResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetResponseNormalResult {
    oad: OAD,
    result: DAR,
}

// SetThenGetResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetThenGetResponseNormalList {
    piid_acd: PIID_ACD,
    result_list: SequenceOf<SetThenGetResponseNormalResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetThenGetResponseNormalResult {
    oad: OAD,
    result: DAR,
    a_result_normal: AResultNormal,
}

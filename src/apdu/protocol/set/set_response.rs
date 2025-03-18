use crate::apdu::data_unit::{DAR, OAD, PIID_ACD};
use crate::apdu::protocol::get::AResultNormal;
use asn1_type::SequenceOf;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// SetResponseNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetResponseNormal {
    piid_acd: PIID_ACD,
    oad: OAD,
    result: DAR,
}

// SetResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetResponseNormalList {
    piid_acd: PIID_ACD,
    result_list: SequenceOf<SetResponseNormalResult>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct SetResponseNormalResult {
    oad: OAD,
    result: DAR,
}

// SetThenGetResponseNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetThenGetResponseNormalList<'a> {
    piid_acd: PIID_ACD,
    result_list: SequenceOf<SetThenGetResponseNormalResult<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct SetThenGetResponseNormalResult<'a> {
    oad: OAD,
    result: DAR,
    a_result_normal: AResultNormal<'a>,
}

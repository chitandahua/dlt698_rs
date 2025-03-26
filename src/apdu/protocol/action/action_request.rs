use crate::apdu::data_unit::{Data, OAD, OMD, PIID};
use asn1_type::{SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ActionRequest
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionRequest {
    piid: PIID,
    omd: OMD,
    parameter: Data,
}

// ActionRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionRequestList {
    piid: PIID,
    attributes: SequenceOf<ActionRequestAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionRequestAttribute {
    omd: OMD,
    parameter: Data,
}

// ActionThenGetRequestNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionThenGetRequestNormalList {
    piid: PIID,
    attributes: SequenceOf<ActionThenGetRequestNormalAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionThenGetRequestNormalAttribute {
    omd: OMD,
    parameter: Data,
    oad: OAD,
    delay: Unsigned,
}

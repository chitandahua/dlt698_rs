use crate::apdu::data_unit::{Data, OAD, OMD, PIID};
use asn1_type::{SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ActionRequest
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionRequest<'a> {
    piid: PIID,
    omd: OMD,
    parameter: Data<'a>,
}

// ActionRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionRequestList<'a> {
    piid: PIID,
    attributes: SequenceOf<ActionRequestAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ActionRequestAttribute<'a> {
    omd: OMD,
    parameter: Data<'a>,
}

// ActionThenGetRequestNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ActionThenGetRequestNormalList<'a> {
    piid: PIID,
    attributes: SequenceOf<ActionThenGetRequestNormalAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ActionThenGetRequestNormalAttribute<'a> {
    omd: OMD,
    parameter: Data<'a>,
    oad: OAD,
    delay: Unsigned,
}

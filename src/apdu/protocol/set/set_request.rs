use crate::apdu::data_unit::{Data, OAD, PIID};
use asn1_type::{SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// SetRequestNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetRequestNormal {
    piid: PIID,
    oad: OAD,
    data: Data,
}

// SetRequestNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetRequestNormalList {
    piid: PIID,
    attributes: SequenceOf<SetRequestNormalAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetRequestNormalAttribute {
    oad: OAD,
    data: Data,
}

// SetThenGetRequestNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetThenGetRequestNormalList {
    piid: PIID,
    attributes: SequenceOf<SetThenGetRequestNormalAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetThenGetRequestNormalAttribute {
    oad: OAD,
    data: Data,
    read_oad: OAD,
    delay: Unsigned,
}

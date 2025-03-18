use crate::apdu::data_unit::{Data, OAD, PIID};
use asn1_type::{SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// SetRequestNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetRequestNormal<'a> {
    piid: PIID,
    oad: OAD,
    data: Data<'a>,
}

// SetRequestNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetRequestNormalList<'a> {
    piid: PIID,
    attributes: SequenceOf<SetRequestNormalAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct SetRequestNormalAttribute<'a> {
    oad: OAD,
    data: Data<'a>,
}

// SetThenGetRequestNormalList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct SetThenGetRequestNormalList<'a> {
    piid: PIID,
    attributes: SequenceOf<SetThenGetRequestNormalAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct SetThenGetRequestNormalAttribute<'a> {
    oad: OAD,
    data: Data<'a>,
    read_oad: OAD,
    delay: Unsigned,
}

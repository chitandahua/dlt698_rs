use crate::apdu::data_unit::{OAD, PIID, RCSD, RSD};
use asn1_type::{LongUnsigned, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetRequestNormal {
    piid: PIID,
    oad: OAD,
}

impl GetRequestNormal {
    pub fn new(piid: PIID, oad: OAD) -> Self {
        Self { piid, oad }
    }
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetRequestNormalList {
    piid: PIID,
    oad_list: SequenceOf<OAD>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetRequestRecord<'a> {
    piid: PIID,
    get_record: GetRecord<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct GetRecord<'a> {
    oad: OAD,
    rsd: RSD<'a>,
    rcsd: RCSD,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetRequestRecordList<'a> {
    piid: PIID,
    get_record_list: SequenceOf<GetRecord<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetRequestNext {
    piid: PIID,
    last_block_number: LongUnsigned,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetRequestMD5 {
    piid: PIID,
    oad: OAD,
}

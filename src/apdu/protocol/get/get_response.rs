use crate::apdu::data_unit::{Data, DAR, OAD, PIID_ACD, RCSD};
use asn1_type::{Boolean, LongUnsigned, OctetString, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// GetResponseNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseNormal<'a> {
    piid_acd: PIID_ACD,
    a_result_normal: SequenceOf<AResultNormal<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct AResultNormal<'a> {
    oad: OAD,
    get_result: GetResult<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum GetResult<'a> {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    Data(Data<'a>),
}

// GetResponseNormalList∷
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseNormalList<'a> {
    piid_acd: PIID_ACD,
    a_result_normal: SequenceOf<AResultNormal<'a>>,
}

// GetResponseRecord
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseRecord<'a> {
    piid_acd: PIID_ACD,
    a_result_record: SequenceOf<AResultRecord<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct AResultRecord<'a> {
    oad: OAD,
    rcsd: RCSD,
    record_result: RecordResult<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RecordResult<'a> {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    RecordRows(ARecordRow<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ARecordRow<'a> {
    data: SequenceOf<Data<'a>>,
}

// GetResponseRecordList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseRecordList<'a> {
    piid_acd: PIID_ACD,
    a_result_record: SequenceOf<AResultRecord<'a>>,
}

// GetResponseNext
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseNext<'a> {
    piid_acd: PIID_ACD,
    last_fragment: Boolean,
    fragment_number: LongUnsigned,
    fragment_response: FragmentResponse<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum FragmentResponse<'a> {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    AResultNormal(SequenceOf<AResultNormal<'a>>),
    #[tag(2)]
    AResultRecord(SequenceOf<AResultRecord<'a>>),
}

// GetResponseMD5
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseMD5<'a> {
    piid_acd: PIID_ACD,
    oad: OAD,
    result: ResultMD5<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ResultMD5<'a> {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    MD5(OctetString<'a>),
}

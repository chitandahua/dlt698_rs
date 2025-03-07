use asn1_type::{LongUnsigned, Unsigned};
use asn1_rs::SequenceOf;

pub type OI = LongUnsigned;

#[derive(Debug, AxdrSequence, ToAxdrSequence)]
pub struct OAD {
    pub object_identifier: OI,
    pub attribute: Unsigned,
    pub index: Unsigned
}

#[derive(Debug, AxdrSequence, ToAxdrSequence)]
pub struct ROAD {
    pub oad: OAD,
    pub relate_oads: SequenceOf<OAD>
}




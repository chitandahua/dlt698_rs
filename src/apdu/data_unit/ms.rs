use super::{Region, TSA};
use asn1_type::{LongUnsigned, SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum MS {
    #[tag(0)]
    NoMeter,
    #[tag(1)]
    AllMeter,
    #[tag(2)]
    UserTypes(SequenceOf<Unsigned>),
    #[tag(3)]
    UserAddrs(SequenceOf<TSA>),
    #[tag(4)]
    ConfigSeqs(SequenceOf<LongUnsigned>),
    #[tag(5)]
    UserTypeRegions(SequenceOf<Region>),
    #[tag(6)]
    UserAddrRegions(SequenceOf<Region>),
    #[tag(7)]
    ConfigSeqRegions(SequenceOf<Region>),
}

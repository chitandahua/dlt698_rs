mod get_request;
pub use get_request::*;

mod get_response;
pub use get_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum GetRequest<'a> {
    #[tag(1)]
    GetRequestNormal(GetRequestNormal),
    #[tag(2)]
    GetRequestNormalList(GetRequestNormalList),
    #[tag(3)]
    GetRequestRecord(GetRequestRecord<'a>),
    #[tag(4)]
    GetRequestRecordList(GetRequestRecordList<'a>),
    #[tag(5)]
    GetRequestNext(GetRequestNext),
    #[tag(6)]
    GetRequestMD5(GetRequestMD5),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum GetResponse<'a> {
    #[tag(1)]
    GetResponseNormal(GetResponseNormal<'a>),
    #[tag(2)]
    GetResponseNormalList(GetResponseNormalList<'a>),
    #[tag(3)]
    GetResponseRecord(GetResponseRecord<'a>),
    #[tag(4)]
    GetResponseRecordList(GetResponseRecordList<'a>),
    #[tag(5)]
    GetResponseNext(GetResponseNext<'a>),
    #[tag(6)]
    GetResponseMD5(GetResponseMD5<'a>),
}
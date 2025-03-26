mod get_request;
pub use get_request::*;

mod get_response;
pub use get_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum GetRequest {
    #[tag(1)]
    GetRequestNormal(GetRequestNormal),
    #[tag(2)]
    GetRequestNormalList(GetRequestNormalList),
    #[tag(3)]
    GetRequestRecord(GetRequestRecord),
    #[tag(4)]
    GetRequestRecordList(GetRequestRecordList),
    #[tag(5)]
    GetRequestNext(GetRequestNext),
    #[tag(6)]
    GetRequestMD5(GetRequestMD5),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum GetResponse {
    #[tag(1)]
    GetResponseNormal(GetResponseNormal),
    #[tag(2)]
    GetResponseNormalList(GetResponseNormalList),
    #[tag(3)]
    GetResponseRecord(GetResponseRecord),
    #[tag(4)]
    GetResponseRecordList(GetResponseRecordList),
    #[tag(5)]
    GetResponseNext(GetResponseNext),
    #[tag(6)]
    GetResponseMD5(GetResponseMD5),
}

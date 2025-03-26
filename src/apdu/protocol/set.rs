mod set_request;
pub use set_request::*;

mod set_response;
pub use set_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum SetRequest {
    #[tag(1)]
    SetRequestNormal(SetRequestNormal),
    #[tag(2)]
    SetRequestNormalList(SetRequestNormalList),
    #[tag(3)]
    SetThenGetRequestNormalList(SetThenGetRequestNormalList),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum SetResponse {
    #[tag(1)]
    SetResponseNormal(SetResponseNormal),
    #[tag(2)]
    SetResponseNormalList(SetResponseNormalList),
    #[tag(3)]
    SetThenGetResponseNormalList(SetThenGetResponseNormalList),
}

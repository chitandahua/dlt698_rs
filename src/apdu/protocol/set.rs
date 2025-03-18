mod set_request;
pub use set_request::*;

mod set_response;
pub use set_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum SetRequest<'a> {
    #[tag(1)]
    SetRequestNormal(SetRequestNormal<'a>),
    #[tag(2)]
    SetRequestNormalList(SetRequestNormalList<'a>),
    #[tag(3)]
    SetThenGetRequestNormalList(SetThenGetRequestNormalList<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum SetResponse<'a> {
    #[tag(1)]
    SetResponseNormal(SetResponseNormal),
    #[tag(2)]
    SetResponseNormalList(SetResponseNormalList),
    #[tag(3)]
    SetThenGetResponseNormalList(SetThenGetResponseNormalList<'a>),
}
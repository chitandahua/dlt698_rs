mod action_request;
pub use action_request::*;

mod action_response;
pub use action_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ActionRequestType<'a> {
    #[tag(1)]
    ActionRequest(ActionRequest<'a>),
    #[tag(2)]
    ActionRequestList(ActionRequestList<'a>),
    #[tag(3)]
    ActionThenGetRequestNormalList(ActionThenGetRequestNormalList<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ActionResponse<'a> {
    #[tag(1)]
    ActionResponseNormal(ActionResponseNormal<'a>),
    #[tag(2)]
    ActionResponseNormalList(ActionResponseNormalList<'a>),
    #[tag(3)]
    ActionThenGetResponseNormalList(ActionThenGetResponseNormalList<'a>),
}
mod action_request;
pub use action_request::*;

mod action_response;
pub use action_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ActionRequestType {
    #[tag(1)]
    ActionRequest(ActionRequest),
    #[tag(2)]
    ActionRequestList(ActionRequestList),
    #[tag(3)]
    ActionThenGetRequestNormalList(ActionThenGetRequestNormalList),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ActionResponse {
    #[tag(1)]
    ActionResponseNormal(ActionResponseNormal),
    #[tag(2)]
    ActionResponseNormalList(ActionResponseNormalList),
    #[tag(3)]
    ActionThenGetResponseNormalList(ActionThenGetResponseNormalList),
}

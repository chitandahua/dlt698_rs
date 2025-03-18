use axdr_macro::{AxdrSequence, ToAxdrSequence};

mod link_request;
pub use link_request::*;

mod link_response;
pub use link_response::*;

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum LinkApdu {
    #[tag(0)]
    LinkRequest(LinkRequest),
    #[tag(129)]
    LinkResponse(LinkResponse),
}

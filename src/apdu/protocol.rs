mod action;
mod connect;
mod get;
mod link;
mod release;
mod report;
mod set;

use crate::apdu::protocol::link::{LinkRequest, LinkResponse};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum LinkApdu {
    #[tag(0)]
    LinkRequest(LinkRequest),
    #[tag(129)]
    LinkResponse(LinkResponse),
}

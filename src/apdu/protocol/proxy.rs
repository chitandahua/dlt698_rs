mod proxy_request;
pub use proxy_request::*;

mod proxy_response;
pub use proxy_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ProxyRequest {
    #[tag(1)]
    ProxyGetRequestList(ProxyGetRequestList),
    #[tag(2)]
    ProxyGetRequestRecord(ProxyGetRequestRecord),
    #[tag(3)]
    ProxySetRequestList(ProxySetRequestList),
    #[tag(4)]
    ProxySetThenGetRequestList(ProxySetThenGetRequestList),
    #[tag(5)]
    ProxyActionRequestList(ProxyActionRequestList),
    #[tag(6)]
    ProxyActionThenGetRequestList(ProxyActionThenGetRequestList),
    #[tag(7)]
    ProxyTransCommandRequest(ProxyTransCommandRequest),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ProxyResponse {
    #[tag(1)]
    ProxyGetResponseList(ProxyGetResponseList),
    #[tag(2)]
    ProxyGetResponseRecord(ProxyGetResponseRecord),
    #[tag(3)]
    ProxySetResponseList(ProxySetResponseList),
    #[tag(4)]
    ProxySetThenGetResponseList(ProxySetThenGetResponseList),
    #[tag(5)]
    ProxyActionResponseList(ProxyActionResponseList),
    #[tag(6)]
    ProxyActionThenGetResponseList(ProxyActionThenGetResponseList),
    #[tag(7)]
    ProxyTransCommandResponse(ProxyTransCommandResponse),
}

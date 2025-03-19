mod proxy_request;
pub use proxy_request::*;

mod proxy_response;
pub use proxy_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ProxyRequest<'a> {
    #[tag(1)]
    ProxyGetRequestList(ProxyGetRequestList<'a>),
    #[tag(2)]
    ProxyGetRequestRecord(ProxyGetRequestRecord<'a>),
    #[tag(3)]
    ProxySetRequestList(ProxySetRequestList<'a>),
    #[tag(4)]
    ProxySetThenGetRequestList(ProxySetThenGetRequestList<'a>),
    #[tag(5)]
    ProxyActionRequestList(ProxyActionRequestList<'a>),
    #[tag(6)]
    ProxyActionThenGetRequestList(ProxyActionThenGetRequestList<'a>),
    #[tag(7)]
    ProxyTransCommandRequest(ProxyTransCommandRequest<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ProxyResponse<'a> {
    #[tag(1)]
    ProxyGetResponseList(ProxyGetResponseList<'a>),
    #[tag(2)]
    ProxyGetResponseRecord(ProxyGetResponseRecord<'a>),
    #[tag(3)]
    ProxySetResponseList(ProxySetResponseList<'a>),
    #[tag(4)]
    ProxySetThenGetResponseList(ProxySetThenGetResponseList<'a>),
    #[tag(5)]
    ProxyActionResponseList(ProxyActionResponseList<'a>),
    #[tag(6)]
    ProxyActionThenGetResponseList(ProxyActionThenGetResponseList<'a>),
    #[tag(7)]
    ProxyTransCommandResponse(ProxyTransCommandResponse<'a>),
}

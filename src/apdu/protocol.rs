mod action;
pub use action::{ActionRequest, ActionResponse};

mod connect;
pub use connect::{ConnectRequest, ConnectResponse};

mod get;
pub use get::{AResultNormal, AResultRecord, GetRequest, GetRequestNormal, GetResponse};

mod link;
pub use link::{LinkRequest, LinkResponse, RequestType};

mod proxy;
pub use proxy::{ProxyRequest, ProxyResponse};

mod release;
pub use release::{ReleaseNotification, ReleaseRequest, ReleaseResponse};

mod report;
pub use report::{ReportNotification, ReportResponse};

mod security;
pub use security::{SecurityRequest, SecurityResponse};

mod set;
pub use set::{SetRequest, SetResponse};

use crate::apdu::data_unit::{DateTimeS, TI};
use asn1_type::SequenceOf;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum LinkApdu {
    #[tag(1)]
    LinkRequest(LinkRequest),
    #[tag(129)]
    LinkResponse(LinkResponse),
}

// Client-APDU
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ClientApdu<'a> {
    application_service: ClientApplicationService<'a>,
    time_tag: Option<TimeTag>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ClientApplicationService<'a> {
    #[tag(2)]
    ConnectRequest(ConnectRequest<'a>),
    #[tag(3)]
    ReleaseRequest(ReleaseRequest),
    #[tag(5)]
    GetRequest(GetRequest<'a>),
    #[tag(6)]
    SetRequest(SetRequest<'a>),
    #[tag(7)]
    ActionRequest(ActionRequest<'a>),
    #[tag(8)]
    ReportResponse(ReportResponse<'a>),
    #[tag(9)]
    ProxyRequest(ProxyRequest<'a>),
}

impl<'a> ClientApdu<'a> {
    pub fn new(
        application_service: ClientApplicationService<'a>,
        time_tag: Option<TimeTag>,
    ) -> Self {
        Self {
            application_service,
            time_tag,
        }
    }
}

// Server-APDU
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ServerApdu<'a> {
    application_service: ServerApplicationService<'a>,
    follow_report: Option<FollowReport<'a>>,
    time_tag: Option<TimeTag>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ServerApplicationService<'a> {
    #[tag(130)]
    ConnectResponse(ConnectResponse<'a>),
    #[tag(131)]
    ReleaseResponse(ReleaseResponse),
    #[tag(132)]
    ReleaseNotification(ReleaseNotification),
    #[tag(133)]
    GetResponse(GetResponse<'a>),
    #[tag(134)]
    SetResponse(SetResponse<'a>),
    #[tag(135)]
    ActionResponse(ActionResponse<'a>),
    #[tag(136)]
    ReportNotification(ReportNotification<'a>),
    #[tag(137)]
    ProxyResponse(ProxyResponse<'a>),
}

// Security-APDU
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum SecurityApdu<'a> {
    #[tag(16)]
    SecurityRequest(SecurityRequest<'a>),
    #[tag(144)]
    SecurityResponse(SecurityResponse<'a>),
}

// FollowReport
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum FollowReport<'a> {
    #[tag(1)]
    ResultNormalList(SequenceOf<AResultNormal<'a>>),
    #[tag(2)]
    ResultRecordList(SequenceOf<AResultRecord<'a>>),
}

// TimeTag
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct TimeTag {
    send_time: DateTimeS,
    permission_delay: TI,
}

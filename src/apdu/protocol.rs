mod action;
pub use action::{ActionRequest, ActionResponse};

mod connect;
pub use connect::{ConnectRequest, ConnectResponse};

mod get;
pub use get::{
    AResultNormal, AResultRecord, GetRequest, GetRequestNormal, GetResponse, GetResponseNormal,
    GetResult,
};

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
pub struct ClientApdu {
    pub application_service: ClientApplicationService,
    pub time_tag: Option<TimeTag>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ClientApplicationService {
    #[tag(2)]
    ConnectRequest(ConnectRequest),
    #[tag(3)]
    ReleaseRequest(ReleaseRequest),
    #[tag(5)]
    GetRequest(GetRequest),
    #[tag(6)]
    SetRequest(SetRequest),
    #[tag(7)]
    ActionRequest(ActionRequest),
    #[tag(8)]
    ReportResponse(ReportResponse),
    #[tag(9)]
    ProxyRequest(ProxyRequest),
}

impl ClientApdu {
    pub fn new(application_service: ClientApplicationService, time_tag: Option<TimeTag>) -> Self {
        Self {
            application_service,
            time_tag,
        }
    }
}

// Server-APDU
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ServerApdu {
    pub application_service: ServerApplicationService,
    pub follow_report: Option<FollowReport>,
    pub time_tag: Option<TimeTag>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ServerApplicationService {
    #[tag(130)]
    ConnectResponse(ConnectResponse),
    #[tag(131)]
    ReleaseResponse(ReleaseResponse),
    #[tag(132)]
    ReleaseNotification(ReleaseNotification),
    #[tag(133)]
    GetResponse(GetResponse),
    #[tag(134)]
    SetResponse(SetResponse),
    #[tag(135)]
    ActionResponse(ActionResponse),
    #[tag(136)]
    ReportNotification(ReportNotification),
    #[tag(137)]
    ProxyResponse(ProxyResponse),
}

// Security-APDU
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum SecurityApdu {
    #[tag(16)]
    SecurityRequest(SecurityRequest),
    #[tag(144)]
    SecurityResponse(SecurityResponse),
}

// FollowReport
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum FollowReport {
    #[tag(1)]
    ResultNormalList(SequenceOf<AResultNormal>),
    #[tag(2)]
    ResultRecordList(SequenceOf<AResultRecord>),
}

// TimeTag
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct TimeTag {
    send_time: DateTimeS,
    permission_delay: TI,
}

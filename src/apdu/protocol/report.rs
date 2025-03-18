mod report_notification;
pub use report_notification::*;

mod report_response;
pub use report_response::*;

use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ReportNotification<'a> {
    #[tag(1)]
    ReportNotificationList(ReportNotificationList<'a>),
    #[tag(2)]
    ReportNotificationRecordList(ReportNotificationRecordList<'a>),
    #[tag(3)]
    ReportNotificationTransData(ReportNotificationTransData<'a>),
    #[tag(4)]
    ReportRequestClientService(ReportRequestClientService<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ReportResponse<'a> {
    #[tag(1)]
    ReportResponseList(ReportResponseList),
    #[tag(2)]
    ReportResponseRecordList(ReportResponseRecordList),
    #[tag(3)]
    ReportResponseTransData(ReportResponseTransData),
    #[tag(4)]
    ReportResponseClientService(ReportResponseClientService<'a>),
}

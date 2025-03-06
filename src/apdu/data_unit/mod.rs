

use crate::apdu::data_type::{LongUnsigned, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, AxdrSequence, ToAxdrSequence)]
pub struct Date {
    year: LongUnsigned,
    month: Unsigned,
    day_of_month: Unsigned,
    day_of_week: Unsigned,
    hour: Unsigned,
    minute: Unsigned,
    second: Unsigned,
    milliseconds: LongUnsigned
}
use asn1_type::{LongUnsigned, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct DateTime {
    year: LongUnsigned,
    month: Unsigned,
    day_of_month: Unsigned,
    day_of_week: Unsigned,
    hour: Unsigned,
    minute: Unsigned,
    second: Unsigned,
    milliseconds: LongUnsigned,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Date {
    year: LongUnsigned,
    month: Unsigned,
    day_of_month: Unsigned,
    day_of_week: Unsigned,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Time {
    hour: Unsigned,
    minute: Unsigned,
    second: Unsigned,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct DateTimeS {
    year: LongUnsigned,
    month: Unsigned,
    day: Unsigned,
    hour: Unsigned,
    minute: Unsigned,
    second: Unsigned,
}

//impl From<NaiveDateTime> for DateTime {
//    fn from(value: NaiveDateTime) -> Self {
//        Self {
//            year: value.year(),
//            month: value.month(),
//            day_of_month: value.day(),
//            day_of_week: value.weekday().number_from_monday(),
//            hour: value.hour(),
//            minute: value.minute(),
//            second: value.second(),
//        }
//    }
//}

impl DateTimeS {
    pub fn new(
        year: LongUnsigned,
        month: Unsigned,
        day: Unsigned,
        hour: Unsigned,
        minute: Unsigned,
        second: Unsigned,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

mod tests {
    use super::*;
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_date_time_to_axdr() {
        let date = DateTime {
            year: 2022,
            month: 1,
            day_of_month: 1,
            day_of_week: 1,
            hour: 0x06,
            minute: 0x21,
            second: 0x10,
            milliseconds: 0x1234,
        };
        let axdr = date.to_axdr_vec().unwrap();
        assert_eq!(
            axdr,
            [0x07, 0xe6, 0x01, 0x01, 0x01, 0x06, 0x21, 0x10, 0x12, 0x34]
        );
    }

    #[test]
    fn test_date_time_from_axdr() {
        let axdr = [0x07, 0xe6, 0x01, 0x01, 0x01, 0x06, 0x21, 0x10, 0x12, 0x34];
        let (_, date) = DateTime::from_axdr(&axdr).unwrap();
        assert_eq!(date.year, 2022);
        assert_eq!(date.month, 1);
        assert_eq!(date.day_of_month, 1);
        assert_eq!(date.day_of_week, 1);
        assert_eq!(date.hour, 0x06);
        assert_eq!(date.minute, 0x21);
        assert_eq!(date.second, 0x10);
        assert_eq!(date.milliseconds, 0x1234);
    }

    #[test]
    fn test_date_to_axdr() {
        let date = Date {
            year: 2022,
            month: 1,
            day_of_month: 1,
            day_of_week: 1,
        };
        let axdr = date.to_axdr_vec().unwrap();
        assert_eq!(axdr, [0x07, 0xe6, 0x01, 0x01, 0x01]);
    }

    #[test]
    fn test_date_from_axdr() {
        let axdr = [0x07, 0xe6, 0x01, 0x01, 0x01];
        let (_, date) = Date::from_axdr(&axdr).unwrap();
        assert_eq!(date.year, 2022);
        assert_eq!(date.month, 1);
        assert_eq!(date.day_of_month, 1);
        assert_eq!(date.day_of_week, 1);
    }
}

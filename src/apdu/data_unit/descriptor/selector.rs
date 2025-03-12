use super::attribute::{OAD, ROAD};
use crate::apdu::data_unit::{Data, DateTimeS, TiUnit, MS, TI};
use asn1_type::{Null, SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum CSD {
    #[tag(0)]
    Oad(OAD),
    #[tag(1)]
    Road(ROAD),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RSD<'a> {
    #[tag(0)]
    Null(Null),
    #[tag(1)]
    Selector1(Selector1<'a>),
    #[tag(2)]
    Selector2(Selector2<'a>),
    #[tag(3)]
    Selector3(Selector3<'a>),
    #[tag(4)]
    Selector4(Selector4<'a>),
    #[tag(5)]
    Selector5(Selector5<'a>),
    #[tag(6)]
    Selector6(Selector6<'a>),
    #[tag(7)]
    Selector7(Selector7<'a>),
    #[tag(8)]
    Selector8(Selector8<'a>),
    #[tag(9)]
    Selector9(Selector9),
    #[tag(10)]
    Selector10(Selector10<'a>),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector1<'a> {
    oad: OAD,
    value: Box<Data<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector2<'a> {
    oad: OAD,
    start: Box<Data<'a>>,
    end: Box<Data<'a>>,
    interval: Box<Data<'a>>,
}

pub type Selector3<'a> = SequenceOf<Selector2<'a>>;

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector4<'a> {
    launch_time: DateTimeS,
    ms: MS<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector5<'a> {
    save_time: DateTimeS,
    ms: MS<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector6<'a> {
    launch_time_start: DateTimeS,
    launch_time_end: DateTimeS,
    interval: TI,
    ms: MS<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector7<'a> {
    save_time_start: DateTimeS,
    save_time_end: DateTimeS,
    interval: TI,
    ms: MS<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector8<'a> {
    success_time_start: DateTimeS,
    success_time_end: DateTimeS,
    interval: TI,
    ms: MS<'a>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector9 {
    last_nth_record: Unsigned,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Selector10<'a> {
    last_n_record: Unsigned,
    ms: MS<'a>,
}

pub type RCSD = SequenceOf<CSD>;

mod tests {
    use super::*;
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_csd_to_axdr() {
        let oad = OAD::new(0x5002, 0x02, 0);
        let csd = CSD::Oad(oad);

        let axdr = csd.to_axdr_vec().unwrap();

        assert_eq!(axdr, [0x00, 0x50, 0x02, 0x02, 0x00]);

        let csd = CSD::Road(ROAD::new(
            OAD::new(0x5002, 0x02, 0),
            vec![
                OAD::new(0x2021, 0x02, 0),
                OAD::new(0x0010, 0x02, 0),
                OAD::new(0x0020, 0x02, 0),
            ],
        ));
        let axdr = csd.to_axdr_vec().unwrap();
        assert_eq!(
            axdr,
            [
                0x01, 0x50, 0x02, 0x02, 0x00, 0x03, 0x20, 0x21, 0x02, 0x00, 0x00, 0x10, 0x02, 0x00,
                0x00, 0x20, 0x02, 0x00
            ]
        );
    }

    #[test]
    fn test_csd_from_axdr() {
        let axdr = [
            0x01, 0x50, 0x02, 0x02, 0x00, 0x03, 0x20, 0x21, 0x02, 0x00, 0x00, 0x10, 0x02, 0x00,
            0x00, 0x20, 0x02, 0x00,
        ];
        let (_byte, csd) = CSD::from_axdr(&axdr).unwrap();
        assert_eq!(
            csd,
            CSD::Road(ROAD::new(
                OAD::new(0x5002, 0x02, 0),
                vec![
                    OAD::new(0x2021, 0x02, 0),
                    OAD::new(0x0010, 0x02, 0),
                    OAD::new(0x0020, 0x02, 0),
                ],
            ))
        );

        let axdr = [0x00, 0x50, 0x02, 0x02, 0x00];
        let (_byte, csd) = CSD::from_axdr(&axdr).unwrap();
        assert_eq!(csd, CSD::Oad(OAD::new(0x5002, 0x02, 0)));
    }

    #[test]
    fn test_rsd_to_axdr() {
        let rsd = RSD::Selector5(Selector5 {
            save_time: DateTimeS::new(2023, 5, 1, 0, 0, 0),
            ms: MS::AllMeter,
        });

        let axdr = rsd.to_axdr_vec().unwrap();
        assert_eq!(axdr, [0x05, 0x07, 0xe7, 0x05, 0x01, 0x00, 0x00, 0x00, 0x01]);

        let rsd = RSD::Selector7(Selector7 {
            save_time_start: DateTimeS::new(2023, 3, 3, 0, 0, 0),
            save_time_end: DateTimeS::new(2023, 3, 3, 23, 59, 59),
            interval: TI::new(TiUnit::Second, 0),
            ms: MS::AllMeter,
        });

        let axdr = rsd.to_axdr_vec().unwrap();
        assert_eq!(
            axdr,
            [
                0x07, 0x07, 0xe7, 0x03, 0x03, 0x00, 0x00, 0x00, 0x07, 0xe7, 0x03, 0x03, 0x17, 0x3b,
                0x3b, 0x00, 0x00, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn test_rsd_from_axdr() {
        let axdr = [0x05, 0x07, 0xe7, 0x05, 0x01, 0x00, 0x00, 0x00, 0x01];
        let (_byte, rsd) = RSD::from_axdr(&axdr).unwrap();

        match rsd {
            RSD::Selector5(selector5) => {
                assert_eq!(selector5.save_time, DateTimeS::new(2023, 5, 1, 0, 0, 0));
                assert_eq!(selector5.ms, MS::AllMeter);
            }
            _ => panic!("axdr sequence failed"),
        }

        let axdr = [
            0x07, 0x07, 0xe7, 0x03, 0x03, 0x00, 0x00, 0x00, 0x07, 0xe7, 0x03, 0x03, 0x17, 0x3b,
            0x3b, 0x00, 0x00, 0x00, 0x01,
        ];
        let (_byte, rsd) = RSD::from_axdr(&axdr).unwrap();
        match rsd {
            RSD::Selector7(selector7) => {
                assert_eq!(
                    selector7.save_time_start,
                    DateTimeS::new(2023, 3, 3, 0, 0, 0)
                );
                assert_eq!(
                    selector7.save_time_end,
                    DateTimeS::new(2023, 3, 3, 23, 59, 59)
                );
                assert_eq!(selector7.interval, TI::new(TiUnit::Second, 0));
                assert_eq!(selector7.ms, MS::AllMeter);
            }
            _ => panic!("axdr sequence failed"),
        }
    }
}

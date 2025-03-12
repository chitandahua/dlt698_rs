use super::Data;
use asn1_type::LongUnsigned;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum TiUnit {
    #[tag(0)]
    Second,
    #[tag(1)]
    Minute,
    #[tag(2)]
    Hour,
    #[tag(3)]
    Day,
    #[tag(4)]
    Month,
    #[tag(5)]
    Year,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct TI {
    unit: TiUnit,
    interval: LongUnsigned,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RegionType {
    #[tag(0)]
    CloseOpen,
    #[tag(1)]
    OpenClose,
    #[tag(2)]
    CloseClose,
    #[tag(3)]
    OpenOpen,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct Region<'a> {
    region_type: RegionType,
    start: Box<Data<'a>>,
    end: Box<Data<'a>>,
}

mod tests {
    use super::*;
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_ti_unit_to_axdr() {
        let ti_unit = TiUnit::Hour;
        let axdr = ti_unit.to_axdr_vec().unwrap();

        assert_eq!(axdr, [0x02]);
    }

    #[test]
    fn test_ti_unit_from_axdr() {
        let axdr = [0x02];
        let (_, ti_unit) = TiUnit::from_axdr(&axdr).unwrap();

        assert_eq!(ti_unit, TiUnit::Hour);
    }

    #[test]
    fn test_ti_to_axdr() {
        let ti = TI {
            unit: TiUnit::Hour,
            interval: 0x1234,
        };
        let axdr = ti.to_axdr_vec().unwrap();

        assert_eq!(axdr, [0x02, 0x12, 0x34]);
    }

    #[test]
    fn test_ti_from_axdr() {
        let axdr = [0x02, 0x12, 0x34];
        let (_, ti) = TI::from_axdr(&axdr).unwrap();

        assert_eq!(ti.unit, TiUnit::Hour);
        assert_eq!(ti.interval, 0x1234);
    }
}

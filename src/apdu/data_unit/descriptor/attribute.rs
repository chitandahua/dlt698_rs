use asn1_type::{LongUnsigned, SequenceOf, Unsigned};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

pub type OI = LongUnsigned;

#[derive(Debug, AxdrSequence, ToAxdrSequence, Eq, PartialEq)]
pub struct OAD {
    object_identifier: OI,
    attribute: Unsigned,
    index: Unsigned,
}

impl OAD {
    pub fn new(oi: OI, attr: Unsigned, index: Unsigned) -> OAD {
        OAD {
            object_identifier: oi,
            attribute: attr,
            index,
        }
    }
}

#[derive(Debug, AxdrSequence, ToAxdrSequence, PartialEq, Eq)]
pub struct ROAD {
    oad: OAD,
    relate_oads: SequenceOf<OAD>,
}

impl ROAD {
    pub fn new(oad: OAD, relate_oads: SequenceOf<OAD>) -> ROAD {
        ROAD { oad, relate_oads }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_oad_to_axdr() {
        let oad = OAD::new(0x202a, 0x02, 0);

        let axdr = oad.to_axdr_vec().unwrap();
        assert_eq!(axdr, [0x20, 0x2a, 0x02, 0x00]);
    }

    #[test]
    fn test_oad_from_axdr() {
        let axdr = [0x20, 0x2a, 0x02, 0x00];
        let (_byte, oad) = OAD::from_axdr(&axdr).unwrap();
        assert_eq!(oad.object_identifier, 0x202a);
        assert_eq!(oad.attribute, 0x02);
        assert_eq!(oad.index, 0);
    }

    #[test]
    fn test_road_to_axdr() {
        let road = ROAD::new(
            OAD::new(0x5002, 0x02, 0),
            vec![
                OAD::new(0x2021, 0x02, 0),
                OAD::new(0x0010, 0x02, 0),
                OAD::new(0x0020, 0x02, 0),
            ],
        );

        let axdr = road.to_axdr_vec().unwrap();
        assert_eq!(
            axdr,
            [
                0x50, 0x02, 0x02, 0x00, 0x03, 0x20, 0x21, 0x02, 0x00, 0x00, 0x10, 0x02, 0x00, 0x00,
                0x20, 0x02, 0x00
            ]
        );
    }

    #[test]
    fn test_road_from_axdr() {
        let axdr = [
            0x50, 0x02, 0x02, 0x00, 0x03, 0x20, 0x21, 0x02, 0x00, 0x00, 0x10, 0x02, 0x00, 0x00,
            0x20, 0x02, 0x00,
        ];

        let (_byte, road) = ROAD::from_axdr(&axdr).unwrap();
        let result = ROAD::new(
            OAD::new(0x5002, 0x02, 0),
            vec![
                OAD::new(0x2021, 0x02, 0),
                OAD::new(0x0010, 0x02, 0),
                OAD::new(0x0020, 0x02, 0),
            ],
        );

        assert_eq!(result, road);
    }
}

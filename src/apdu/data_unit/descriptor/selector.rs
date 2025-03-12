use super::attribute::{OAD, ROAD};
use asn1_type::{Null, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum CSD {
    #[tag(0)]
    Oad(OAD),
    #[tag(1)]
    Road(ROAD),
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum RSD {
    #[tag(0)]
    Null(Null),
    //    #[tag(1)]
    //    Selector1(Selector1),
    //    #[tag(2)]
    //    Selector2(Selector2),
    //    #[tag(3)]
    //    Selector3(Selector3),
    //    #[tag(4)]
    //    Selector4(Selector4),
    //    #[tag(5)]
    //    Selector5(Selector5),
    //    #[tag(6)]
    //    Selector6(Selector6),
    //    #[tag(7)]
    //    Selector7(Selector7),
    //    #[tag(8)]
    //    Selector8(Selector8),
    //    #[tag(9)]
    //    Selector9(Selector9),
    //    #[tag(10)]
    //    Selector10(Selector10),
}

pub type RCSD = SequenceOf<CSD>;

mod tests {

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
}

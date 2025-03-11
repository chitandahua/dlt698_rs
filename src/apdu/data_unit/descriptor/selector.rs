use super::attribute::{OAD, ROAD};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
enum CSD {
    #[tag(0)]
    Oad(OAD),
    #[tag(1)]
    Road(ROAD),
}

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
}

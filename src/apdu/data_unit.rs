use asn1_rs::{Error, ParseResult, Result, SerializeResult};
use asn1_type::traits::{FromAxdr, ToAxdr};
use asn1_type::{
    BitString, Float32, Float64, Long64, Long64Unsigned, Null, Utf8String, VisibleString,
};
use asn1_type::{
    Boolean, DoubleLong, DoubleLongUnsigned, Enumerated, Integer, Long, LongUnsigned, OctetString,
    SequenceOf, Unsigned,
};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

mod com;
pub use com::COMDCB;

mod connect;
pub use connect::*;

mod dar;
pub use dar::DAR;

mod date_time;
pub use date_time::*;

mod descriptor;
pub use descriptor::*;

mod ms;
pub use ms::MS;

mod region;
pub use region::{Region, TI};

mod security;
pub use security::{MAC, RN, SID, SIDMAC};

pub type PIID = Unsigned;
#[allow(non_camel_case_types)]
pub type PIID_ACD = Unsigned;
pub type TSA<'a> = OctetString<'a>;

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ScalerUnit<'a> {
    scaler: Integer<'a>,
    unit: Enumerated,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum Data<'a> {
    #[tag(0)]
    Null(Null),
    #[tag(1)]
    Array(SequenceOf<Data<'a>>),
    #[tag(2)]
    Structure(SequenceOf<Data<'a>>),
    #[tag(3)]
    Bool(Boolean),
    #[tag(4)]
    BitString(BitString<'a>),
    #[tag(5)]
    DoubleLong(DoubleLong),
    #[tag(6)]
    DoubleLongUnsigned(DoubleLongUnsigned),
    #[tag(9)]
    OctetString(OctetString<'a>),
    #[tag(10)]
    VisibleString(VisibleString<'a>),
    #[tag(12)]
    Utf8String(Utf8String<'a>),
    #[tag(15)]
    Integer(Integer<'a>),
    #[tag(16)]
    Long(Long),
    #[tag(17)]
    Unsigned(Unsigned),
    #[tag(18)]
    LongUnsigned(LongUnsigned),
    #[tag(20)]
    Long64(Long64),
    #[tag(21)]
    Long64Unsigned(Long64Unsigned),
    #[tag(22)]
    Enum(Enumerated),
    #[tag(23)]
    Float32(Float32),
    #[tag(24)]
    Float64(Float64),
    #[tag(25)]
    DateTime(DateTime),
    #[tag(26)]
    Date(Date),
    #[tag(27)]
    Time(Time),
    #[tag(28)]
    DateTimeS(DateTimeS),
    #[tag(80)]
    OI(OI),
    #[tag(81)]
    OAD(OAD),
    #[tag(82)]
    ROAD(ROAD),
    #[tag(83)]
    OMD(OMD),
    #[tag(84)]
    TI(TI),
    #[tag(85)]
    TSA(TSA<'a>),
    #[tag(86)]
    MAC(MAC<'a>),
    #[tag(87)]
    RN(RN<'a>),
    #[tag(88)]
    Region(Region<'a>),
    #[tag(89)]
    ScalerUnit(ScalerUnit<'a>),
    #[tag(90)]
    RSD(RSD),
    #[tag(91)]
    CSD(CSD),
    #[tag(92)]
    MS(MS<'a>),
    #[tag(93)]
    SID(SID<'a>),
    #[tag(94)]
    SIDMAC(SIDMAC<'a>),
    #[tag(95)]
    COMDCB(COMDCB),
    #[tag(96)]
    RCSD(RCSD),
}

impl<'a> FromAxdr<'a> for Box<Data<'a>> {
    fn from_axdr(bytes: &'a [u8]) -> asn1_rs::ParseResult<'a, Self, asn1_rs::Error> {
        let (bytes, data) = Data::from_axdr(bytes)?;
        Ok((bytes, Box::new(data)))
    }
}

impl ToAxdr for Box<Data<'_>> {
    fn to_axdr_len(&self) -> Result<usize> {
        self.as_ref().to_axdr_len()
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        self.as_ref().write_axdr_header(writer)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        self.as_ref().write_axdr_content(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_data_long_to_axdr() {
        let data = Data::Long(0x1234);
        let axdr = data.to_axdr_vec().unwrap();

        assert_eq!(axdr, [0x10, 0x12, 0x34]);
    }

    #[test]
    fn test_data_long_from_axdr() {
        let axdr = [0x10, 0x12, 0x34];
        let (_, data) = Data::from_axdr(&axdr).unwrap();

        assert_eq!(data, Data::Long(0x1234));
    }

    #[test]
    fn test_data_tsa_to_axdr() {
        let data = Data::TSA(TSA::new(&[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]));
        let axdr = data.to_axdr_vec().unwrap();

        assert_eq!(axdr, [0x55, 0x07, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn test_data_tsa_from_axdr() {
        let axdr = [0x55, 0x07, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let (_, data) = Data::from_axdr(&axdr).unwrap();

        assert_eq!(
            data,
            Data::TSA(TSA::new(&[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]))
        )
    }

    #[test]
    fn test_data_date_time_s_to_axdr() {
        let data = Data::DateTimeS(DateTimeS::new(0x07e7, 0x03, 0x04, 0x00, 0x00, 0x00));
        let axdr = data.to_axdr_vec().unwrap();

        assert_eq!(axdr, [0x1c, 0x07, 0xe7, 0x03, 0x04, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_data_date_time_s_from_axdr() {
        let axdr = [0x1c, 0x07, 0xe7, 0x03, 0x04, 0x00, 0x00, 0x00];
        let (_, data) = Data::from_axdr(&axdr).unwrap();

        assert_eq!(
            data,
            Data::DateTimeS(DateTimeS::new(0x07e7, 0x03, 0x04, 0x00, 0x00, 0x00))
        )
    }

    #[test]
    fn test_data_combine_to_axdr() {}

    #[test]
    fn test_data_combine_from_axdr() {
        let axdr = hex::decode("01055507050000000000011c07e703040000001c07e703041502031c07e70303173b0001031c07e70304000000010506000007d006000001f4060000019006000001f40600000258010506000007d006000001f4060000019006000001f40600000258").unwrap();
        let (_, data) = Data::from_axdr(&axdr).unwrap();

        let data = match data {
            Data::Array(array) => {
                assert_eq!(array.len(), 5);
                array
            }
            _ => panic!("axdr sequence failed"),
        };

        match &data[0] {
            Data::TSA(tsa) => {
                assert_eq!(tsa, &TSA::new(&[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]));
            }
            _ => panic!("axdr sequence failed"),
        }

        match &data[3] {
            Data::DateTimeS(data_time_s) => {
                assert_eq!(
                    data_time_s,
                    &DateTimeS::new(0x07e7, 0x03, 0x03, 0x17, 0x3b, 0x00)
                );
            }
            _ => panic!("axdr sequence failed"),
        }

        let data = match &data[4] {
            Data::Array(array) => {
                eprintln!("{:?}", array);
                assert_eq!(array.len(), 3);
                array
            }
            _ => panic!("axdr sequence failed"),
        };

        let data = match &data[2] {
            Data::Array(array) => {
                assert_eq!(array.len(), 5);
                array
            }
            _ => panic!("axdr sequence failed"),
        };

        assert_eq!(data[0], Data::DoubleLongUnsigned(0x000007d0));
        assert_eq!(data[3], Data::DoubleLongUnsigned(0x000001f4));
    }
}

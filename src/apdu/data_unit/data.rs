use asn1_type::traits::{FromAxdr, ToAxdr};
use asn1_type::{
    BitString, FixedBitString, FixedOctetString, FixedUtf8String, FixedVisibleString, Long64,
    Long64Unsigned, Null, Utf8String, VisibleString,
};
use asn1_type::{
    Boolean, DoubleLong, DoubleLongUnsigned, Enumerated, Integer, Long, LongUnsigned, OctetString,
    SequenceOf, Unsigned,
};
use asn1_type::{Error, ParseResult, Result, SerializeResult};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

use crate::apdu::data_unit::{
    Date, DateTime, DateTimeS, Float32, Float64, Region, Time, COMDCB, CSD, MAC, MS, OAD, OI, OMD,
    RCSD, RN, ROAD, RSD, SID, SIDMAC, TI, TSA,
};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ScalerUnit {
    scaler: Integer,
    unit: Enumerated,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum Data {
    #[tag(0)]
    Null(Null),
    #[tag(1)]
    Array(SequenceOf<Data>),
    #[tag(2)]
    Structure(SequenceOf<Data>),
    #[tag(3)]
    Bool(Boolean),
    #[tag(4)]
    BitString(BitString),
    #[tag(5)]
    DoubleLong(DoubleLong),
    #[tag(6)]
    DoubleLongUnsigned(DoubleLongUnsigned),
    #[tag(9)]
    OctetString(OctetString),
    #[tag(10)]
    VisibleString(VisibleString),
    #[tag(12)]
    Utf8String(Utf8String),
    #[tag(15)]
    Integer(Integer),
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
    TSA(TSA),
    #[tag(86)]
    MAC(MAC),
    #[tag(87)]
    RN(RN),
    #[tag(88)]
    Region(Region),
    #[tag(89)]
    ScalerUnit(ScalerUnit),
    #[tag(90)]
    RSD(RSD),
    #[tag(91)]
    CSD(CSD),
    #[tag(92)]
    MS(MS),
    #[tag(93)]
    SID(SID),
    #[tag(94)]
    SIDMAC(SIDMAC),
    #[tag(95)]
    COMDCB(COMDCB),
    #[tag(96)]
    RCSD(RCSD),
}

impl FromAxdr<'_> for Box<Data> {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self, Error> {
        let (bytes, data) = Data::from_axdr(bytes)?;
        Ok((bytes, Box::new(data)))
    }
}

impl ToAxdr for Box<Data> {
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

macro_rules! impl_into_data {
    ($ty:ty, $to:ident) => {
        impl From<$ty> for Data {
            fn from(value: $ty) -> Self {
                Data::$to(value)
            }
        }
    };
}

impl_into_data!(Null, Null);
impl_into_data!(Boolean, Bool);
impl_into_data!(BitString, BitString);
impl_into_data!(DoubleLong, DoubleLong);
impl_into_data!(DoubleLongUnsigned, DoubleLongUnsigned);
impl_into_data!(OctetString, OctetString);
impl_into_data!(VisibleString, VisibleString);
impl_into_data!(Utf8String, Utf8String);
impl_into_data!(Integer, Integer);
impl_into_data!(Long, Long);
impl_into_data!(Unsigned, Unsigned);
impl_into_data!(LongUnsigned, LongUnsigned);
impl_into_data!(Long64, Long64);
impl_into_data!(Long64Unsigned, Long64Unsigned);
impl_into_data!(Enumerated, Enum);
impl_into_data!(Float32, Float32);
impl_into_data!(Float64, Float64);
impl_into_data!(DateTime, DateTime);
impl_into_data!(Date, Date);
impl_into_data!(Time, Time);
impl_into_data!(DateTimeS, DateTimeS);
impl_into_data!(OI, OI);
impl_into_data!(OAD, OAD);
impl_into_data!(ROAD, ROAD);
impl_into_data!(OMD, OMD);
impl_into_data!(TI, TI);
impl_into_data!(TSA, TSA);
impl_into_data!(MAC, MAC);
impl_into_data!(RN, RN);
impl_into_data!(Region, Region);
impl_into_data!(ScalerUnit, ScalerUnit);
impl_into_data!(RSD, RSD);
impl_into_data!(CSD, CSD);
impl_into_data!(MS, MS);
impl_into_data!(SID, SID);
impl_into_data!(SIDMAC, SIDMAC);
impl_into_data!(COMDCB, COMDCB);
impl_into_data!(RCSD, RCSD);

impl<const N: usize> From<FixedOctetString<N>> for Data {
    fn from(value: FixedOctetString<N>) -> Self {
        Data::OctetString(value.into())
    }
}

impl<const N: usize, const BYTES: usize> From<FixedBitString<N, BYTES>> for Data {
    fn from(value: FixedBitString<N, BYTES>) -> Self {
        Data::BitString(value.into())
    }
}

impl<const N: usize> From<FixedUtf8String<N>> for Data {
    fn from(value: FixedUtf8String<N>) -> Self {
        Data::Utf8String(value.into())
    }
}

impl<const N: usize> From<FixedVisibleString<N>> for Data {
    fn from(value: FixedVisibleString<N>) -> Self {
        Data::VisibleString(value.into())
    }
}

impl<T> From<SequenceOf<T>> for Data
where
    T: Into<Data>,
{
    fn from(value: SequenceOf<T>) -> Self {
        Data::Array(value.into_iter().map(Into::into).collect())
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
    fn test_data_combine_to_axdr() {
        let addr = Data::TSA(TSA::new(&[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]));
        let date_time_1 = Data::DateTimeS(DateTimeS::new(2023, 3, 4, 0, 0, 0));
        let date_time_2 = Data::DateTimeS(DateTimeS::new(2023, 3, 4, 21, 2, 3));
        let date_time_3 = Data::DateTimeS(DateTimeS::new(2023, 3, 3, 23, 59, 0));

        let frozen = Data::Array(vec![
            Data::DateTimeS(DateTimeS::new(2023, 3, 4, 0, 0, 0)),
            Data::Array(vec![
                Data::DoubleLongUnsigned(2000),
                Data::DoubleLongUnsigned(500),
                Data::DoubleLongUnsigned(400),
                Data::DoubleLongUnsigned(500),
                Data::DoubleLongUnsigned(600),
            ]),
            Data::Array(vec![
                Data::DoubleLongUnsigned(2000),
                Data::DoubleLongUnsigned(500),
                Data::DoubleLongUnsigned(400),
                Data::DoubleLongUnsigned(500),
                Data::DoubleLongUnsigned(600),
            ]),
        ]);

        let data = Data::Array(vec![addr, date_time_1, date_time_2, date_time_3, frozen]);

        let axdr = data.to_axdr_vec().unwrap();
        assert_eq!(hex::encode(axdr), "01055507050000000000011c07e703040000001c07e703041502031c07e70303173b0001031c07e70304000000010506000007d006000001f4060000019006000001f40600000258010506000007d006000001f4060000019006000001f40600000258");
    }

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

use asn1_rs::{BitString, Boolean, Enumerated, Integer, OctetString, Utf8String, VisibleString};
use traits::AxdrTag;


impl AxdrTag for Data {
    fn tag(&self) -> u8 {
        match self {
            Data::Null => 0,
            Data::Array(_array) => 1,
            Data::Structure(_structure) => 2,
            Data::Bool(_bool) => 3,
            Data::BitString(_bit_string) => 4,
            Data::DoubleLong(_double_long) => 5,
            Data::DoubleLongUnsigned(_double_long_unsigned) => 6,
            Data::OctetString(_octet_string) => 9,
            Data::VisibleString(_visible_string) => 10,
            Data::Utf8String(_utf8_string) => 12,
            Data::Integer(_integer) => 15,
            Data::Long(_long) => 16,
            Data::Unsigned(_unsigned) => 17,
            Data::LongUnsigned(_long_unsigned) => 18,
            Data::Long64(_long64) => 20,
            Data::Long64Unsigned(_long64_unsigned) => 21,
            Data::Enum(_enum) => 22,
            Data::Float32(_float32) => 23,
            Data::Float64(_float64) => 24,
            Data::DateTime(_date_time) => 25,
            Data::Date(_date) => 26,
            Data::Time(_time) => 27,
            Data::DateTimes(_date_time_s) => 28,
            Data::Oi(_oi) => 80,
            Data::Oad(_oad) => 81,
            //Data::Road(_road) => 82,
            //Data::Omd(_omd) => 83,
            //Data::Ti(_ti) => 84,
            //Data::Tsa(_tsa) => 85,
            //Data::Mac(_mac) => 86,
            //Data::Rn(_rn) => 87,
            //Data::Region(_region) => 88,
            //Data::ScalerUnit(_scaler_unit) => 89,
            //Data::Rsd(_rsd) => 90,
            //Data::Csd(_csd) => 91,
            //Data::Ms(_ms) => 92,
            //Data::Sid(_sid) => 93,
            //Data::SidMac(_sid_mac) => 94,
            //Data::ComDcb(_com_dcb) => 95,
            //Data::Rcsd(_rcsd) => 96
        }
    }
}

pub enum Data {
    Null,
    Array(Box<Array>),
    Structure(Box<Structure>),
    Bool(Boolean),
    BitString(BitString),
    DoubleLong(i32),
    DoubleLongUnsigned(u32),
    OctetString(OctetString),
    VisibleString(VisibleString),
    Utf8String(Utf8String),
    Integer(i8),
    Long(i16),
    Unsigned(u8),
    LongUnsigned(u16),
    Long64(i64),
    Long64Unsigned(u64),
    Enum(Enumerated),
    Float32([u8; 4]),
    Float64([u8; 8]),
    DateTime([u8; 10]),
    Date([u8; 5]),
    Time([u8; 3]),
    DateTimes([u8; 7]),
    Oi(Oi),
    Oad(Oad),
    //Road(Road),
    //Omd(Omd),
    //Ti(Ti),
    //Tsa(Tsa),
    //Mac(Mac),
    //Rn(Rn),
    //Region(Region),
    //ScalerUnit(ScalerUnit),
    //Rsd(Rsd),
    //Csd(Csd),
    //Ms(Ms),
    //Sid(Sid),
    //SidMac(SidMac),
    //ComDcb(ComDcb),
    //Rcsd(Rcsd),
}

type Array = Vec<Data>;
type Structure = Vec<Data>;
type Oi = u32;

pub struct Oad {
    identifier: Oi,
    attr: u8,
    index: u8,
}
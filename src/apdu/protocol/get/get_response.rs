use crate::apdu::data_unit::{Data, DAR, OAD, PIID_ACD, RCSD};
use asn1_type::traits::{FromAxdr, ToAxdr};
use asn1_type::{Boolean, LongUnsigned, OctetString, SequenceOf, UnsignedInteger};
use asn1_type::{ParseResult, Result, SerializeResult};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// GetResponseNormal
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseNormal {
    pub piid_acd: PIID_ACD,
    pub a_result_normal: AResultNormal,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct AResultNormal {
    pub oad: OAD,
    pub get_result: GetResult,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum GetResult {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    Data(Data),
}

impl<T, E> From<Result<T, E>> for GetResult
where
    T: Into<Data>,
    E: Into<DAR>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => GetResult::Data(data.into()),
            Err(err) => GetResult::Dar(err.into()),
        }
    }
}

// GetResponseNormalList∷
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseNormalList {
    piid_acd: PIID_ACD,
    a_result_normal: SequenceOf<AResultNormal>,
}

// GetResponseRecord
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseRecord {
    piid_acd: PIID_ACD,
    a_result_record: AResultRecord,
}

// 由于ARecordRow中的定义并不是SequenceOf 是根据RCSD里面CSD的数量决定的 故不能直接derive宏
#[derive(Debug, PartialEq, Eq, ToAxdrSequence)]
pub struct AResultRecord {
    oad: OAD,
    rcsd: RCSD,
    record_result: RecordResult,
}

#[derive(Debug, PartialEq, Eq, ToAxdrSequence)]
pub enum RecordResult {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    RecordRows(SequenceOf<ARecordRow>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ARecordRow {
    data: SequenceOf<Data>,
}

impl ToAxdr for ARecordRow {
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(if self.data.is_empty() {
            0
        } else {
            self.data[0].to_axdr_len()?
        } * self.data.len())
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        let mut num_bytes = 0;
        for t in self.data.iter() {
            num_bytes += t.write_axdr(writer)?;
        }
        Ok(num_bytes)
    }
}

impl FromAxdr<'_> for AResultRecord {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        let (bytes, oad) = OAD::from_axdr(bytes)?;
        let (bytes, rcsd) = RCSD::from_axdr(bytes)?;
        let csd_num = rcsd.len();

        let (bytes, tag) = u8::from_axdr(bytes)?;
        let (bytes, record_result) = match tag {
            0 => {
                let (bytes, dar) = DAR::from_axdr(bytes)?;
                Ok((bytes, RecordResult::Dar(dar)))
            }
            1 => {
                let (mut bytes, int) = UnsignedInteger::from_axdr(bytes)?;
                let len = int.as_usize()?;
                let mut record_rows = Vec::with_capacity(len);

                while record_rows.len() < len {
                    let mut a_record_row = Vec::with_capacity(csd_num);
                    while a_record_row.len() < csd_num {
                        let (b, t) = Data::from_axdr(bytes)?;
                        a_record_row.push(t);
                        bytes = b;
                    }
                    record_rows.push(ARecordRow { data: a_record_row });
                }
                Ok((bytes, RecordResult::RecordRows(record_rows)))
            }
            _ => Err(asn1_type::Error::InvalidTag),
        }?;

        Ok((
            bytes,
            AResultRecord {
                oad,
                rcsd,
                record_result,
            },
        ))
    }
}

// GetResponseRecordList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseRecordList {
    piid_acd: PIID_ACD,
    a_result_record: SequenceOf<AResultRecord>,
}

// GetResponseNext
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseNext {
    piid_acd: PIID_ACD,
    last_fragment: Boolean,
    fragment_number: LongUnsigned,
    fragment_response: FragmentResponse,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum FragmentResponse {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    AResultNormal(SequenceOf<AResultNormal>),
    #[tag(2)]
    AResultRecord(SequenceOf<AResultRecord>),
}

// GetResponseMD5
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct GetResponseMD5 {
    piid_acd: PIID_ACD,
    oad: OAD,
    result: ResultMD5,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ResultMD5 {
    #[tag(0)]
    Dar(DAR),
    #[tag(1)]
    MD5(OctetString),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apdu::data_unit::CSD;
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_get_response_record_from_axdr() {
        let axdr = hex::decode("25601203000400202a02000000100200002000020000200102000102550705000000000001010506000927c006000126a006000136a006000146a006000216a001031208991207da1208200103050000053c05000009240500000d0c550705000000000002010506000927c006000186a006000186a006000186a006000186a001031208991207da1208200103050000053c05000009240500000d0c").unwrap();
        let (bytes, get_response_record) = GetResponseRecord::from_axdr(&axdr).unwrap();

        assert!(bytes.is_empty());
        //println!("{:#?}", get_response_record);

        assert_eq!(get_response_record.piid_acd, 0x25);
    }

    #[test]
    fn test_get_response_record_to_axdr() {
        let get_response_record = GetResponseRecord {
            piid_acd: 37,
            a_result_record: AResultRecord {
                oad: OAD::new(24594, 3, 0),
                rcsd: vec![
                    CSD::Oad(OAD::new(8234, 2, 0)),
                    CSD::Oad(OAD::new(16, 2, 0)),
                    CSD::Oad(OAD::new(8192, 2, 0)),
                    CSD::Oad(OAD::new(8193, 2, 0)),
                ],
                record_result: RecordResult::RecordRows(vec![
                    ARecordRow {
                        data: vec![
                            Data::TSA(OctetString::new(&[5, 0, 0, 0, 0, 0, 1])),
                            Data::Array(vec![
                                Data::DoubleLongUnsigned(600000),
                                Data::DoubleLongUnsigned(75424),
                                Data::DoubleLongUnsigned(79520),
                                Data::DoubleLongUnsigned(83616),
                                Data::DoubleLongUnsigned(136864),
                            ]),
                            Data::Array(vec![
                                Data::LongUnsigned(2201),
                                Data::LongUnsigned(2010),
                                Data::LongUnsigned(2080),
                            ]),
                            Data::Array(vec![
                                Data::DoubleLong(1340),
                                Data::DoubleLong(2340),
                                Data::DoubleLong(3340),
                            ]),
                        ],
                    },
                    ARecordRow {
                        data: vec![
                            Data::TSA(OctetString::new(&[5, 0, 0, 0, 0, 0, 2])),
                            Data::Array(vec![
                                Data::DoubleLongUnsigned(600000),
                                Data::DoubleLongUnsigned(100000),
                                Data::DoubleLongUnsigned(100000),
                                Data::DoubleLongUnsigned(100000),
                                Data::DoubleLongUnsigned(100000),
                            ]),
                            Data::Array(vec![
                                Data::LongUnsigned(2201),
                                Data::LongUnsigned(2010),
                                Data::LongUnsigned(2080),
                            ]),
                            Data::Array(vec![
                                Data::DoubleLong(1340),
                                Data::DoubleLong(2340),
                                Data::DoubleLong(3340),
                            ]),
                        ],
                    },
                ]),
            },
        };

        let axdr = get_response_record.to_axdr_vec().unwrap();

        assert_eq!(hex::encode(axdr), "25601203000400202a02000000100200002000020000200102000102550705000000000001010506000927c006000126a006000136a006000146a006000216a001031208991207da1208200103050000053c05000009240500000d0c550705000000000002010506000927c006000186a006000186a006000186a006000186a001031208991207da1208200103050000053c05000009240500000d0c");
    }
}

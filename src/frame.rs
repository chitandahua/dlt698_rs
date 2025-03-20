use super::checksum::caculate_fcs16;
use crate::apdu::Apdu;
use crate::Result;
use anyhow::ensure;
use asn1_type::traits::{FromAxdr, ToAxdr};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumString;
use thiserror::Error;

const HEADER_START: u8 = 0x68;

// length field
#[derive(Debug, Clone, Copy, PartialEq)]
struct LengthField(u16);

impl LengthField {
    fn new(length: u16) -> Self {
        // 取bit0-bit13
        Self(length & 0x1FFF)
    }

    fn len(&self) -> u16 {
        self.0
    }
}

impl From<&[u8]> for LengthField {
    fn from(bytes: &[u8]) -> Self {
        LengthField(u16::from_le_bytes([bytes[0], bytes[1]]))
    }
}

impl From<LengthField> for Vec<u8> {
    fn from(length: LengthField) -> Self {
        length.len().to_le_bytes().to_vec()
    }
}

impl IntoIterator for LengthField {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<u8>>::into(self).into_iter()
    }
}

// ctrl field
#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Dir {
    Client = 0,
    Server = 1,
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Prm {
    Server = 0,
    Client = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum FunctionCode {
    LinkManagement = 1,
    UserData = 3,
}

#[derive(Debug, Clone)]
pub struct CtrlField {
    dir: Dir,
    prm: Prm,
    fragment: u8,
    function_code: FunctionCode,
}

impl Default for CtrlField {
    fn default() -> Self {
        CtrlField {
            dir: Dir::Client,
            prm: Prm::Server,
            fragment: 0,
            function_code: FunctionCode::UserData,
        }
    }
}

impl CtrlField {
    pub fn new(is_response: bool, is_framing: bool, function_code: FunctionCode) -> Self {
        CtrlField {
            dir: Dir::Client,
            prm: if is_response {
                Prm::Server
            } else {
                Prm::Client
            },
            fragment: is_framing as u8,
            function_code,
        }
    }
}

impl From<CtrlField> for u8 {
    fn from(ctrl_field: CtrlField) -> Self {
        // 7: dir, 6: prm, 5: fragment 3-1: function_code
        ((ctrl_field.dir as u8) << 7)
            | ((ctrl_field.prm as u8) << 6)
            | (Into::<u8>::into(ctrl_field.fragment) << 5)
            | (ctrl_field.function_code as u8)
    }
}

impl TryFrom<u8> for CtrlField {
    type Error = crate::Error;
    fn try_from(ctrl_field: u8) -> Result<Self> {
        Ok(CtrlField {
            dir: (ctrl_field >> 7).try_into().unwrap(),
            prm: ((ctrl_field >> 6) & 0x01).try_into().unwrap(),
            fragment: ((ctrl_field >> 5) & 0x01).try_into().unwrap(),
            function_code: (ctrl_field & 0x07).try_into()?,
        })
    }
}

impl IntoIterator for CtrlField {
    type Item = u8;
    type IntoIter = std::iter::Once<u8>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.into())
    }
}

#[derive(Debug, Clone)]
pub struct AddressField {
    server_addr: ServerAddr,
    client_addr: u8,
}

#[derive(Debug, Clone)]
pub struct ServerAddr {
    addr_type: u8,
    logic_addr: u8,
    addr: Vec<u8>, // TODO Cow
}

impl ServerAddr {
    pub fn new(addr_type: u8, logic_addr: u8, addr: Vec<u8>) -> Self {
        Self {
            addr_type,
            logic_addr,
            addr,
        }
    }
}

impl AddressField {
    pub fn new(server_addr: ServerAddr, client_addr: u8) -> Self {
        Self {
            server_addr,
            client_addr,
        }
    }

    fn bytes_len(&self) -> usize {
        2 + self.server_addr.addr.len()
    }
}

impl TryFrom<&[u8]> for AddressField {
    type Error = crate::Error;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        ensure!(bytes.len() >= 2, FrameError::Length(bytes.len()));
        let len = ((bytes[0] & 0x0f) + 1) as usize;
        ensure!(bytes.len() >= 2 + len, FrameError::Length(bytes.len()));
        let server_addr = ServerAddr {
            addr_type: (bytes[0] & 0xc0) >> 6,
            logic_addr: (bytes[0] & 0x30) >> 4,
            addr: bytes[1..len + 1]
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        };
        Ok(AddressField {
            server_addr,
            client_addr: bytes[len + 1],
        })
    }
}

impl From<AddressField> for Vec<u8> {
    fn from(address: AddressField) -> Self {
        let mut bytes = Vec::with_capacity(address.bytes_len());
        bytes.push(
            (Into::<u8>::into(address.server_addr.addr_type) << 6)
                | (Into::<u8>::into(address.server_addr.logic_addr) << 4)
                | ((address.server_addr.addr.len() - 1) as u8 & 0x0f),
        );
        bytes.extend(address.server_addr.addr.iter().rev());
        bytes.push(address.client_addr);
        bytes
    }
}

impl IntoIterator for AddressField {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<u8>>::into(self).into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    start: u8,
    length_field: LengthField,
    control_field: CtrlField,
    address_field: AddressField,
    checksum: u16,
}

impl Header {
    pub fn new(control_field: CtrlField, address_field: AddressField, length: u16) -> Self {
        let mut header = Self {
            start: HEADER_START,
            length_field: LengthField::new(length),
            control_field,
            address_field,
            checksum: 0,
        };
        header.caculate_checksum();
        header
    }

    fn caculate_checksum(&mut self) {
        let mut bytes = Vec::with_capacity(self.bytes_len() - 1 - CHECKSUM_SIZE);
        bytes.extend(self.length_field);
        bytes.extend(self.control_field.clone());
        bytes.extend(self.address_field.clone());
        self.checksum = caculate_fcs16(bytes.as_slice());
    }

    fn _into_vec(self) -> Vec<u8> {
        self.into()
    }

    fn bytes_len(&self) -> usize {
        1 + LENGTH_FIELD_SIZE + CONTROL_FIELD_SIZE + self.address_field.bytes_len() + CHECKSUM_SIZE
    }

    fn is_framing(&self) -> bool {
        self.control_field.fragment == 1
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = crate::Error;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut offset = 1 + LENGTH_FIELD_SIZE + CONTROL_FIELD_SIZE;
        ensure!(bytes.len() >= offset, FrameError::Length(bytes.len()));
        ensure!(bytes[0] == HEADER_START, FrameError::Header(bytes[0]));
        let length_field = LengthField::from(&bytes[1..3]);
        let control_field = CtrlField::try_from(bytes[3])?;
        let address_field = AddressField::try_from(&bytes[4..])?;

        offset += address_field.bytes_len();
        ensure!(
            bytes[4..].len() >= offset + CHECKSUM_SIZE,
            FrameError::Length(bytes.len())
        );
        let checksum = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        let expect_checksum = caculate_fcs16(&bytes[1..offset]);
        ensure!(
            expect_checksum == checksum,
            FrameError::HeaderChecksum {
                checksum: checksum,
                expected: expect_checksum
            }
        );
        Ok(Header {
            start: HEADER_START,
            length_field,
            control_field,
            address_field,
            checksum,
        })
    }
}

impl From<Header> for Vec<u8> {
    fn from(header: Header) -> Self {
        let mut bytes = Vec::new();
        bytes.push(header.start);
        bytes.extend(header.length_field);
        bytes.extend(header.control_field);
        bytes.extend(header.address_field);
        bytes.extend(header.checksum.to_le_bytes());
        bytes
    }
}

impl IntoIterator for Header {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<u8>>::into(self).into_iter()
    }
}

// tail
const TAIL_END: u8 = 0x16;
#[derive(Debug)]
pub struct Tail {
    checksum: u16,
    end: u8,
}

impl From<Tail> for Vec<u8> {
    fn from(tail: Tail) -> Self {
        let mut bytes = Vec::new();
        bytes.extend(tail.checksum.to_le_bytes());
        bytes.push(tail.end);
        bytes
    }
}

impl IntoIterator for Tail {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<u8>>::into(self).into_iter()
    }
}

// user_data
#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum FragmentTag {
    Start = 0,
    End = 1,
    Confirm = 2,
    Middle = 3,
}

#[derive(Debug, Clone, PartialEq)]
struct FormatDomain {
    tag: FragmentTag,
    index: u16,
}

impl From<u16> for FormatDomain {
    fn from(format: u16) -> Self {
        FormatDomain {
            tag: (((format & 0xc000) >> 14) as u8).try_into().unwrap(),
            index: format & 0x07ff,
        }
    }
}

impl From<FormatDomain> for u16 {
    fn from(format: FormatDomain) -> Self {
        (format.tag as u16) << 14 | format.index
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApduFragment {
    format_domain: FormatDomain,
    fragment: Vec<u8>,
}

impl TryFrom<&[u8]> for ApduFragment {
    type Error = crate::Error;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        ensure!(bytes.len() >= 2, FrameError::Length(bytes.len()));
        let format_domain = FormatDomain::from(u16::from_le_bytes([bytes[0], bytes[1]]));
        let fragment = bytes[2..bytes.len() - TAIL_SIZE].to_vec();
        Ok(ApduFragment {
            format_domain,
            fragment,
        })
    }
}

impl From<ApduFragment> for Vec<u8> {
    fn from(fragment: ApduFragment) -> Self {
        let mut bytes = Vec::new();
        bytes.extend(Into::<u16>::into(fragment.format_domain).to_le_bytes());
        bytes.extend(fragment.fragment);
        bytes
    }
}

#[derive(Debug, PartialEq)]
pub enum UserData<'a> {
    Apdu(Apdu<'a>),
    Fragment(ApduFragment),
}

impl<'a> UserData<'a> {
    fn new(is_framing: bool, bytes: &'a [u8]) -> Result<(&'a [u8], Self)> {
        if is_framing {
            let fragment = ApduFragment::try_from(bytes)?;
            Ok((
                &bytes[bytes.len() - TAIL_SIZE..],
                UserData::Fragment(fragment),
            ))
        } else {
            let (bytes, apdu) = Apdu::from_axdr(&bytes[..])?;
            Ok((bytes, UserData::Apdu(apdu)))
        }
    }

    fn bytes_len(&self) -> usize {
        const FORMAT_DOMAIN_SIZE: usize = 2;
        match self {
            UserData::Apdu(apdu) => apdu.to_axdr_len().unwrap(),
            UserData::Fragment(fragment) => FORMAT_DOMAIN_SIZE + fragment.fragment.len(),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        match self {
            UserData::Apdu(apdu) => apdu.to_axdr_vec().unwrap(),
            UserData::Fragment(fragment) => fragment.clone().into(),
        }
    }
}

impl<'a> From<UserData<'a>> for Vec<u8> {
    fn from(user_data: UserData<'a>) -> Self {
        match user_data {
            UserData::Apdu(apdu) => apdu.to_axdr_vec().unwrap(),
            UserData::Fragment(fragment) => fragment.into(),
        }
    }
}

impl<'a> IntoIterator for UserData<'a> {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<u8>>::into(self).into_iter()
    }
}

// frame
const LENGTH_FIELD_SIZE: usize = 2;
const CONTROL_FIELD_SIZE: usize = 1;
const CHECKSUM_SIZE: usize = 2;
const TAIL_SIZE: usize = 3;

#[derive(Debug)]
pub struct Frame<'a> {
    header: Header,
    user_data: UserData<'a>,
    tail: Tail,
}

impl<'a> Frame<'a> {
    pub fn new(
        control_field: CtrlField,
        address_field: AddressField,
        user_data: UserData<'a>,
    ) -> Self {
        const FRAME_SIZE: usize =
            LENGTH_FIELD_SIZE + CONTROL_FIELD_SIZE + CHECKSUM_SIZE + TAIL_SIZE - 1;
        let length = user_data.bytes_len() + address_field.bytes_len() + FRAME_SIZE;
        let header = Header::new(control_field, address_field, length as u16);
        let mut frame = Self {
            header,
            user_data,
            tail: Tail {
                checksum: 0,
                end: TAIL_END,
            },
        };
        frame.caculate_checksum();
        frame
    }

    fn caculate_checksum(&mut self) {
        let mut bytes =
            Vec::with_capacity(self.header.bytes_len() + self.user_data.bytes_len() + TAIL_SIZE);
        bytes.extend(self.header.clone().into_iter().skip(1)); // 去掉帧起始字符
        bytes.extend(self.user_data.to_vec());
        self.tail.checksum = caculate_fcs16(bytes.as_slice());
    }
}

impl<'a> TryFrom<&'a [u8]> for Frame<'a> {
    type Error = crate::Error;
    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        let slice = bytes;
        let header = Header::try_from(bytes)?;
        let (bytes, user_data) = UserData::new(header.is_framing(), &bytes[header.bytes_len()..])?;
        let checksum = u16::from_le_bytes([bytes[0], bytes[1]]);
        let expect_checksum = caculate_fcs16(&slice[1..slice.len() - TAIL_SIZE]);
        ensure!(
            expect_checksum == checksum,
            FrameError::FrameChecksum {
                checksum: checksum,
                expected: expect_checksum
            }
        );
        ensure!(bytes[2] == TAIL_END, FrameError::Tail(bytes[2]));
        Ok(Frame {
            header,
            user_data,
            tail: Tail {
                checksum,
                end: TAIL_END,
            },
        })
    }
}

impl<'a> From<Frame<'a>> for Vec<u8> {
    fn from(frame: Frame<'a>) -> Self {
        let mut bytes = Vec::new();
        bytes.extend(frame.header);
        bytes.extend(frame.user_data);
        bytes.extend(frame.tail);
        bytes
    }
}

impl<'a> IntoIterator for Frame<'a> {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<u8>>::into(self).into_iter()
    }
}

#[derive(Error, Debug, PartialEq, EnumString)]
pub enum FrameError {
    #[error("length {0} error")]
    Length(usize),
    #[error("header {0} error")]
    Header(u8),
    #[error("header checksum {checksum:04x} error, expected {expected:04x}")]
    HeaderChecksum { checksum: u16, expected: u16 },
    #[error("frame checksum {checksum:04x} error, expected {expected:04x}")]
    FrameChecksum { checksum: u16, expected: u16 },
    #[error("tail {0} error")]
    Tail(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apdu::data_unit::OAD;
    use crate::apdu::protocol::{
        ClientApdu, ClientApplicationService, GetRequest, GetRequestNormal,
    };
    use crate::apdu::Apdu;

    #[test]
    fn test_frame_from_bytes() {
        let bytes: Vec<u8> = vec![
            0x68, 0x17, 0x00, 0x43, 0x05, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x48, 0x5e,
            0x05, 0x01, 0x15, 0xf1, 0x01, 0x02, 0x00, 0x00, 0x1a, 0x00, 0x16,
        ];
        let frame = Frame::try_from(bytes.as_slice()).unwrap();

        //println!("{:#?}", frame);
        assert_eq!(frame.header.length_field.len(), 0x17);
        assert_eq!(frame.header.control_field.dir, Dir::Client);
        assert_eq!(frame.header.control_field.prm, Prm::Client);

        assert_eq!(frame.header.address_field.server_addr.addr_type, 0);
        assert_eq!(frame.header.address_field.server_addr.logic_addr, 0);
        assert_eq!(
            frame.header.address_field.server_addr.addr.as_slice(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x02]
        );
        assert_eq!(frame.header.address_field.client_addr, 0x10);
        assert_eq!(frame.header.checksum, 0x5e48);
        assert_eq!(
            frame.user_data,
            UserData::Apdu(Apdu::ClientApdu(ClientApdu::new(
                ClientApplicationService::GetRequest(GetRequest::GetRequestNormal(
                    GetRequestNormal::new(0x15, OAD::new(0xf101, 0x02, 0x00)),
                )),
                None,
            )))
        );
        assert_eq!(frame.tail.checksum, 0x001a);
    }

    #[test]
    fn test_frame_to_bytes() {
        let bytes: Vec<u8> = vec![
            0x68, 0x17, 0x00, 0x43, 0x05, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x48, 0x5e,
            0x05, 0x01, 0x15, 0xf1, 0x01, 0x02, 0x00, 0x00, 0x1a, 0x00, 0x16,
        ];

        let apdu = ClientApdu::new(
            ClientApplicationService::GetRequest(GetRequest::GetRequestNormal(
                GetRequestNormal::new(0x15, OAD::new(0xf101, 0x02, 0x00)),
            )),
            None,
        );
        let frame = Frame::new(
            CtrlField {
                dir: Dir::Client,
                prm: Prm::Client,
                fragment: 0,
                function_code: FunctionCode::UserData,
            },
            AddressField::new(
                ServerAddr::new(0, 0, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x02]),
                0x10,
            ),
            UserData::Apdu(Apdu::ClientApdu(apdu)),
        );

        assert_eq!(Into::<Vec<u8>>::into(frame), bytes);
    }
}

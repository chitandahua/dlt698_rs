use super::checksum::calculate_fcs16;
use crate::apdu::Apdu;
use crate::Result;
use anyhow::{ensure, Ok};
use asn1_type::traits::{FromAxdr, ToAxdr};
use modular_bitfield::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::Cursor;
use strum_macros::EnumString;
use thiserror::Error;

const HEADER_START: u8 = 0x68;

// length field
#[bitfield] // 必须放在derive之前
#[derive(Debug, Clone, Copy, PartialEq)]
struct LengthField {
    len: B14,
    #[skip]
    __: B2,
}

impl From<&[u8]> for LengthField {
    fn from(bytes: &[u8]) -> Self {
        //TODO 字节序问题？
        //LengthField::from_bytes([bytes[0], bytes[1]])
        LengthField::new().with_len(u16::from_le_bytes([bytes[0], bytes[1]]))
    }
}

impl From<LengthField> for Vec<u8> {
    fn from(length: LengthField) -> Self {
        length.into_bytes().to_vec()
    }
}

macro_rules! impl_into_iterator {
    ($type:ty) => {
        impl IntoIterator for $type {
            type Item = u8;
            type IntoIter = std::vec::IntoIter<u8>;
            fn into_iter(self) -> Self::IntoIter {
                Into::<Vec<u8>>::into(self).into_iter()
            }
        }
    };
}

impl_into_iterator!(LengthField);

// ctrl field
#[derive(Debug, Clone, Copy, PartialEq, BitfieldSpecifier)]
enum Dir {
    Client = 0,
    Server = 1,
}

impl std::ops::Not for Dir {
    type Output = Dir;
    fn not(self) -> Self::Output {
        match self {
            Dir::Client => Dir::Server,
            Dir::Server => Dir::Client,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, BitfieldSpecifier)]
enum Prm {
    Server = 0,
    Client = 1,
}

impl std::ops::Not for Prm {
    type Output = Prm;
    fn not(self) -> Self::Output {
        match self {
            Prm::Server => Prm::Client,
            Prm::Client => Prm::Server,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, BitfieldSpecifier)]
#[bits = 3]
pub enum FunctionCode {
    LinkManagement = 1,
    UserData = 3,
}

#[bitfield]
#[derive(Debug, Clone)]
pub struct CtrlField {
    #[bits = 3]
    function_code: FunctionCode,
    is_scramble: bool,
    #[skip]
    __: B1,
    is_fragment: bool,
    prm: Prm,
    dir: Dir,
}

impl CtrlField {
    fn reverse(&mut self) {
        self.set_prm(!self.prm());
        self.set_dir(!self.dir());
    }
}

impl From<CtrlField> for u8 {
    fn from(ctrl_field: CtrlField) -> Self {
        ctrl_field.into_bytes()[0]
    }
}

impl TryFrom<u8> for CtrlField {
    type Error = crate::Error;
    fn try_from(ctrl_field: u8) -> Result<Self> {
        Ok(CtrlField::from_bytes([ctrl_field]))
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
    addr: Vec<u8>,
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
            (address.server_addr.addr_type << 6)
                | (address.server_addr.logic_addr << 4)
                | ((address.server_addr.addr.len() - 1) as u8 & 0x0f),
        );
        bytes.extend(address.server_addr.addr.iter().rev());
        bytes.push(address.client_addr);
        bytes
    }
}

impl_into_iterator!(AddressField);

#[derive(Debug, Clone)]
pub struct Header {
    start: u8,
    length_field: LengthField,
    control_field: CtrlField,
    address_field: AddressField,
    checksum: u16,
}

impl Header {
    pub fn new(control_field: CtrlField, address_field: AddressField) -> Self {
        Self {
            start: HEADER_START,
            length_field: LengthField::new(), // 默认0 后续构造frame时计算
            control_field,
            address_field,
            checksum: 0,
        }
    }

    fn calculate_checksum(&mut self) {
        let mut bytes = Vec::with_capacity(self.bytes_len() - 1 - CHECKSUM_SIZE);
        bytes.extend(self.length_field);
        bytes.extend(self.control_field.clone());
        bytes.extend(self.address_field.clone());
        self.checksum = calculate_fcs16(bytes.as_slice());
    }

    fn _into_vec(self) -> Vec<u8> {
        self.into()
    }

    fn bytes_len(&self) -> usize {
        1 + LENGTH_FIELD_SIZE + CONTROL_FIELD_SIZE + self.address_field.bytes_len() + CHECKSUM_SIZE
    }

    fn is_fragment(&self) -> bool {
        self.control_field.is_fragment()
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
        let expect_checksum = calculate_fcs16(&bytes[1..offset]);
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

impl_into_iterator!(Header);

// tail
const TAIL_END: u8 = 0x16;
#[derive(Debug, Clone)]
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

impl_into_iterator!(Tail);

// user_data
#[derive(Debug, Default, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum FragmentTag {
    #[default]
    Start = 0,
    End = 1,
    Confirm = 2,
    Middle = 3,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct FormatDomain {
    tag: FragmentTag,
    index: u16,
}

impl FormatDomain {
    fn next_index(&self) -> u16 {
        (self.index + 1) % 0x1000
    }
}

impl From<u16> for FormatDomain {
    fn from(format: u16) -> Self {
        FormatDomain {
            tag: (((format & 0xc000) >> 14) as u8).try_into().unwrap(),
            index: format & 0x0fff,
        }
    }
}

impl From<FormatDomain> for u16 {
    fn from(format: FormatDomain) -> Self {
        (format.tag as u16) << 14 | format.index
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
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
pub enum UserData {
    Apdu(Apdu),
    Fragment(ApduFragment),
}

impl UserData {
    fn new(is_fragment: bool, bytes: &[u8]) -> Result<(&[u8], Self)> {
        if is_fragment {
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

    fn _into_vec(self) -> Vec<u8> {
        self.into()
    }
}

impl From<UserData> for Vec<u8> {
    fn from(user_data: UserData) -> Self {
        match user_data {
            UserData::Apdu(apdu) => apdu.to_axdr_vec().unwrap(),
            UserData::Fragment(fragment) => fragment.into(),
        }
    }
}

impl_into_iterator!(UserData);

// frame
const LENGTH_FIELD_SIZE: usize = 2;
const CONTROL_FIELD_SIZE: usize = 1;
const CHECKSUM_SIZE: usize = 2;
const TAIL_SIZE: usize = 3;

#[derive(Debug)]
pub struct Frame {
    pub header: Header,
    pub user_data: UserData,
    pub tail: Tail,
}

impl Frame {
    pub fn new(control_field: CtrlField, address_field: AddressField, user_data: UserData) -> Self {
        let header = Header::new(control_field, address_field);
        let mut frame = Self {
            header,
            user_data,
            tail: Tail {
                checksum: 0,
                end: TAIL_END,
            },
        };
        frame.calculate_length();
        frame.calculate_checksum();
        frame
    }

    pub fn new_server_link_request(address_field: AddressField, user_data: UserData) -> Self {
        let control_field = CtrlField::new()
            .with_function_code(FunctionCode::LinkManagement)
            .with_prm(Prm::Server)
            .with_dir(Dir::Server);
        Self::new(control_field, address_field, user_data)
    }

    pub fn new_server_data_response(address_field: AddressField, user_data: UserData) -> Self {
        let control_field = CtrlField::new()
            .with_function_code(FunctionCode::UserData)
            .with_prm(Prm::Client)
            .with_dir(Dir::Server);
        Self::new(control_field, address_field, user_data)
    }

    pub fn is_server_request(&self) -> bool {
        self.header.control_field.prm() == Prm::Server
            && self.header.control_field.dir() == Dir::Server
    }

    pub fn is_client_response(&self) -> bool {
        self.header.control_field.prm() == Prm::Server
            && self.header.control_field.dir() == Dir::Client
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Option<Self>> {
        //println!("parse frame: {}", hex::encode(src.get_ref()));
        let end = src.get_ref().len();

        // header 不匹配的则直接移除
        while src.position() < end as u64 && src.get_ref()[src.position() as usize] != HEADER_START
        {
            src.set_position(src.position() + 1);
        }

        // 判断长度
        let start = src.position() as usize;
        if end - start < 1 + LENGTH_FIELD_SIZE {
            return Ok(None);
        }
        // length不包含起始和结束字符
        let length =
            u16::from_le_bytes(src.get_ref()[1..LENGTH_FIELD_SIZE + 1].try_into()?) as usize + 2;
        if end - start < length {
            return Ok(None);
        }

        src.set_position(src.position() + length as u64);
        Ok(Some(src.get_ref()[start..start + length].try_into()?))
    }

    pub fn fragment_response(&self) -> Result<Frame> {
        match &self.user_data {
            UserData::Apdu(_) => Err(FrameError::InvalidApduType.into()),
            UserData::Fragment(fragment) => {
                let response_fragment = ApduFragment {
                    format_domain: FormatDomain {
                        tag: FragmentTag::Confirm,
                        index: fragment.format_domain.index,
                    },
                    fragment: vec![],
                };
                let mut control_field = self.header.control_field.clone();
                control_field.reverse();
                Ok(Frame::new(
                    control_field,
                    self.header.address_field.clone(),
                    UserData::Fragment(response_fragment),
                ))
            }
        }
    }

    pub fn is_fragment(&self) -> bool {
        match &self.user_data {
            UserData::Apdu(_) => false,
            UserData::Fragment(_) => true,
        }
    }

    pub fn is_first_fragment(&self) -> bool {
        match &self.user_data {
            UserData::Apdu(_) => false,
            UserData::Fragment(fragment) => fragment.format_domain.tag == FragmentTag::Start,
        }
    }

    pub fn apdu(self) -> Apdu {
        match self.user_data {
            UserData::Apdu(apdu) => apdu,
            _ => unreachable!(),
        }
    }

    // 对于接收到的fragment进行合并 返回是否是最后一片
    pub fn combine_fragment(&mut self, frame: Frame) -> Result<bool> {
        ensure!(self.header.is_fragment(), "invalid fragment frame");
        match (&mut self.user_data, frame.user_data) {
            (UserData::Fragment(fragment1), UserData::Fragment(fragment)) => {
                if fragment1.format_domain.next_index() != fragment.format_domain.index {
                    return Err(FrameError::MismatchFragmentIndex.into());
                }
                match fragment.format_domain.tag {
                    FragmentTag::Confirm | FragmentTag::Start => {
                        Err(FrameError::InvalidFragmentTag.into())
                    }
                    FragmentTag::Middle | FragmentTag::End => {
                        fragment1.format_domain = fragment.format_domain;
                        fragment1.fragment.extend(fragment.fragment);
                        Ok(fragment1.format_domain.tag == FragmentTag::End)
                    }
                }
            }
            _ => Err(FrameError::InvalidApduType.into()),
        }
    }

    pub fn fragment_transfer(&mut self) -> Result<Frame> {
        match &self.user_data {
            UserData::Apdu(_) => unreachable!(),
            UserData::Fragment(fragment) => {
                let (_, user_data) = UserData::new(false, fragment.fragment.as_slice())?;
                let mut frame = Frame {
                    header: self.header.clone(),
                    user_data,
                    tail: self.tail.clone(),
                };
                frame.header.control_field.set_is_fragment(false);
                frame.calculate_length();
                frame.calculate_checksum();
                Ok(frame)
            }
        }
    }

    fn calculate_length(&mut self) {
        const FRAME_SIZE: usize =
            LENGTH_FIELD_SIZE + CONTROL_FIELD_SIZE + CHECKSUM_SIZE + TAIL_SIZE - 1;
        let length =
            self.header.address_field.bytes_len() + self.user_data.bytes_len() + FRAME_SIZE;
        self.header.length_field.set_len(length as u16); // set_len_checked
        self.header.calculate_checksum();
    }

    fn calculate_checksum(&mut self) {
        let mut bytes =
            Vec::with_capacity(self.header.bytes_len() + self.user_data.bytes_len() + TAIL_SIZE);
        bytes.extend(self.header.clone().into_iter().skip(1)); // 去掉帧起始字符
        bytes.extend(self.user_data.to_vec());
        self.tail.checksum = calculate_fcs16(bytes.as_slice());
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.header.clone());
        bytes.extend(self.user_data.to_vec());
        bytes.extend(self.tail.clone());
        bytes
    }
}

impl TryFrom<&[u8]> for Frame {
    type Error = crate::Error;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        let slice = bytes;
        let header = Header::try_from(bytes)?;
        let (bytes, user_data) = UserData::new(header.is_fragment(), &bytes[header.bytes_len()..])?;
        ensure!(bytes.len() >= TAIL_SIZE, FrameError::Length(slice.len()));
        let checksum = u16::from_le_bytes([bytes[0], bytes[1]]);
        let expect_checksum = calculate_fcs16(&slice[1..slice.len() - TAIL_SIZE]);
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

impl From<Frame> for Vec<u8> {
    fn from(frame: Frame) -> Self {
        let mut bytes = Vec::new();
        bytes.extend(frame.header);
        bytes.extend(frame.user_data);
        bytes.extend(frame.tail);
        bytes
    }
}

impl_into_iterator!(Frame);

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
    #[error("invalid apdu type")]
    InvalidApduType,
    #[error("mismatch fragment index")]
    MismatchFragmentIndex,
    #[error("invalid fragment tag")]
    InvalidFragmentTag,
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
        let bytes = hex::decode("681700430502000000000010485e050115f1010200001a0016").unwrap();
        let frame = Frame::try_from(bytes.as_slice()).unwrap();

        //println!("{:#?}", frame);
        assert_eq!(frame.header.length_field.len(), 0x17);
        assert_eq!(frame.header.control_field.dir(), Dir::Client);
        assert_eq!(frame.header.control_field.prm(), Prm::Client);

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
        let bytes = hex::decode("681700430502000000000010485e050115f1010200001a0016").unwrap();

        let apdu = ClientApdu::new(
            ClientApplicationService::GetRequest(GetRequest::GetRequestNormal(
                GetRequestNormal::new(0x15, OAD::new(0xf101, 0x02, 0x00)),
            )),
            None,
        );
        let frame = Frame::new(
            CtrlField::new()
                .with_dir(Dir::Client)
                .with_prm(Prm::Client)
                .with_is_fragment(false)
                .with_function_code(FunctionCode::UserData),
            AddressField::new(
                ServerAddr::new(0, 0, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x02]),
                0x10,
            ),
            UserData::Apdu(Apdu::ClientApdu(apdu)),
        );

        assert_eq!(Into::<Vec<u8>>::into(frame), bytes);
    }

    #[test]
    fn test_frame_fragment_response() {
        let bytes = hex::decode("681700a30502000000000010c58b050115f1010200001a0016").unwrap();
        let frame = Frame::try_from(bytes.as_slice()).unwrap();

        let response = frame.fragment_response().unwrap();

        assert_eq!(
            "681100630502000000000010065c0581e71716",
            hex::encode(Into::<Vec<u8>>::into(response))
        );
    }

    #[test]
    fn test_frame_fragment_combine() {
        let first = hex::decode("681400a3050200000000001076750501050115d6d116").unwrap();
        let mut first = Frame::try_from(first.as_slice()).unwrap();
        let middle = hex::decode("681400a30502000000000010767506f1f101025a3016").unwrap();
        let middle = Frame::try_from(middle.as_slice()).unwrap();
        let middle = match middle.user_data {
            UserData::Fragment(f) => f,
            _ => unreachable!(),
        };
        let tail = hex::decode("681300a3050200000000001090d50741000004fc16").unwrap();
        let tail = Frame::try_from(tail.as_slice()).unwrap();
        let tail = match tail.user_data {
            UserData::Fragment(f) => f,
            _ => unreachable!(),
        };

        assert!(!first.combine_fragment(middle).unwrap());
        assert!(first.combine_fragment(tail).unwrap());
        first.header.control_field.set_is_fragment(false);
        let f = match first.user_data {
            UserData::Fragment(f) => f,
            _ => unreachable!(),
        };

        let (_, user_data) = UserData::new(false, f.fragment.as_slice()).unwrap();
        first.user_data = user_data;

        first.calculate_length();
        first.calculate_checksum();

        //println!("{}", hex::encode(Into::<Vec<u8>>::into(first)));
        assert_eq!(
            "681700830502000000000010fc7c050115f1010200001a0016",
            hex::encode(Into::<Vec<u8>>::into(first))
        );
    }
}

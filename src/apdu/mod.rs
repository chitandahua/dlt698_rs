pub mod data_unit;
pub mod protocol;

use crate::apdu::protocol::{ClientApdu, LinkApdu, SecurityApdu, ServerApdu};

#[derive(Debug, PartialEq)]
pub enum Apdu {
    Link(LinkApdu),
    Client(ClientApdu),
    Server(ServerApdu),
    Security(SecurityApdu),
}

impl asn1_type::traits::ToAxdr for Apdu {
    fn to_axdr_len(&self) -> asn1_type::Result<usize> {
        match self {
            Apdu::Link(link_apdu) => link_apdu.to_axdr_len(),
            Apdu::Client(client_apdu) => client_apdu.to_axdr_len(),
            Apdu::Server(server_apdu) => server_apdu.to_axdr_len(),
            Apdu::Security(security_apdu) => security_apdu.to_axdr_len(),
        }
    }

    fn write_axdr_header(
        &self,
        writer: &mut dyn std::io::Write,
    ) -> asn1_type::SerializeResult<usize> {
        match self {
            Apdu::Link(link_apdu) => link_apdu.write_axdr_header(writer),
            Apdu::Client(client_apdu) => client_apdu.write_axdr_header(writer),
            Apdu::Server(server_apdu) => server_apdu.write_axdr_header(writer),
            Apdu::Security(security_apdu) => security_apdu.write_axdr_header(writer),
        }
    }

    fn write_axdr_content(
        &self,
        writer: &mut dyn std::io::Write,
    ) -> asn1_type::SerializeResult<usize> {
        match self {
            Apdu::Link(link_apdu) => link_apdu.write_axdr_content(writer),
            Apdu::Client(client_apdu) => client_apdu.write_axdr_content(writer),
            Apdu::Server(server_apdu) => server_apdu.write_axdr_content(writer),
            Apdu::Security(security_apdu) => security_apdu.write_axdr_content(writer),
        }
    }
}

impl asn1_type::traits::FromAxdr<'_> for Apdu {
    fn from_axdr(bytes: &[u8]) -> asn1_type::ParseResult<Self> {
        // 依次解析
        if let Ok((bytes, link_apdu)) = LinkApdu::from_axdr(bytes) {
            Ok((bytes, Apdu::Link(link_apdu)))
        } else if let Ok((bytes, client_apdu)) = ClientApdu::from_axdr(bytes) {
            Ok((bytes, Apdu::Client(client_apdu)))
        } else if let Ok((bytes, server_apdu)) = ServerApdu::from_axdr(bytes) {
            Ok((bytes, Apdu::Server(server_apdu)))
        } else if let Ok((bytes, security_apdu)) = SecurityApdu::from_axdr(bytes) {
            Ok((bytes, Apdu::Security(security_apdu)))
        } else {
            Err(asn1_type::Error::InvalidTag.into())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apdu::data_unit::DateTime;
    use crate::apdu::protocol::{LinkRequest, RequestType};
    use asn1_type::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_apdu_to_axdr() {
        let (_, datetime) =
            DateTime::from_axdr(&[0x07, 0xe6, 0x01, 0x01, 0x01, 0x06, 0x21, 0x10, 0x12, 0x34])
                .unwrap();
        let apdu = Apdu::Link(LinkApdu::LinkRequest(LinkRequest::new(
            0x10,
            RequestType::Heartbeat,
            0x0010,
            datetime,
        )));

        let axdr = apdu.to_axdr_vec().unwrap();
        assert_eq!(hex::encode(axdr), "011001001007e60101010621101234");
    }

    #[test]
    fn test_apdu_from_axdr() {
        let (_, datetime) =
            DateTime::from_axdr(&[0x07, 0xe6, 0x01, 0x01, 0x01, 0x06, 0x21, 0x10, 0x12, 0x34])
                .unwrap();
        let result = Apdu::Link(LinkApdu::LinkRequest(LinkRequest::new(
            0x10,
            RequestType::Heartbeat,
            0x0010,
            datetime,
        )));

        let axdr = hex::decode("011001001007e60101010621101234").unwrap();
        let (_, apdu) = Apdu::from_axdr(axdr.as_slice()).unwrap();
        assert_eq!(apdu, result);
    }
}

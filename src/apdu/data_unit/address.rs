use std::any;

use super::TSA;

#[derive(Debug)]
pub struct ServerAddress {
    address_type: u8,
    logic_address: u8,
    address_length: u8,
    address: Vec<u8>,
}

impl ServerAddress {
    fn new(address_type: u8, logic_address: u8, address: Vec<u8>) -> Self {
        ServerAddress {
            address_type,
            logic_address,
            address_length: address.len() as u8,
            address,
        }
    }
}

impl TryFrom<TSA<'_>> for ServerAddress {
    type Error = anyhow::Error;

    fn try_from(value: TSA<'_>) -> Result<Self, Self::Error> {
        let (address_type, logic_address) = (value.as_ref()[0] >> 6, value.as_ref()[1] & 0x30);
        let address_length = value.as_ref()[0] & 0x0f;

        if value.as_ref().len() != address_length as usize + 1 {
            return Err(anyhow::anyhow!("Invalid address length"));
        }

        Ok(ServerAddress::new(
            address_type,
            logic_address,
            value.as_ref()[1..].to_vec(),
        ))
    }
}

use crate::apdu::data_unit::{COMDCB, OAD, PIID, RCSD, RSD, TSA};
use crate::apdu::protocol::action::{ActionRequestAttribute, ActionThenGetRequestNormalAttribute};
use crate::apdu::protocol::set::{SetRequestNormalAttribute, SetThenGetRequestNormalAttribute};
use asn1_type::{LongUnsigned, OctetString, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ProxyGetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetRequestList {
    piid: PIID,
    timeout: LongUnsigned,
    server_attributes: SequenceOf<ProxyGetRequestServerAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyGetRequestServerAttribute {
    server_addr: TSA,
    timeout: LongUnsigned,
    oads: SequenceOf<OAD>,
}

// ProxyGetRequestRecord
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetRequestRecord {
    piid: PIID,
    timeout: LongUnsigned,
    server_addr: TSA,
    oad: OAD,
    rsd: RSD,
    rcsd: RCSD,
}

// ProxySetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetRequestList {
    piid: PIID,
    timeout: LongUnsigned,
    server_attributes: SequenceOf<ProxySetRequestServerAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxySetRequestServerAttribute {
    server_addr: TSA,
    timeout: LongUnsigned,
    oads: SequenceOf<SetRequestNormalAttribute>,
}

// ProxySetThenGetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetThenGetRequestList {
    piid: PIID,
    timeout: LongUnsigned,
    server_attributes: SequenceOf<ProxySetThenGetRequestServerAttribute>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxySetThenGetRequestServerAttribute {
    server_addr: TSA,
    timeout: LongUnsigned,
    attributes: SequenceOf<SetThenGetRequestNormalAttribute>,
}

// ProxyActionRequestList∷
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionRequestList {
    piid: PIID,
    timeout: LongUnsigned,
    server_actions: SequenceOf<ProxyActionRequestServerAction>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionRequestServerAction {
    server_addr: TSA,
    timeout: LongUnsigned,
    actions: SequenceOf<ActionRequestAttribute>,
}

// ProxyActionThenGetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionThenGetRequestList {
    piid: PIID,
    timeout: LongUnsigned,
    server_actions: SequenceOf<ProxyActionThenGetRequestServerAction>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionThenGetRequestServerAction {
    server_addr: TSA,
    timeout: LongUnsigned,
    actions: SequenceOf<ActionThenGetRequestNormalAttribute>,
}

// ProxyTransCommandRequest
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyTransCommandRequest {
    piid: PIID,
    oad: OAD,
    comdcb: COMDCB,
    timeout: LongUnsigned,      // seconds
    byte_timeout: LongUnsigned, // milliseconds
    command: OctetString,
}

use crate::apdu::data_unit::{COMDCB, OAD, PIID, RCSD, RSD, TSA};
use crate::apdu::protocol::action::{ActionRequestAttribute, ActionThenGetRequestNormalAttribute};
use crate::apdu::protocol::set::{SetRequestNormalAttribute, SetThenGetRequestNormalAttribute};
use asn1_type::{LongUnsigned, OctetString, SequenceOf};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

// ProxyGetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetRequestList<'a> {
    piid: PIID,
    timeout: LongUnsigned,
    server_attributes: SequenceOf<ProxyGetRequestServerAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyGetRequestServerAttribute<'a> {
    server_addr: TSA<'a>,
    timeout: LongUnsigned,
    oads: SequenceOf<OAD>,
}

// ProxyGetRequestRecord
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyGetRequestRecord<'a> {
    piid: PIID,
    timeout: LongUnsigned,
    server_addr: TSA<'a>,
    oad: OAD,
    rsd: RSD<'a>,
    rcsd: RCSD,
}

// ProxySetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetRequestList<'a> {
    piid: PIID,
    timeout: LongUnsigned,
    server_attributes: SequenceOf<ProxySetRequestServerAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxySetRequestServerAttribute<'a> {
    server_addr: TSA<'a>,
    timeout: LongUnsigned,
    oads: SequenceOf<SetRequestNormalAttribute<'a>>,
}

// ProxySetThenGetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxySetThenGetRequestList<'a> {
    piid: PIID,
    timeout: LongUnsigned,
    server_attributes: SequenceOf<ProxySetThenGetRequestServerAttribute<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxySetThenGetRequestServerAttribute<'a> {
    server_addr: TSA<'a>,
    timeout: LongUnsigned,
    attributes: SequenceOf<SetThenGetRequestNormalAttribute<'a>>,
}

// ProxyActionRequestList∷
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionRequestList<'a> {
    piid: PIID,
    timeout: LongUnsigned,
    server_actions: SequenceOf<ProxyActionRequestServerAction<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionRequestServerAction<'a> {
    server_addr: TSA<'a>,
    timeout: LongUnsigned,
    actions: SequenceOf<ActionRequestAttribute<'a>>,
}

// ProxyActionThenGetRequestList
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyActionThenGetRequestList<'a> {
    piid: PIID,
    timeout: LongUnsigned,
    server_actions: SequenceOf<ProxyActionThenGetRequestServerAction<'a>>,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
struct ProxyActionThenGetRequestServerAction<'a> {
    server_addr: TSA<'a>,
    timeout: LongUnsigned,
    actions: SequenceOf<ActionThenGetRequestNormalAttribute<'a>>,
}

// ProxyTransCommandRequest
#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ProxyTransCommandRequest<'a> {
    piid: PIID,
    oad: OAD,
    comdcb: COMDCB,
    timeout: LongUnsigned,      // seconds
    byte_timeout: LongUnsigned, // milliseconds
    command: OctetString<'a>,
}

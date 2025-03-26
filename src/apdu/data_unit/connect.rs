use asn1_type::{Null, OctetString, VisibleString};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ConnectMechanismInfo {
    #[tag(0)]
    NullSecurity(Null),
    #[tag(1)]
    PasswordSecurity(VisibleString),
    #[tag(2)]
    SymmetrySecurity {
        data: OctetString,
        signature: OctetString,
    },
    #[tag(3)]
    SignatureSecurity {
        data: OctetString,
        signature: OctetString,
    },
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum ConnectResult {
    #[tag(0)]
    Allow,
    #[tag(1)]
    PasswordError,
    #[tag(2)]
    SymmetryError,
    #[tag(3)]
    AsymmetryError,
    #[tag(4)]
    SignatureError,
    #[tag(5)]
    ProtocolVersionError,
    #[tag(255)]
    OtherError,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct ConnectResponseInfo {
    result: ConnectResult,
    security_data: Option<OctetString>,
}

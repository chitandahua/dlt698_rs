use asn1_type::Unsigned;
use axdr_macro::{AxdrSequence, ToAxdrSequence};

mod attribute;
pub use attribute::{OAD, OI, ROAD};

mod selector;
pub use selector::{CSD, RCSD, RSD};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct OMD {
    oi: OI,
    method_tag: Unsigned,
    mode: Unsigned,
}

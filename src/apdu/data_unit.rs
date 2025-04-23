use asn1_type::{FixedOctetString, OctetString, Unsigned};

mod com;
pub use com::COMDCB;

mod connect;
pub use connect::*;

mod dar;
pub use dar::DAR;

mod data;
pub use data::Data;

mod date_time;
pub use date_time::*;

mod descriptor;
pub use descriptor::*;

mod ms;
pub use ms::MS;

mod region;
pub use region::{Region, TI};

mod security;
pub use security::{MAC, RN, SID, SIDMAC};

pub type PIID = Unsigned;
#[allow(non_camel_case_types)]
pub type PIID_ACD = Unsigned;

use axdr_macro::{AxdrSequence, ToAxdrSequence};
#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct Float32(pub FixedOctetString<4>);
#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct Float64(pub FixedOctetString<8>);

#[derive(Debug, PartialEq, Eq, ToAxdrSequence, AxdrSequence)]
pub struct TSA(pub OctetString);

impl TSA {
    pub fn new(data: &[u8]) -> Self {
        Self(OctetString::new(data))
    }
}

use modular_bitfield::prelude::*;
use std::sync::atomic::{AtomicU8, Ordering};

static PIID_SEQ: AtomicU8 = AtomicU8::new(0);
static PIID_ACD_SEQ: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, BitfieldSpecifier)]
pub enum ServicePriority {
    Normal = 0,
    High = 1,
}

#[bitfield]
pub struct PiidAcd {
    pub index: B6,
    pub is_request_acd: bool,
    pub priority: ServicePriority,
}

impl PiidAcd {
    pub fn with_piid(piid: &Piid) -> Self {
        Self::new()
            .with_index(piid.index())
            .with_is_request_acd(false)
            .with_priority(piid.priority())
    }

    pub fn new_req() -> Self {
        Self::new().with_index(PIID_ACD_SEQ.fetch_add(1, Ordering::SeqCst) % 64)
    }

    pub fn match_piid(&self, piid: &Piid) -> bool {
        self.index() == piid.index()
    }
}

impl From<PiidAcd> for PIID_ACD {
    fn from(value: PiidAcd) -> Self {
        value.into_bytes()[0]
    }
}

#[bitfield]
pub struct Piid {
    pub index: B6,
    #[skip]
    __: B1,
    pub priority: ServicePriority,
}

impl Piid {
    pub fn new_req() -> Self {
        Self::new().with_index(PIID_SEQ.fetch_add(1, Ordering::SeqCst) % 64)
    }

    pub fn match_piid_acd(&self, piid: &PiidAcd) -> bool {
        self.index() == piid.index()
    }
}

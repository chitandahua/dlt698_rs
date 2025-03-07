use super::attribute::{OAD, ROAD};
use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug)]
enum CSD {
    //#[tag(0)]
    Oad(OAD),
    //#[tag(1)]
    Road(ROAD),
}

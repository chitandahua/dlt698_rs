use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum BaudRate {
    #[tag(0)]
    B300,
    #[tag(1)]
    B600,
    #[tag(2)]
    B1200,
    #[tag(3)]
    B2400,
    #[tag(4)]
    B4800,
    #[tag(5)]
    B7200,
    #[tag(6)]
    B9600,
    #[tag(7)]
    B19200,
    #[tag(8)]
    B38400,
    #[tag(9)]
    B57600,
    #[tag(10)]
    B115200,
    #[tag(255)]
    Auto,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum Parity {
    #[tag(0)]
    None,
    #[tag(1)]
    Odd,
    #[tag(2)]
    Even,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum DataBits {
    #[tag(5)]
    Five,
    #[tag(6)]
    Six,
    #[tag(7)]
    Seven,
    #[tag(8)]
    Eight,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum StopBits {
    #[tag(1)]
    One,
    #[tag(2)]
    Two,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub enum FlowControl {
    #[tag(0)]
    None,
    #[tag(1)]
    Hardware,
    #[tag(2)]
    Software,
}

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence)]
pub struct COMDCB {
    baudrate: BaudRate,
    parity: Parity,
    data_bits: DataBits,
    stop_bits: StopBits,
    flow_control: FlowControl,
}

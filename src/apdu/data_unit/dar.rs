use axdr_macro::{AxdrSequence, ToAxdrSequence};

#[derive(Debug, PartialEq, Eq, AxdrSequence, ToAxdrSequence, Clone, Copy)]
#[repr(u8)]
pub enum DAR {
    #[tag(0)]
    Success,
    #[tag(1)]
    HardwareFailure,
    #[tag(2)]
    TemporaryFailure,
    #[tag(3)]
    Denied,
    #[tag(4)]
    ObjectUndefined,
    #[tag(5)]
    InterfaceClassMismatch,
    #[tag(6)]
    ObjectNotExist,
    #[tag(7)]
    TypeMismatch,
    #[tag(8)]
    OutOfRange,
    #[tag(9)]
    DataBlockUnavailable,
    #[tag(10)]
    FrameTransferCanceled,
    #[tag(11)]
    NotInFrameTransferState,
    #[tag(12)]
    BlockWriteCanceled,
    #[tag(13)]
    BlockWriteStateNotExist,
    #[tag(14)]
    InvalidBlockNumber,
    #[tag(15)]
    PasswordError,
    #[tag(16)]
    CannotChangeBaudRate,
    #[tag(17)]
    InvalidYear,
    #[tag(18)]
    InvalidDay,
    #[tag(19)]
    InvalidRate,
    #[tag(20)]
    SecurityMismatch,
    #[tag(21)]
    DuplicateRecharge,
    #[tag(22)]
    ESAMError,
    #[tag(23)]
    SecurityError,
    #[tag(24)]
    InvalidCustomerID,
    #[tag(25)]
    InvalidRechargeTimes,
    #[tag(26)]
    Overload,
    #[tag(27)]
    InvalidAddress,
    #[tag(28)]
    DecryptionError,
    #[tag(29)]
    NonDecryptionError,
    #[tag(30)]
    SignatureError,
    #[tag(31)]
    Suspended,
    #[tag(32)]
    InvalidTimeTag,
    #[tag(33)]
    Timeout,
    #[tag(34)]
    InvalidP1P2,
    #[tag(35)]
    InvalidLC,
    #[tag(255)]
    Other,
}

impl<T, E> From<std::result::Result<T, E>> for DAR
where
    E: Into<DAR>,
{
    fn from(result: std::result::Result<T, E>) -> Self {
        match result {
            Ok(_) => DAR::Success,
            Err(t) => t.into(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    AccountNotFound,
    InstrumentNotFound,
    DumplicateTrade,
    FrontDisconnected,
    CtpAuthFailed,
    MpscSendErr,
    InvalidCtpInstrumentId
}

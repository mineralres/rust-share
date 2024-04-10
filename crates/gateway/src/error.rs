#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    AccountNotFound,
    InstrumentNotFound,
    DumplicateTrade,
    FrontDisconnected,
    CtpAuthFailed,
    MpscSendErr,
    InvalidCtpInstrumentId,
    CtpLastQueryIsProceeding,
    CtpQueryTimeout,
    InvalidSymbol,
    MdNotFound,
    CtpApiErr(i32),
    SimpleErr(simple_error::SimpleError),
    OrderNotFound,
}

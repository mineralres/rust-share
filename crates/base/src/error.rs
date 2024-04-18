#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    AccountNotFound,
    InstrumentNotFound,
    DumplicateTrade,
    FrontDisconnected,
    CtpAuthFailed,
    LoginFailed,
    MpscSendErr,
    InvalidCtpInstrumentId,
    CtpLastQueryIsProceeding,
    CtpQueryTimeout,
    InvalidSymbol,
    MdNotFound,
    TraderApiErr(i32),
    SimpleErr(simple_error::SimpleError),
    OrderNotFound,
    ShareholderAccountNotFound,
    InvalidRouteType,
    QueryTimeout
}

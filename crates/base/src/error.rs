#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    AccountNotFound,
    #[display(fmt = "InstrumentNotFound {}:{}", exchange, symbol)]
    InstrumentNotFound { exchange: String, symbol: String },
    DumplicateTrade,
    FrontDisconnected,
    CtpAuthFailed,
    InitTraderFailed,
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
    QueryTimeout,
    MdNotValid,
    TraderNotValid,
}

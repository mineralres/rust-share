pub mod error;
pub mod state;
pub mod util;

pub fn init_logger() {
    use time::{macros::format_description, UtcOffset};
    use tracing_subscriber::fmt::time::OffsetTime;
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt().with_timer(local_time).init();
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct UniqueSymbol {
    pub exchange: String,
    pub symbol: String,
}

impl UniqueSymbol {
    pub fn new(exchange: &str, symbol: &str) -> Self {
        Self {
            exchange: exchange.into(),
            symbol: symbol.into(),
        }
    }
}

/// 交易账号
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct TradingAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub route_type: String,
    pub name_servers: Vec<String>,
    pub trade_fronts: Vec<String>,
    pub md_fronts: Vec<String>,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
    pub remark: String,
    pub money_password: String,
    pub terminal_info: String,
    pub hd_serial: String,
    pub inner_ip_address: String,
    pub mac_address: String,
    pub query_fronts: Vec<String>,
    pub fens_trade_fronts: Vec<String>,
    pub fens_md_fronts: Vec<String>,
    pub dynamic_password: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ExecutorConfig {
    // 开启
    pub enabled: bool,
    // 端口
    pub http_port: u16,
    // https端口
    pub https_port: u16,
    // 类型 ctp_futures, tora_stock
    pub r#type: String,
    // 账户
    pub accounts: Vec<TradingAccountConfig>,
    // 行情源
    pub md_account: TradingAccountConfig,
    // 开启后只允许一个下单源锁定，排他性下单
    pub lock_check: bool,
    // 排他性锁定的ttl
    pub lock_credential_ttl: i64,
    // 锁定时用的token
    pub token: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct LoggerConfig {
    pub host: String,
    pub token: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GatewayConfig {
    pub executors: Vec<ExecutorConfig>,
    pub logger: LoggerConfig,
}

impl GatewayConfig {
    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let c = serde_json::from_reader(reader)?;
        Ok(c)
    }
}

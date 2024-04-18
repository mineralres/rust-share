use base::util::*;
use base::*;
use bincode::config;
use ctp_futures::query::*;
use ctp_futures::*;
use log::info;
use std::ffi::CStr;
use std::io::Write;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() {
    init_logger();
    // let trade_front = "tcp://180.168.146.187:10130"; // 7*24
    let account = TradingAccountConfig {
        broker_id: "9999".to_string(),
        account: "143650".to_string(),
        trade_fronts: vec!["tcp://180.168.146.187:10201".to_string()],
        md_fronts: vec!["180.168.146.187:10211".to_string()],
        name_servers: vec![],
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "198612".to_string(),
        remark: "".into(),
        fens_md_fronts: vec![],
        fens_trade_fronts: vec![],
        hd_serial: "".into(),
        inner_ip_address: "".into(),
        mac_address: "".into(),
        money_password: "".into(),
        route_type: "ctp_futures".into(),
        query_fronts: vec![],
        terminal_info: "".into(),
    };
    let result = query(&account).await.unwrap();
    let config = config::standard();
    let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
    let save_path = std::path::Path::new(".cache")
        .join("ctp_futures_query_result")
        .join(format!("{}_{}", account.broker_id, account.account));
    info!("save_path = {:?}", save_path);
    check_make_dir(save_path.to_str().unwrap());
    let save_path = save_path.join(format!("{}.dat", result.trading_day));
    info!("path={:?}", save_path);
    let mut f = std::fs::File::create(&save_path).unwrap();
    f.write_all(&encoded).unwrap();
    info!("{} 查询完成. bin.len={}", account.account, encoded.len());
    let (decoded, _len): (CtpQueryResult, usize) =
        bincode::decode_from_slice(&encoded[..], config).unwrap();
    info!(
        "decoded {} {} {}",
        decoded.broker_id, decoded.account, decoded.trading_day
    );
    let ver = unsafe { CStr::from_ptr(trader_api::get_api_version()) }
        .to_str()
        .unwrap();
    info!("version={ver}");
}

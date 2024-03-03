use std::io::Write;

use bincode::{config, Decode, Encode};
use ctp_futures::*;
use futures::StreamExt;
use log::info;
use rust_share_util::*;

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
    let account = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: "143650".to_string(),
        trade_fronts: vec!["tcp://180.168.146.187:10201".to_string()],
        md_fronts: vec!["tcp://180.168.146.187:10211".to_string()],
        // md_fronts: vec!["tcp://180.168.146.187:10131".to_string()],
        name_servers: vec!["".to_string()],
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "198612".to_string(),
        remark: "".into(),
    };
    md(&account).await;
}

async fn md(ca: &CtpAccountConfig) {
    use ctp_futures::md_api::*;
    let broker_id = ca.broker_id.as_str();
    let account = ca.account.as_str();
    let md_front = ca.md_fronts[0].as_str();
    let auth_code = ca.auth_code.as_str();
    let user_product_info = ca.user_product_info.as_str();
    let app_id = ca.app_id.as_str();
    let password = ca.password.as_str();
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    let flow_path = format!(".cache/ctp_futures_md_flow_{}_{}//", broker_id, account);
    check_make_dir(&flow_path);
    let mut api = create_api(&flow_path, false, false);
    let mut stream = {
        let (stream, pp) = create_spi();
        api.register_spi(pp);
        stream
    };
    use std::ffi::CString;
    api.register_front(CString::new(md_front).unwrap());
    info!("register front {}", md_front);
    api.init();
    // 处理登陆初始化查询
    while let Some(spi_msg) = stream.next().await {
        use ctp_futures::md_api::CThostFtdcMdSpiOutput::*;
        match spi_msg {
            OnFrontConnected(_p) => {
                let mut req = CThostFtdcReqUserLoginField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                set_cstr_from_str_truncate_i8(&mut req.Password, password);
                api.req_user_login(&mut req, get_request_id());
                info!("OnFrontConnected");
            }
            OnFrontDisconnected(p) => {
                info!("on front disconnected {:?} 直接Exit ", p);
                std::process::exit(-1);
            }
            OnRspUserLogin(ref p) => {
                info!("OnRspUserLogin");
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let u = p.p_rsp_user_login.unwrap();
                    let mut v = vec![];
                    let mut d = chrono::Local::now();
                    for i in 0..12 {
                        d = d.checked_add_months(chrono::Months::new(1)).unwrap();
                        let symbol = std::ffi::CString::new(format!(
                            "ru{}",
                            d.format("%Y%m").to_string().trim_start_matches("20")
                        ))
                        .unwrap();
                        v.push(symbol);
                    }
                    let count = v.len() as i32;
                    info!("symbols={:?}", v);
                    let ret = api.subscribe_market_data(v, count);
                    info!("Subscribe ret = {}", ret);
                } else {
                    info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                }
            }
            OnRtnDepthMarketData(ref md) => {
                info!("md={:?}", md);
            }
            _ => {}
        }
    }
    api.release();
    api.join();
    // Box::leak(api);

    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    info!("完成保存查询结果");
}

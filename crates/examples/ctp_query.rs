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
        trade_front: "tcp://180.168.146.187:10201".to_string(),
        md_front: "180.168.146.187:10211".to_string(),
        name_server: "".to_string(),
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "198612".to_string(),
    };
    query(&account).await;
}

#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct CtpQueryResult {
    broker_id: String,
    account: String,
    trading_day: i32,
    timestamp: i64,
    dmd_list: Vec<CThostFtdcDepthMarketDataField>,
    icr_list: Vec<CThostFtdcInstrumentCommissionRateField>,
    instrument_list: Vec<CThostFtdcInstrumentField>,
    position_list: Vec<CThostFtdcInvestorPositionField>,
    position_detail_list: Vec<CThostFtdcInvestorPositionDetailField>,
    trading_account: CThostFtdcTradingAccountField,
    product_list: Vec<CThostFtdcProductField>,
    order_list: Vec<CThostFtdcOrderField>,
    trade_list: Vec<CThostFtdcTradeField>,
}

async fn query(ca: &CtpAccountConfig) {
    use ctp_futures::trader_api::*;
    let broker_id = ca.broker_id.as_str();
    let account = ca.account.as_str();
    let trade_front = ca.trade_front.as_str();
    let name_server = ca.name_server.as_str();
    let auth_code = ca.auth_code.as_str();
    let user_product_info = ca.user_product_info.as_str();
    let app_id = ca.app_id.as_str();
    let password = ca.password.as_str();
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    let flow_path = format!(".cache/ctp_futures_trade_flow_{}_{}//", broker_id, account);
    check_make_dir(&flow_path);
    let mut api = create_api(&flow_path, false);
    let mut stream = {
        let (stream, pp) = create_spi();
        api.register_spi(pp);
        stream
    };
    use std::ffi::CString;
    if name_server.len() > 0 {
        api.register_name_server(CString::new(name_server).unwrap());
    } else {
        api.register_front(CString::new(trade_front).unwrap());
        info!("register front {}", trade_front);
    }
    api.subscribe_public_topic(ctp_futures::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
    api.subscribe_private_topic(ctp_futures::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
    api.init();
    let mut result = CtpQueryResult::default();
    result.broker_id = broker_id.to_string();
    result.account = account.to_string();
    // 处理登陆初始化查询
    while let Some(spi_msg) = stream.next().await {
        use ctp_futures::trader_api::CThostFtdcTraderSpiOutput::*;
        match spi_msg {
            OnFrontConnected(_p) => {
                let mut req = CThostFtdcReqAuthenticateField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                set_cstr_from_str_truncate_i8(&mut req.AuthCode, auth_code);
                set_cstr_from_str_truncate_i8(&mut req.UserProductInfo, user_product_info);
                set_cstr_from_str_truncate_i8(&mut req.AppID, app_id);
                api.req_authenticate(&mut req, get_request_id());
                info!("OnFrontConnected");
            }
            OnFrontDisconnected(p) => {
                info!("on front disconnected {:?} 直接Exit ", p);
                std::process::exit(-1);
            }
            OnRspAuthenticate(ref p) => {
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let mut req = CThostFtdcReqUserLoginField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                    set_cstr_from_str_truncate_i8(&mut req.Password, password);
                    api.req_user_login(&mut req, get_request_id());
                } else {
                    info!("RspAuthenticate={:?}", p);
                    std::process::exit(-1);
                }
            }
            OnRspUserLogin(ref p) => {
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let u = p.p_rsp_user_login.unwrap();
                    result.trading_day = trading_day_from_ctp_trading_day(&u.TradingDay);
                } else {
                    info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                }
                let mut req = CThostFtdcSettlementInfoConfirmField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                let result = api.req_settlement_info_confirm(&mut req, get_request_id());
                if result != 0 {
                    info!("ReqSettlementInfoConfirm={}", result);
                }
            }
            OnRspSettlementInfoConfirm(ref _p) => {
                let mut req = CThostFtdcQryTradingAccountField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                let result = api.req_qry_trading_account(&mut req, get_request_id());
                if result != 0 {
                    info!("ReqQueryTradingAccount={}", result);
                }
            }
            OnRspQryTradingAccount(ref p) => {
                if let Some(taf) = p.p_trading_account {
                    result.trading_account = taf;
                    info!(
                        "查询账户资金完成.  account={} trading_day={} balance={}",
                        gb18030_cstr_to_str_i8(&taf.AccountID),
                        gb18030_cstr_to_str_i8(&taf.TradingDay),
                        taf.Balance
                    );
                }
                if p.b_is_last {
                    let mut req = CThostFtdcQryInvestorPositionDetailField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    // flow control query limitation
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let result = api.req_qry_investor_position_detail(&mut req, get_request_id());
                    if result != 0 {
                        info!("ReqQryInvestorPositionDetail = {:?}", result);
                    }
                }
            }
            OnRspQryInvestorPositionDetail(ref detail) => {
                if let Some(d) = detail.p_investor_position_detail {
                    info!("d={:?}", d);
                    result.position_detail_list.push(d);
                }
                if detail.b_is_last {
                    info!(
                        "查询持仓明细完成, len={}",
                        result.position_detail_list.len()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let mut req = CThostFtdcQryInvestorPositionField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    let result = api.req_qry_investor_position(&mut req, get_request_id());
                    if result != 0 {
                        info!("ReqQueryPosition={}", result);
                    }
                }
            }
            OnRspQryInvestorPosition(ref p) => {
                if let Some(p) = p.p_investor_position {
                    result.position_list.push(p);
                }
                if p.b_is_last {
                    info!("查询持仓完成, len={}", result.position_list.len());
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let mut req = CThostFtdcQryInstrumentField::default();
                    let result = api.req_qry_instrument(&mut req, get_request_id());
                    if result != 0 {
                        info!("ReqQryInstrument = {:?}", result);
                    }
                }
            }
            OnRspQryInstrument(ref p) => {
                if let Some(instrument) = p.p_instrument {
                    result.instrument_list.push(instrument);
                }
                if p.b_is_last {
                    // 查询行情
                    info!("查询合约完成, len={}", result.instrument_list.len());
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let mut req = CThostFtdcQryDepthMarketDataField::default();
                    let result = api.req_qry_depth_market_data(&mut req, get_request_id());
                    if result != 0 {
                        info!("ReqQryDepthMarketData= {:?}", result);
                    }
                }
            }
            OnRspQryDepthMarketData(ref p) => {
                if let Some(md) = p.p_depth_market_data {
                    result.dmd_list.push(md);
                }
                if p.b_is_last {
                    info!("查询行情完成 len={}", result.dmd_list.len());
                    let mut req = CThostFtdcQryOrderField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let result = api.req_qry_order(&mut req, get_request_id());
                    if result != 0 {
                        info!("ReqQryOrder = {:?}", result);
                    }
                }
            }

            OnRspQryOrder(ref p) => {
                if let Some(order) = p.p_order {
                    result.order_list.push(order);
                }

                if p.b_is_last {
                    info!("查询委托完成 len={}", result.order_list.len());
                    let mut req = CThostFtdcQryTradeField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, account);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let result = api.req_qry_trade(&mut req, get_request_id());
                    if result != 0 {
                        info!("ReqQryTrade = {:?}", result);
                    }
                }
            }
            OnRspQryTrade(ref p) => {
                if let Some(trade) = p.p_trade {
                    result.trade_list.push(trade);
                }
                if p.b_is_last {
                    info!("查询成交明细完成 len={}", result.trade_list.len());
                    break;
                }
            }
            OnRspQryInstrumentCommissionRate(ref p) => {
                // 未处理
                if let Some(icr) = p.p_instrument_commission_rate {
                    result.icr_list.push(icr);
                }
                if p.b_is_last {}
            }

            _ => {}
        }
    }
    result.timestamp = chrono::Local::now().timestamp();
    let config = config::standard();
    let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
    use std::path::PathBuf;
    let save_path = std::path::Path::new(".cache")
        .join("ctp_futures_query_result")
        .join(format!("{}_{}", broker_id, account));
    info!("save_path = {:?}", save_path);
    check_make_dir(save_path.to_str().unwrap());
    let save_path = save_path.join(format!("{}.dat", result.trading_day));
    info!("path={:?}", save_path);
    let mut f = std::fs::File::create(&save_path).unwrap();
    f.write_all(&encoded).unwrap();
    info!("{} 初始化查询完成. bin.len={}", account, encoded.len());
    let (decoded, len): (CtpQueryResult, usize) =
        bincode::decode_from_slice(&encoded[..], config).unwrap();
    info!(
        "decoded {} {} {}",
        decoded.broker_id, decoded.account, decoded.trading_day
    );
    api.release();
    api.join();
    // Box::leak(api);

    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    info!("完成保存查询结果");
}

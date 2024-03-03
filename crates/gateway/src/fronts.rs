pub mod http {
    use crate::executor::Executor;
    use crate::{error::Error, executor::ReqMessage};
    use axum::{
        extract::{Json, State},
        response::Response,
        routing::any,
        Router,
    };
    use ctp_futures::*;
    use futures::StreamExt;
    use log::info;
    use rust_share_util::*;
    use std::ffi::{CStr, CString};
    use std::io::Write;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use types::*;

    impl axum::response::IntoResponse for Error {
        fn into_response(self) -> Response {
            XResponse {
                data: "".to_string(),
                msg: self.to_string(),
                code: 0,
            }
            .into_response()
        }
    }

    mod types {
        use std::sync::Arc;

        use axum::{extract::Json, response::Response};
        use serde::Serialize;

        use crate::executor::Executor;
        use tokio::sync::Mutex;

        #[derive(Clone)]
        pub struct ShareState {
            pub config: crate::Config,
            pub executor: Arc<Mutex<Executor>>,
        }

        #[derive(Debug, Serialize)]
        pub struct XResponse<T> {
            pub data: T,
            pub msg: String,
            pub code: i32,
        }

        impl<T: serde::Serialize> axum::response::IntoResponse for XResponse<T> {
            fn into_response(self) -> Response {
                Json(self).into_response()
            }
        }

        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct ReqQuery {
            pub broker_id: String,
            pub account: String,
        }
    }

    pub async fn serve(conf: crate::Config, executor: Arc<Mutex<Executor>>) {
        let addr = SocketAddr::from(([0, 0, 0, 0], conf.http_port));
        info!("Http listening on {}", addr);
        let state = ShareState {
            config: conf,
            executor,
        };
        let app = Router::new()
            .route("/api/query", any(query))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("监听错误:");
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }

    async fn query(State(s): State<ShareState>, Json(req): Json<ReqQuery>) -> Result<(), Error> {
        let ca = s
            .config
            .ctp_accounts
            .iter()
            .find(|ca| ca.broker_id == req.broker_id && ca.account == req.account)
            .ok_or(Error::AccountNotFound)?;
        // ctp_query(ca).await;
        let req_msg = ReqMessage::QueryPosition {
            account: req.account.clone(),
        };
        s.executor
            .lock()
            .await
            .send_request_message(&req.account, req_msg)
            .await?;
        Ok(())
    }

    async fn ctp_query(ca: &CtpAccountConfig) {
        info!("ca={:?}", ca);
        use ctp_futures::trader_api::*;
        let broker_id = ca.broker_id.as_str();
        let account = ca.account.as_str();
        let trade_front = ca.trade_fronts[0].as_str();
        // let name_server = ca.name_servers[0].as_str();
        let auth_code = ca.auth_code.as_str();
        let user_product_info = ca.user_product_info.as_str();
        let app_id = ca.app_id.as_str();
        let password = ca.password.as_str();
        let mut request_id: i32 = 0;
        let mut get_request_id = || {
            request_id += 1;
            request_id
        };
        let flow_path = format!(
            ".cache/ctp_futures_trade_query_flow_{}_{}//",
            broker_id, account
        );
        check_make_dir(&flow_path);
        let mut api = create_api(&flow_path, false);
        let mut stream = {
            let (stream, pp) = create_spi();
            api.register_spi(pp);
            stream
        };
        if ca.name_servers.len() > 0 {
            api.register_name_server(CString::new(ca.name_servers[0].as_str()).unwrap());
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
                        let result =
                            api.req_qry_investor_position_detail(&mut req, get_request_id());
                        if result != 0 {
                            info!("ReqQryInvestorPositionDetail = {:?}", result);
                        }
                    }
                }
                OnRspQryInvestorPositionDetail(ref detail) => {
                    if let Some(d) = detail.p_investor_position_detail {
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
                        result.positions.push(p);
                    }
                    if p.b_is_last {
                        info!("查询持仓完成, len={}", result.positions.len());
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
                        result.instruments.push(instrument);
                    }
                    if p.b_is_last {
                        // 查询行情
                        info!("查询合约完成, len={}", result.instruments.len());
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
                        result.orders.push(order);
                    }

                    if p.b_is_last {
                        info!("查询委托完成 len={}", result.orders.len());
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
                        result.trades.push(trade);
                    }
                    if p.b_is_last {
                        info!("查询成交明细完成 len={}", result.trades.len());
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
        let config = bincode::config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
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
        let (decoded, _len): (CtpQueryResult, usize) =
            bincode::decode_from_slice(&encoded[..], config).unwrap();
        info!(
            "decoded {} {} {}",
            decoded.broker_id, decoded.account, decoded.trading_day
        );
        let ver = unsafe { CStr::from_ptr(get_api_version()) }
            .to_str()
            .unwrap();
        info!("version={ver}");
        api.release();
        Box::leak(api);
        info!("完成保存查询结果");
    }
}

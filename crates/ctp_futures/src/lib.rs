#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(unused_variables, unused_mut)]

use bincode::{Decode, Encode};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod md_api {
    use crate::*;
    include!("md_impl.rs");

    unsafe impl Send for MdApi {}

    pub struct MdApi {
        pub api: *mut CThostFtdcMdApi,
    }

    impl Drop for MdApi {
        fn drop(&mut self) {
            let spi: *const CThostFtdcMdSpiStream = std::ptr::null();
            let api = unsafe { &mut (*self.api) };
            api.register_spi(spi);
            api.release();
        }
    }

    impl std::ops::Deref for MdApi {
        type Target = CThostFtdcMdApi;
        fn deref(&self) -> &Self::Target {
            unsafe { &(*self.api) }
        }
    }

    impl std::ops::DerefMut for MdApi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut (*self.api) }
        }
    }

    pub fn create_api(flow_path: &str, b_is_using_udp: bool, b_is_multicast: bool) -> MdApi {
        let md_flow_path = std::ffi::CString::new(flow_path).unwrap();
        let p = unsafe {
            CThostFtdcMdApi_CreateFtdcMdApi(md_flow_path.as_ptr(), b_is_using_udp, b_is_multicast)
        };
        MdApi { api: p }
    }
}

pub mod trader_api {
    use crate::*;
    include!("trade_impl.rs");

    unsafe impl Send for TraderApi {}

    pub struct TraderApi {
        pub api: *mut CThostFtdcTraderApi,
    }

    impl Drop for TraderApi {
        fn drop(&mut self) {
            let spi: *const CThostFtdcTraderSpiStream = std::ptr::null();
            let api = unsafe { &mut (*self.api) };
            api.register_spi(spi);
            api.release();
        }
    }

    impl std::ops::Deref for TraderApi {
        type Target = CThostFtdcTraderApi;
        fn deref(&self) -> &Self::Target {
            unsafe { &(*self.api) }
        }
    }

    impl std::ops::DerefMut for TraderApi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut (*self.api) }
        }
    }

    pub fn create_api(flow_path: &str, b_encrypt: bool) -> TraderApi {
        let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
        let p = unsafe { CThostFtdcTraderApi_CreateFtdcTraderApi(trade_flow_path.as_ptr()) };
        TraderApi { api: p }
    }

    pub fn get_api_version() -> *const std::os::raw::c_char {
        unsafe { CThostFtdcTraderApi_GetApiVersion() }
    }
}

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    InvalidCtpInstrumentId,
}

pub mod route {
    use crate::trader_api::TraderApi;
    use crate::*;
    use base::error::Error;
    use base::state::md_cache::MdCache;
    use base::state::*;
    use base::util::*;
    use base::*;
    use chrono::Local;
    use futures::StreamExt;
    use itertools::Itertools;
    use log::{error, info};
    use std::collections::HashMap;
    use std::ffi::CString;
    use std::sync::Arc;
    use tokio::sync::{mpsc, oneshot, Mutex};

    type AccountStateType = route::CtpAccountState;

    impl std::cmp::PartialEq for CThostFtdcOrderField {
        fn eq(&self, other: &Self) -> bool {
            self.FrontID == other.FrontID
                && self.SessionID == other.SessionID
                && cstr_i8_eq(&self.OrderRef, &other.OrderRef)
        }
    }

    fn cstr_i8_eq(c: &[i8], other: &[i8]) -> bool {
        for (ch, other_ch) in c.iter().zip(other.iter()) {
            if *ch != *other_ch {
                return false;
            }
            if *ch == 0i8 || *other_ch == 0i8 {
                return true;
            }
        }
        true
    }

    pub type CtpAccountState = AccountState<CThostFtdcOrderField, CThostFtdcTradeField>;

    impl From<&CThostFtdcInvestorPositionDetailField> for PositionDetail {
        fn from(value: &CThostFtdcInvestorPositionDetailField) -> Self {
            PositionDetail {
                open_date: value.OpenDate.clone(),
                volume: value.Volume,
                direction: to_direction_type(value.Direction),
                exchange: get_ascii_str(&value.ExchangeID).unwrap().into(),
                symbol: get_ascii_str(&value.InstrumentID).unwrap().into(),
                open_price: value.OpenPrice,
                trade_id: value.TradeID.clone(),
            }
        }
    }

    impl TradeType for CThostFtdcTradeField {
        fn volume(&self) -> i32 {
            self.Volume
        }
        fn direction(&self) -> DirectionType {
            to_direction_type(self.Direction)
        }
        fn direction_i8(&self) -> i8 {
            self.Direction
        }
        fn price(&self) -> f64 {
            self.Price
        }
        fn trade_date(&self) -> &[i8; 9] {
            &self.TradeDate
        }
        fn trading_day(&self) -> &[i8; 9] {
            &self.TradingDay
        }
        fn exchange(&self) -> &str {
            get_ascii_str(&self.ExchangeID)
                .expect("CThostFtdcInvestorPositionDetailField.ExchangeID error")
        }
        fn symbol(&self) -> &str {
            get_ascii_str(&self.InstrumentID)
                .expect("CThostFtdcInvestorPositionDetailField.InstrumentID error")
        }
        fn order_sys_id(&self) -> &[i8; 21] {
            &self.OrderSysID
        }
        fn trade_id(&self) -> &[i8; 21] {
            &self.TradeID
        }
        fn offset_flag(&self) -> OffsetFlag {
            match self.OffsetFlag as u8 {
                THOST_FTDC_OF_Open => OffsetFlag::Open,
                THOST_FTDC_OF_CloseToday => OffsetFlag::CloseToday,
                THOST_FTDC_OF_CloseYesterday => OffsetFlag::CloseYesterday,
                THOST_FTDC_OF_Close => OffsetFlag::Close,
                _ => OffsetFlag::OfOther,
            }
        }
        fn to_position_detail(&self) -> PositionDetail {
            PositionDetail {
                open_date: self.trading_day().clone(),
                volume: self.volume(),
                direction: self.direction(),
                exchange: self.exchange().into(),
                symbol: self.symbol().into(),
                open_price: self.price(),
                trade_id: self.trade_id().clone(),
            }
        }
    }

    impl OrderType for CThostFtdcOrderField {
        fn pending_status(&self) -> PendingOrderStatus {
            /*
            ///全部成交
            #define THOST_FTDC_OST_AllTraded '0'
            ///部分成交还在队列中
            #define THOST_FTDC_OST_PartTradedQueueing '1'
            ///部分成交不在队列中
            #define THOST_FTDC_OST_PartTradedNotQueueing '2'
            ///未成交还在队列中
            #define THOST_FTDC_OST_NoTradeQueueing '3'
            ///未成交不在队列中
            #define THOST_FTDC_OST_NoTradeNotQueueing '4'
            ///撤单
            #define THOST_FTDC_OST_Canceled '5'
            ///未知
            #define THOST_FTDC_OST_Unknown 'a'
            ///尚未触发
            #define THOST_FTDC_OST_NotTouched 'b'
            ///已触发
            #define THOST_FTDC_OST_Touched 'c'

            */

            match self.OrderStatus as u8 {
                THOST_FTDC_OST_PartTradedNotQueueing | THOST_FTDC_OST_Canceled => {
                    PendingOrderStatus::Canceled
                }
                THOST_FTDC_OST_AllTraded => PendingOrderStatus::Done,
                _ => PendingOrderStatus::Pending,
            }
        }
        // fn front_id(&self) -> i32 {
        //     self.FrontID
        // }
        // fn session_id(&self) -> i32 {
        //     self.SessionID
        // }
        // fn order_ref(&self) -> &[i8; 13] {
        //     &self.OrderRef
        // }
        fn to_pending_order(&self) -> PendingOrder {
            PendingOrder {
                front_id: self.FrontID,
                session_id: self.SessionID,
                order_ref: self.OrderRef.clone(),
                order_ref_i32: get_ascii_str(&self.OrderRef)
                    .unwrap()
                    .parse::<i32>()
                    .expect("Ctp OrderRef error format"),
                order_sys_id: self.OrderSysID.clone(),
                volume_traded: self.VolumeTraded,
                volume_canceled: self.volume_canceled(),
                volume_total_original: self.VolumeTotalOriginal,
                status: self.pending_status(),
                direction: to_direction_type(self.Direction),
                price: self.LimitPrice,
                trades: vec![],
            }
        }
        fn volume_traded(&self) -> i32 {
            self.VolumeTraded
        }
        fn volume_canceled(&self) -> i32 {
            match self.pending_status() {
                PendingOrderStatus::Canceled => self.VolumeTotalOriginal - self.VolumeTraded,
                _ => 0,
            }
        }
        fn exchange(&self) -> &str {
            get_ascii_str(&self.ExchangeID)
                .expect("CThostFtdcInvestorPositionDetailField.ExchangeID error")
        }
        fn symbol(&self) -> &str {
            get_ascii_str(&self.InstrumentID)
                .expect("CThostFtdcInvestorPositionDetailField.InstrumentID error")
        }
        fn order_sys_id(&self) -> &[i8; 21] {
            &self.OrderSysID
        }
    }

    impl std::cmp::PartialEq<PendingOrder> for CThostFtdcOrderField {
        fn eq(&self, other: &PendingOrder) -> bool {
            self.FrontID == other.front_id
                && self.SessionID == other.session_id
                && cstr_i8_eq(&self.OrderRef, &other.order_ref)
        }
    }

    fn to_direction_type(d: TThostFtdcDirectionType) -> DirectionType {
        if d as u8 == THOST_FTDC_D_Buy {
            DirectionType::Long
        } else if d as u8 == THOST_FTDC_D_Sell {
            DirectionType::Short
        } else {
            panic!("unkown ctp direction={}", d);
        }
    }

    impl From<&CThostFtdcDepthMarketDataField> for MarketDataSnapshot {
        fn from(src: &CThostFtdcDepthMarketDataField) -> Self {
            Self {
                ask1: src.AskPrice1,
                ask1_volume1: src.AskVolume1 as i64,
                bid1: src.BidPrice1,
                bid1_volume: src.BidVolume1 as i64,
                timestamp: 0,
            }
        }
    }

    // impl TraderApiType for CThostFtdcTraderApi {
    impl TraderApiType for trader_api::TraderApi {
        fn as_any(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn req_order_insert(
            &mut self,
            broker_id: &str,
            account: &str,
            exchange: &str,
            symbol: &str,
            i: &InputOrderField,
            order_ref: i32,
            n_request_id: i32,
            _shareholder_accounts: &Vec<ShareholderAccount>,
        ) -> Result<(), base::error::Error> {
            let mut input = CThostFtdcInputOrderField::default();
            set_cstr_from_str_truncate_i8(&mut input.BrokerID, broker_id);
            set_cstr_from_str_truncate_i8(&mut input.UserID, account);
            set_cstr_from_str_truncate_i8(&mut input.InvestorID, account);
            set_cstr_from_str_truncate_i8(&mut input.ExchangeID, exchange);
            set_cstr_from_str_truncate_i8(&mut input.InstrumentID, symbol);
            input.Direction = match i.direction {
                DirectionType::Long => THOST_FTDC_D_Buy,
                DirectionType::Short => THOST_FTDC_D_Sell,
            } as i8;
            input.CombOffsetFlag[0] = match i.offset {
                OffsetFlag::Open => THOST_FTDC_OF_Open,
                OffsetFlag::Close => THOST_FTDC_OF_Close,
                OffsetFlag::CloseToday => THOST_FTDC_OF_CloseToday,
                OffsetFlag::CloseYesterday => THOST_FTDC_OF_CloseYesterday,
                OffsetFlag::ReverseRepur => panic!("CTP接口不支持回购"),
                OffsetFlag::OfOther => panic!("Invalid offset"),
            } as i8;
            input.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation as i8;
            input.LimitPrice = i.price;
            input.VolumeTotalOriginal = i.volume;
            input.OrderPriceType = THOST_FTDC_OPT_LimitPrice as i8;
            if input.LimitPrice == 0.0 {
                input.OrderPriceType = THOST_FTDC_OPT_AnyPrice as i8;
            }
            input.TimeCondition = THOST_FTDC_TC_GFD as i8;
            input.VolumeCondition = THOST_FTDC_VC_AV as i8;
            input.MinVolume = 1;
            input.ContingentCondition = THOST_FTDC_CC_Immediately as i8;
            input.ForceCloseReason = THOST_FTDC_FCC_NotForceClose as i8;
            input.IsAutoSuspend = 0;
            input.UserForceClose = 0;
            set_cstr_from_str_truncate_i8(&mut input.OrderRef, &format!("{order_ref}"));
            let ret = unsafe { (*self.api).req_order_insert(&mut input, n_request_id) };
            if ret != 0 {
                return Err(base::error::Error::TraderApiErr(ret));
            }
            Ok(())
        }

        fn req_order_action(
            &mut self,
            broker_id: &str,
            account: &str,
            exchange: &str,
            symbol: &str,
            i: &InputOrderActionField,
            _order_action_ref: i32,
            n_request_id: i32,
        ) -> Result<(), base::error::Error> {
            let mut r = CThostFtdcInputOrderActionField::default();
            set_cstr_from_str_truncate_i8(&mut r.ExchangeID, exchange);
            set_cstr_from_str_truncate_i8(&mut r.InstrumentID, symbol);
            set_cstr_from_str_truncate_i8(&mut r.BrokerID, broker_id);
            set_cstr_from_str_truncate_i8(&mut r.UserID, account);
            set_cstr_from_str_truncate_i8(&mut r.InvestorID, account);
            r.FrontID = i.front_id;
            r.SessionID = i.session_id;
            r.OrderRef = i.order_ref.clone();
            r.ActionFlag = THOST_FTDC_AF_Delete as i8;
            let ret = unsafe { (*self.api).req_order_action(&mut r, n_request_id) };
            if ret != 0 {
                return Err(base::error::Error::TraderApiErr(ret));
            }
            Ok(())
        }
    }

    pub async fn handle_spi_msg(
        spi_msg: &trader_api::CThostFtdcTraderSpiOutput,
        state: &mut AccountStateType,
        cmc: &Arc<Mutex<MdCache>>,
        api: &mut Box<dyn TraderApiType + Send>,
    ) -> Result<(), Error> {
        use trader_api::CThostFtdcTraderSpiOutput::*;
        match spi_msg {
            OnFrontDisconnected(p) => {
                info!("{}:{} {:?}", state.broker_id, state.account, p);
                return Err(Error::FrontDisconnected);
            }
            OnRtnTradingNotice(ref p) => {
                info!(
                    "{}:{} RtnTradingNotice = {:?}",
                    state.broker_id, state.account, p
                );
            }
            OnRspError(ref p) => {
                info!(
                    "{}:{} RspError={:?}",
                    state.broker_id,
                    state.account,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnRspOrderInsert(ref p) => {
                // 需要构建order通知撤单
                info!(
                    "{}:{} RspOrderInsert={:?}",
                    state.broker_id,
                    state.account,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnRspOrderAction(ref p) => {
                info!(
                    "{}:{} OnRspOrderAction={:?}",
                    state.broker_id,
                    state.account,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnErrRtnOrderInsert(ref p) => {
                // 需要构建order通知撤单
                if let Some(input) = p.p_input_order.as_ref() {
                    let us = UniqueSymbol::new(
                        get_ascii_str(&input.ExchangeID).unwrap(),
                        get_ascii_str(&input.InstrumentID).unwrap(),
                    );
                    state.remove_po(&us, state.front_id, state.session_id, &input.OrderRef);
                    info!(
                        "{}:{} 删除发送失败的委托 OrderRef={} result={:?}",
                        state.broker_id,
                        state.account,
                        get_ascii_str(&input.OrderRef).unwrap(),
                        print_rsp_info!(&p.p_rsp_info)
                    );
                }
            }
            OnErrRtnOrderAction(ref p) => {
                // info!("ErrRtnOrderAction={:?}", print_rsp_info!(&p.p_rsp_info));
                info!(
                    "{}:{} ErrRtnOrderAction={:?} {:?}",
                    state.broker_id,
                    state.account,
                    p,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnRtnOrder(ref rtn) => {
                match rtn.p_order {
                    Some(o) => {
                        let submit_status = o.OrderSubmitStatus;
                        state.update_by_order(o).unwrap();
                        if o.pending_status() == PendingOrderStatus::Canceled {
                            let submit_status_msg = match submit_status as u8 as char {
                                '0' => "THOST_FTDC_OSS_InsertSubmitted 已经提交",
                                '1' => "THOST_FTDC_OSS_CancelSubmitted 撤单已提交",
                                '2' => "THOST_FTDC_OSS_ModifySubmitted 修改提交",
                                '3' => "THOST_FTDC_OSS_Accepted 已经接受",
                                '4' => "THOST_FTDC_OSS_InsertRejected 报单已被拒绝",
                                '5' => "THOST_FTDC_OSS_CancelRejected 撤单已经被拒绝",
                                '6' => "THOST_FTDC_OSS_ModifyRejected 改单已经被拒绝",
                                _ => panic!(
                                    "OrderSubmitStatus={} submit_status={}",
                                    submit_status, submit_status
                                ),
                            };
                            info!("已撤单 OrderSubmitStatus={}", submit_status_msg);
                            let status_msg = gb18030_cstr_to_str_i8(&o.StatusMsg).to_string();
                            if submit_status == THOST_FTDC_OSS_InsertRejected as i8
                                || submit_status == THOST_FTDC_OSS_CancelRejected as i8
                                || status_msg.contains("当前状态禁止")
                                || status_msg.contains("废单")
                            {
                                info!("非交易时间不发单, OrderSubmitStatus={}", submit_status);
                            } else {
                                // 价格变化导致的撤单，要及时重新发单
                                let us = UniqueSymbol::new(
                                    get_ascii_str(&o.ExchangeID).expect("OnRtnOrder get_ascii_str"),
                                    get_ascii_str(&o.InstrumentID)
                                        .expect("OnRtnOrder get_ascii_str"),
                                );
                                if let Err(e) = state.set_check_target(us, None, &cmc, api).await {
                                    error!("OnRtnOrder set_check_target {e}");
                                }
                            }
                        }
                    }
                    None => error!("RtnOrder rtn=nil"),
                }
            }
            OnRtnTrade(rtn) => {
                match rtn.p_trade {
                    Some(mut trade) => {
                        // rtn.TradingDay = state.trading_day_ctp.clone(); // 上海夜盘成交的交易日没有更新到第二天
                        trade.TradeDate = trade.TradingDay.clone();
                        let changed = state.update_by_trade(trade).unwrap();
                        if changed {
                            let us = UniqueSymbol::new(
                                get_ascii_str(&trade.ExchangeID).expect("OnRtnTrade get_ascii_str"),
                                get_ascii_str(&trade.InstrumentID)
                                    .expect("OnRtnTrade get_ascii_str"),
                            );
                            if let Err(e) = state.set_check_target(us, None, &cmc, api).await {
                                error!("OnRtnTrade set_check_target {e}");
                            }
                        }
                    }
                    None => error!("RtnTrade rtn=nil"),
                }
            }
            OnRspQryTradingAccount(ref _p) => {}
            OnRspQryInvestorPosition(ref p) => {
                let _pos = match p.p_investor_position {
                    Some(ref _ip) => (),
                    None => (),
                };
            }
            OnRspQryInvestorPositionDetail(ref p) => {
                let _detail = match p.p_investor_position_detail {
                    Some(ref _pd) => (),
                    None => (),
                };
            }
            OnRtnInstrumentStatus(_p) => {}
            OnHeartBeatWarning(_p) => {}
            other => {
                info!("Un handled spi msg = {:?}", other);
            }
        };
        Ok(())
    }

    pub async fn handle_request_msg(
        req_msg: &ReqMessage,
        state: &mut AccountStateType,
        api: &mut TraderApi,
    ) -> Result<(), Error> {
        use ReqMessage::*;
        match req_msg {
            SetContractTarget(_target) => {}
            QueryPositionDetail => {
                info!("req_msg={:?}", req_msg);
                let mut req = CThostFtdcQryInvestorPositionDetailField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, &state.broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &state.account);
                let result = api.req_qry_investor_position_detail(&mut req, state.get_request_id());
                if result != 0 {
                    info!("ReqQueryInvestorPositionDetail={}", result);
                }
            }
            QueryTradingAccount => {
                let mut req = CThostFtdcQryTradingAccountField::default();
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &state.account);
                let result = api.req_qry_trading_account(&mut req, state.get_request_id());
                if result != 0 {
                    info!("ReqQueryTradingAccount={}", result);
                }
            }
        }
        info!("handle_request_msg = {:?}", req_msg);
        Ok(())
    }

    pub async fn run_trader(
        ca: TradingAccountConfig,
        cmc: Arc<Mutex<MdCache>>,
        mut rx: mpsc::Receiver<(ReqMessage, oneshot::Sender<RspMessage>)>,
        logger: Box<dyn TradingLoger + Send>,
    ) -> Result<(), Error> {
        info!("Run ctp trader [{}]", ca.account);
        logger.info(
            "Ctp futures trader startup",
            &format!("{}:{} run ctp_futures trader", ca.broker_id, ca.account),
        );
        let mut state = AccountStateType::new(&ca.broker_id, &ca.account);
        state.trading_logger = Some(logger);
        let flow_path = format!(
            ".cache/ctp_trade_daemon_flow_{}_{}//",
            ca.broker_id, ca.account
        );
        check_make_dir(&flow_path);
        let mut api = Box::new(trader_api::create_api(&flow_path, false));
        let mut stream = {
            let (stream, pp) = trader_api::create_spi();
            api.register_spi(pp);
            stream
        };
        if ca.name_servers.len() > 0 {
            api.register_name_server(CString::new(ca.name_servers[0].as_str()).unwrap());
        } else if ca.trade_fronts.len() > 0 {
            for front in ca.trade_fronts.iter().filter(|f| !f.starts_with("#")) {
                api.register_front(CString::new(front.as_str()).unwrap());
                info!("Ctp reigster trade front {front}");
            }
        }
        api.subscribe_public_topic(THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.subscribe_private_topic(THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.init();
        // 处理登陆初始化查询
        let mut cached_pdl: Vec<PositionDetail> = vec![];
        let mut cached_orders: Vec<CThostFtdcOrderField> = vec![];
        let mut cached_trades: Vec<CThostFtdcTradeField> = vec![];

        loop {
            match tokio::time::timeout(tokio::time::Duration::from_secs(10), stream.next()).await {
                Ok(Some(spi_msg)) => {
                    use trader_api::CThostFtdcTraderSpiOutput::*;
                    match spi_msg {
                        OnFrontConnected(_p) => {
                            let mut req = CThostFtdcReqAuthenticateField::default();
                            set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                            set_cstr_from_str_truncate_i8(&mut req.UserID, &ca.account);
                            set_cstr_from_str_truncate_i8(&mut req.AuthCode, &ca.auth_code);
                            set_cstr_from_str_truncate_i8(
                                &mut req.UserProductInfo,
                                &ca.user_product_info,
                            );
                            set_cstr_from_str_truncate_i8(&mut req.AppID, &ca.app_id);
                            api.req_authenticate(&mut req, state.get_request_id());
                        }
                        OnFrontDisconnected(p) => {
                            info!("on front disconnected {:?} 直接Exit ", p);
                            return Err(Error::FrontDisconnected);
                        }
                        OnRspAuthenticate(ref p) => {
                            if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                                let mut req = CThostFtdcReqUserLoginField::default();
                                set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                                set_cstr_from_str_truncate_i8(&mut req.UserID, &ca.account);
                                set_cstr_from_str_truncate_i8(&mut req.Password, &ca.password);
                                api.req_user_login(&mut req, state.get_request_id());
                            } else {
                                error!(
                                    "{}:{} ctp trade RspAuthenticate={:?}",
                                    ca.broker_id,
                                    ca.account,
                                    print_rsp_info!(&p.p_rsp_info)
                                );
                                return Err(Error::CtpAuthFailed);
                            }
                        }
                        OnRspUserLogin(ref p) => {
                            if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                                let u = p.p_rsp_user_login.as_ref().unwrap();
                                state.on_login(
                                    u.FrontID,
                                    u.SessionID,
                                    &u.MaxOrderRef,
                                    &u.TradingDay,
                                );
                                info!(
                            "{}:{} Ctp trade 登陆成功 trading_day={} front_id={} session_id={}",
                            state.broker_id,
                            state.account,
                            state.trading_day_i32,
                            state.front_id,
                            state.session_id
                        );
                            } else {
                                error!(
                                    "ctp trade RspUserLogin = {:?}",
                                    print_rsp_info!(&p.p_rsp_info)
                                );
                            }
                            let mut req = CThostFtdcSettlementInfoConfirmField::default();
                            set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                            set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                            let result =
                                api.req_settlement_info_confirm(&mut req, state.get_request_id());
                            if result != 0 {
                                error!("ReqSettlementInfoConfirm={}", result);
                            }
                        }
                        OnRspSettlementInfoConfirm(ref _p) => {
                            let mut req = CThostFtdcQryInstrumentField::default();
                            let result = api.req_qry_instrument(&mut req, state.get_request_id());
                            if result != 0 {
                                error!("ReqQryInstrument = {:?}", result);
                            }
                        }
                        OnRspQryInvestorPositionDetail(detail) => {
                            if let Some(pd) = detail.p_investor_position_detail {
                                if pd.Volume > 0 {
                                    // info!(
                                    //     "pd {} Volume={} CloseVolume={} OpenDate={} TradingDay={}",
                                    //     get_ascii_str(&pd.InstrumentID).unwrap(),
                                    //     pd.Volume,
                                    //     pd.CloseVolume,
                                    //     get_ascii_str(&pd.OpenDate).unwrap(),
                                    //     get_ascii_str(&pd.TradingDay).unwrap(),
                                    // );
                                    cached_pdl.push(PositionDetail::from(&pd));
                                }
                            }
                            if detail.b_is_last {
                                let mut req = CThostFtdcQryOrderField::default();
                                set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                let result = api.req_qry_order(&mut req, state.get_request_id());
                                if result != 0 {
                                    error!("ReqQryOrder = {:?}", result);
                                }
                            }
                        }
                        OnRspQryOrder(p) => {
                            if let Some(o) = p.p_order {
                                cached_orders.push(o);
                            }
                            if p.b_is_last {
                                let mut req = CThostFtdcQryTradeField::default();
                                set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                let ret = api.req_qry_trade(&mut req, state.get_request_id());
                                if ret != 0 {
                                    error!("ReqQryTrade = {}", ret);
                                }
                            }
                        }
                        OnRtnOrder(p) => {
                            if let Some(o) = p.p_order {
                                cached_orders.push(o);
                            }
                        }
                        OnRspQryTrade(p) => {
                            if let Some(p) = p.p_trade {
                                cached_trades.push(p);
                            }
                            if p.b_is_last {
                                break;
                            }
                        }
                        OnRtnTrade(p) => {
                            if let Some(mut trade) = p.p_trade {
                                trade.TradeDate = trade.TradingDay.clone();
                                cached_trades.push(trade);
                            }
                        }
                        OnRspQryInstrument(ref p) => {
                            if let Some(i) = &p.p_instrument {
                                let xif = InstrumentField {
                                    price_tick: i.PriceTick,
                                    is_close_today_allowed: true,
                                    ctp_product_class: i.ProductClass,
                                    tora_instrument_type: 0,
                                };
                                let us = UniqueSymbol::new(
                                    get_ascii_str(&i.ExchangeID).unwrap(),
                                    get_ascii_str(&i.InstrumentID).unwrap(),
                                );
                                state.hm_inst.insert(us, xif);
                            }
                            if p.b_is_last {
                                let mut req = CThostFtdcQryInvestorPositionDetailField::default();
                                set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                                let result = api.req_qry_investor_position_detail(
                                    &mut req,
                                    state.get_request_id(),
                                );
                                if result != 0 {
                                    error!("ctp ReqQryInvestorPositionDetail = {:?}", result);
                                }
                            }
                        }
                        OnRtnInstrumentStatus(ref p) => match p.p_instrument_status {
                            Some(status) => {
                                let s = match status.InstrumentStatus as u8 as char {
                                    '0' => "THOST_FTDC_IS_BeforeTrading",
                                    '1' => "THOST_FTDC_IS_NoTrading",
                                    '2' => "THOST_FTDC_IS_Continous",
                                    '3' => "THOST_FTDC_IS_AuctionOrdering",
                                    '4' => "THOST_FTDC_IS_AuctionBalance",
                                    '5' => "THOST_FTDC_IS_AuctionMatch",
                                    '6' => "THOST_FTDC_IS_Closed",
                                    _ => "UnkownInstrumentStatus",
                                };
                                info!(
                                    "RtnInstrumentStatus = {}:{} {}",
                                    get_ascii_str(&status.ExchangeID).unwrap().to_string(),
                                    get_ascii_str(&status.InstrumentID).unwrap().to_string(),
                                    s
                                );
                            }
                            None => (),
                        },
                        OnRtnTradingNotice(ref p) => {
                            info!("RtnTradingNotice = {:?}", p);
                        }
                        _ => {}
                    }
                }
                Ok(None) => {}
                Err(_e) => {
                    return Err(Error::InitTraderFailed);
                }
            }
        }

        // 初始化查询过程中推送的成交
        if let Err(e) = state.update_on_initialized(cached_pdl, cached_orders, cached_trades) {
            error!("update_on_initialized {e}");
        }
        info!("{} 初始化查询完成.", ca.account);
        let mut api = api as Box<dyn TraderApiType + Send>;

        info!("{}:{} Trader initialized.", ca.broker_id, ca.account);
        {
            let mut cmc = cmc.lock().await;
            for cd in state.sorted_cds.iter() {
                cmc.subscribe(&cd.us.exchange, &cd.us.symbol).await;
            }
        }

        let mut query_req: Option<(
            ReqMessage,
            oneshot::Sender<RspMessage>,
            Vec<trader_api::CThostFtdcTraderSpiOutput>,
        )> = None;
        loop {
            tokio::select! {
                Some(spi_msg) = stream.next() => {
                    let _ = handle_spi_msg(&spi_msg, &mut state, &cmc, &mut api).await?;
                    use trader_api::CThostFtdcTraderSpiOutput::*;
                    use ReqMessage::*;
                    let is_last = if let Some((req_msg, rsp_tx, ref mut response_packets)) = &mut query_req {
                        let (is_result, is_last) = match (req_msg, &spi_msg) {
                            (SetContractTarget(_), _) => panic!("SetContractTarget do not have response"),
                            (QueryPositionDetail, OnRspQryInvestorPositionDetail(p)) => (true, p.b_is_last),
                            (QueryTradingAccount, OnRspQryTradingAccount(p)) => (true, p.b_is_last),
                            (_, _) => (false, false),
                        };
                        if is_result {
                            response_packets.push(spi_msg);
                        }
                        is_last
                    } else {
                        false
                    };
                    if is_last {
                        if let Some((req_msg, rsp_tx, response_packets)) = query_req.take() {
                            let config = bincode::config::standard();
                            let encoded: Vec<u8> = bincode::encode_to_vec(&response_packets, config).unwrap();
                            if let Err(_) = rsp_tx.send(Ok(encoded)) {
                                error!("the receiver droped");
                            }
                        }
                    }
                },
                Some((req_msg, rsp_tx)) = rx.recv() => {
                    if let ReqMessage::SetContractTarget(target) = req_msg {
                        let us = UniqueSymbol::new(&target.exchange, &target.symbol);
                        let res = state.set_check_target(us, Some(target), &cmc, &mut api).await.map(|_|vec![]);
                        let _ = rsp_tx.send(res);
                    } else {
                        if query_req.is_some() {
                            let _ = rsp_tx.send(Err(Error::CtpLastQueryIsProceeding));
                        } else {
                            query_req = Some((req_msg, rsp_tx, vec![]));
                            let api = api.as_any().downcast_mut::<TraderApi>().unwrap();
                            handle_request_msg(&query_req.as_ref().unwrap().0, &mut state, api).await?;
                        }
                    }
                },
                else => break,
            }
        }
        info!("{}:{} trader_deamon退出", ca.broker_id, ca.account);
        Ok(())
    }

    pub async fn run_md_cache(
        ca: TradingAccountConfig,
        mut rx: mpsc::Receiver<UniqueSymbol>,
        cmc: Arc<Mutex<MdCache>>,
    ) {
        info!("[{}] market data receiver start", ca.account);
        if ca.md_fronts.len() == 0 {
            panic!("md front is not valid");
        }
        let flow_path = format!(".cache/ctp_md_flow_{}_{}//", ca.broker_id, ca.account);
        check_make_dir(&flow_path);
        let mut mdapi = md_api::create_api(&flow_path, false, false);
        for front in ca.md_fronts.iter().filter(|f| !f.starts_with("#")) {
            info!("Md RegisterFront {}", front);
            mdapi.register_front(CString::new(front.as_str()).unwrap());
        }

        let mut stream = {
            let (stream, pp) = md_api::create_spi();
            mdapi.register_spi(pp);
            stream
        };
        mdapi.init();
        let mut trading_day: [i8; 9] = [0; 9];
        let mut initialized = false;
        while let Some(msg) = stream.next().await {
            use md_api::CThostFtdcMdSpiOutput::*;
            match msg {
                OnFrontConnected(ref _p) => {
                    info!("[{}] 行情服务器FrontConnected", ca.account);
                    let mut req_login: CThostFtdcReqUserLoginField = Default::default();
                    let ret = mdapi.req_user_login(&mut req_login, 3);
                    if ret != 0 {
                        info!("Ctp md ReqUserLogin={}", ret);
                    }
                }
                OnFrontDisconnected(ref p) => {
                    info!(
                        "Front disconnected {:?} 直接Exit trading_day={}",
                        p,
                        get_ascii_str(&trading_day).unwrap()
                    );
                    break;
                }
                OnRspUserLogin(ref p) => {
                    if p.p_rsp_info.is_some() {
                        let u = p.p_rsp_user_login.unwrap();
                        trading_day = u.TradingDay.clone();
                        info!(
                            "行情服务器登陆成功, trading_day=[{}]",
                            get_ascii_str(&trading_day).unwrap()
                        );
                        initialized = true;
                    } else {
                        info!(
                            "Quote Md RspUserLogin = {:?}",
                            print_rsp_info!(&p.p_rsp_info)
                        );
                    }
                    break;
                }
                OnRspError(ref p) => {
                    info!("Quote daemon OnRspError!");
                    if let Some(info) = p.p_rsp_info {
                        info!(
                            "ErrorID={}, ErrorMsg={}",
                            info.ErrorID,
                            get_ascii_str(&info.ErrorMsg).unwrap().to_string()
                        );
                    }
                }
                _ => {
                    info!("msg={:?}", msg)
                }
            }
        }
        if !initialized {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            error!("[{}] ctp md failed to initialize", ca.account);
            info!("Restart md cache in 10 seconds");
            test_spawn(ca, rx, cmc);
            return;
        }
        // 重新连接后订阅
        let mut subscribed = { cmc.lock().await.subscribed.clone() };
        let result = mdapi.subscribe_market_data(
            subscribed
                .iter()
                .map(|e| CString::new(e.symbol.clone()).unwrap())
                .collect_vec(),
            subscribed.len() as i32,
        );
        if result != 0 {
            error!("Subscribe result = {}", result);
        }
        subscribed.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        loop {
            tokio::select! {
                Some(msg) = stream.next() => {
                    use md_api::CThostFtdcMdSpiOutput::*;
                    match msg {
                        OnFrontDisconnected(ref p) => {
                            info!(
                                "on front disconnected {:?} 直接Exit trading_day={}",
                                p,
                                get_ascii_str(&trading_day).unwrap()
                            );
                            break;
                        }
                        OnRtnDepthMarketData(ref ctp_md) => {
                            if let Some(ctp_md) = ctp_md.p_depth_market_data.as_ref() {
                                let us = if ctp_md.ExchangeID[0] == 0 {
                                    let symbol = get_ascii_str(&ctp_md.InstrumentID).unwrap();
                                    let i = subscribed.binary_search_by(|probe| probe.symbol.as_str().cmp(symbol)).unwrap();
                                    UniqueSymbol::new(&subscribed[i].exchange, symbol)
                                } else {
                                    UniqueSymbol::new(get_ascii_str(&ctp_md.ExchangeID).unwrap(),get_ascii_str(&ctp_md.InstrumentID).unwrap())
                                };
                                let us1 = us.clone();
                                let mut cmc = cmc.lock().await;
                                    cmc.hm_md
                                    .entry(us)
                                    .and_modify(|e| {
                                        e.ask1 = ctp_md.AskPrice1;
                                        e.bid1 = ctp_md.BidPrice1;
                                        e.ask1_volume1 = ctp_md.AskVolume1 as i64;
                                        e.bid1_volume = ctp_md.BidVolume1 as i64;
                                        e.timestamp = Local::now().timestamp();
                                    })
                                    .or_insert_with(|| {
                                        let mut md = MarketDataSnapshot::from(ctp_md);
                                        md.timestamp = Local::now().timestamp();
                                        info!("[{: >5}:{: <6}] insert md = {:?}", us1.exchange, us1.symbol ,md);
                                        md
                                    });
                            }
                        }
                        _ => (),
                    }
                }
                Some(us) = rx.recv() => {
                    let result = mdapi.subscribe_market_data(vec![CString::new(us.symbol.clone()).unwrap()], 1);
                    if result != 0 {
                        error!("Subscribe result = {}", result);
                    }
                    subscribed.push(us);
                    subscribed.sort_by(|a, b| a.symbol.cmp(&b.symbol));
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // tokio::spawn(Box::pin(run_md_cache(ca, rx, cmc)));
        test_spawn(ca, rx, cmc);
    }

    /// 不加这个编译出错
    fn test_spawn(
        ca: TradingAccountConfig,
        rx: mpsc::Receiver<UniqueSymbol>,
        cmc: Arc<Mutex<MdCache>>,
    ) {
        tokio::spawn(async move { Box::pin(run_md_cache(ca, rx, cmc)).await });
    }

    pub fn new_md_cache(ca: &TradingAccountConfig) -> Arc<Mutex<MdCache>> {
        let (tx, rx) = mpsc::channel::<UniqueSymbol>(1000);
        let cmc = Arc::new(Mutex::new(MdCache {
            tx,
            hm_md: HashMap::new(),
            subscribed: vec![],
        }));
        tokio::spawn(run_md_cache(ca.clone(), rx, Arc::clone(&cmc)));
        cmc
    }
}

pub mod query {
    use super::*;
    use base::util::*;
    use base::*;
    use bincode::{Decode, Encode};
    use futures::StreamExt;
    use log::info;
    use std::ffi::CString;

    /// 查询结果
    #[derive(Clone, Debug, Default, Encode, Decode)]
    pub struct CtpQueryResult {
        pub broker_id: String,
        pub account: String,
        pub trading_day: i32,
        pub timestamp: i64,
        pub dmd_list: Vec<CThostFtdcDepthMarketDataField>,
        pub icr_list: Vec<CThostFtdcInstrumentCommissionRateField>,
        pub instruments: Vec<CThostFtdcInstrumentField>,
        pub positions: Vec<CThostFtdcInvestorPositionField>,
        pub position_detail_list: Vec<CThostFtdcInvestorPositionDetailField>,
        pub trading_account: CThostFtdcTradingAccountField,
        pub products: Vec<CThostFtdcProductField>,
        pub orders: Vec<CThostFtdcOrderField>,
        pub trades: Vec<CThostFtdcTradeField>,
    }

    pub async fn query(ca: &TradingAccountConfig) -> Result<CtpQueryResult, base::error::Error> {
        info!("start query");
        use trader_api::*;
        let broker_id = ca.broker_id.as_str();
        let account = ca.account.as_str();
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
            ".cache/ctp_futures_query_trade_flow_{}_{}//",
            broker_id, account
        );
        check_make_dir(&flow_path);
        let mut api = create_api(&flow_path, false);
        let mut stream = {
            let (stream, pp) = trader_api::create_spi();
            // api::<*const CThostFtdcTraderApi>::as_ref()
            api.register_spi(pp);
            stream
        };
        if ca.name_servers.len() > 0 {
            api.register_name_server(CString::new(ca.name_servers[0].as_str()).unwrap());
        } else {
            api.register_front(CString::new(ca.trade_fronts[0].as_str()).unwrap());
            info!("register front {}", ca.trade_fronts[0]);
        }
        api.subscribe_public_topic(THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.subscribe_private_topic(THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.init();
        let mut result = CtpQueryResult::default();
        result.broker_id = broker_id.to_string();
        result.account = account.to_string();
        // 处理登陆初始化查询
        while let Some(spi_msg) = stream.next().await {
            use trader_api::CThostFtdcTraderSpiOutput::*;
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
        // info!("release on end");
        // api.release();
        // info!("leak on end");
        // Box::leak(api);
        // info!("after leak on end");
        Ok(result)
    }
}

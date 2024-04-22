#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use bincode::{Decode, Encode};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use base::state::*;
use base::util::*;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct StringArray {
    inner: Vec<CString>,
    pl: Vec<*const c_char>,
}

impl StringArray {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            pl: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, s: &str) {
        self.inner.push(CString::new(s).unwrap());
    }

    pub fn to_char_pp(&mut self) -> *mut *mut std::os::raw::c_char {
        self.pl.clear();
        let v = self.inner.iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>();
        self.pl = v;
        self.pl.as_mut_ptr() as *mut *mut i8
    }
}

impl From<Vec<String>> for StringArray {
    fn from(src: Vec<String>) -> StringArray {
        let mut i = vec![];
        for s in &src {
            i.push(CString::new(s.as_str()).unwrap());
        }
        StringArray {
            inner: i,
            pl: vec![],
        }
    }
}

pub mod md_api {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused_variables, unused_mut)]
    #![allow(dead_code)]
    use crate::*;
    // use share::trader::StringArray;
    include!("md_impl.rs");
}

pub mod trader_api {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused_variables, unused_mut)]
    #![allow(dead_code)]

    use crate::*;
    // use share::trader::StringArray;
    include!("trade_impl.rs");
    pub fn create_api(flow_path: &str, b_encrypt: bool) -> Box<TORASTOCKAPI_CTORATstpTraderApi> {
        let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
        unsafe {
            Box::from_raw(TORASTOCKAPI_CTORATstpTraderApi_CreateTstpTraderApi(
                trade_flow_path.as_ptr(),
                b_encrypt,
            ))
        }
    }
    pub fn unsafe_clone_api(
        source: Box<TORASTOCKAPI_CTORATstpTraderApi>,
    ) -> (
        Box<TORASTOCKAPI_CTORATstpTraderApi>,
        Box<TORASTOCKAPI_CTORATstpTraderApi>,
    ) {
        let p = Box::into_raw(source);
        unsafe {
            let p2 = p.clone();
            (Box::from_raw(p), Box::from_raw(p2))
        }
    }
}

pub mod route {
    use crate::*;
    use base::state::md_cache::MdCache;
    use base::state::MarketDataSnapshot;
    use base::*;
    use chrono::Local;
    use futures::StreamExt;
    use log::{error, info};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::{mpsc, oneshot, Mutex};

    type AccountStateType = ToraAccountState;

    impl std::cmp::PartialEq for TORASTOCKAPI_CTORATstpOrderField {
        fn eq(&self, other: &Self) -> bool {
            self.FrontID == other.FrontID
                && self.SessionID == other.SessionID
                && self.OrderRef == other.OrderRef
        }
    }

    impl std::cmp::PartialEq<PendingOrder> for TORASTOCKAPI_CTORATstpOrderField {
        fn eq(&self, other: &PendingOrder) -> bool {
            self.FrontID == other.front_id
                && self.SessionID == other.session_id
                && self.OrderRef == other.order_ref_i32
        }
    }

    pub type ToraAccountState =
        AccountState<TORASTOCKAPI_CTORATstpOrderField, TORASTOCKAPI_CTORATstpTradeField>;

    /// 查询返回的持仓结构中分解出今仓和昨仓
    pub fn make_position_detail(
        src: &TORASTOCKAPI_CTORATstpPositionField,
    ) -> (PositionDetail, PositionDetail) {
        let mut today = PositionDetail::default();
        today.exchange = exchange_from_tora_stock_exchange_id(src.ExchangeID).to_string();
        today.symbol = ascii_cstr_to_str_i8(&src.SecurityID).unwrap().to_string();
        // today.price = src.OpenPosCost / src.CurrentPosition as f64;
        today.volume = src.TodayBSPos;
        today.direction = DirectionType::Long;
        today.open_date = src.TradingDay.clone();

        let mut yesterday = today.clone();
        yesterday.volume = src.HistoryPos;
        set_cstr_from_str_truncate_i8(&mut yesterday.open_date, "20010101");
        if today.volume + yesterday.volume != src.CurrentPosition {
            panic!("srcPosition={:?}", src);
        }
        (today, yesterday)
    }

    impl TradeType for TORASTOCKAPI_CTORATstpTradeField {
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
            exchange_from_tora_stock_exchange_id(self.ExchangeID)
        }
        fn symbol(&self) -> &str {
            get_ascii_str(&self.SecurityID)
                .expect("TORASTOCKAPI_CTORATstpInvestorPositionDetailField.InstrumentID error")
        }
        fn order_sys_id(&self) -> &[i8; 21] {
            &self.OrderSysID
        }
        fn trade_id(&self) -> &[i8; 21] {
            &self.TradeID
        }
        fn offset_flag(&self) -> OffsetFlag {
            match self.Direction {
                TORASTOCKAPI_TORA_TSTP_D_Buy => OffsetFlag::Open,
                TORASTOCKAPI_TORA_TSTP_D_Sell => OffsetFlag::Close,
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

    impl OrderType for TORASTOCKAPI_CTORATstpOrderField {
        // fn front_id(&self) -> i32 {
        //     self.FrontID
        // }
        // fn session_id(&self) -> i32 {
        //     self.SessionID
        // }
        // fn order_ref(&self) -> i32 {
        //     self.OrderRef
        // }
        fn volume_traded(&self) -> i32 {
            self.VolumeTraded
        }
        fn volume_canceled(&self) -> i32 {
            match self.OrderStatus {
                TORASTOCKAPI_TORA_TSTP_OST_Rejected => self.VolumeTotalOriginal,
                _ => self.VolumeCanceled,
            }
        }
        fn exchange(&self) -> &str {
            exchange_from_tora_stock_exchange_id(self.ExchangeID)
        }
        fn symbol(&self) -> &str {
            get_ascii_str(&self.SecurityID)
                .expect("TORASTOCKAPI_CTORATstpInvestorPositionDetailField.InstrumentID error")
        }
        fn order_sys_id(&self) -> &[i8; 21] {
            &self.OrderSysID
        }
        fn to_pending_order(&self) -> PendingOrder {
            let mut order_ref = [0i8; 13];
            set_cstr_from_str_truncate_i8(&mut order_ref, &format!("{}", self.OrderRef));
            PendingOrder {
                front_id: self.FrontID,
                session_id: self.SessionID,
                order_ref,
                order_ref_i32: self.OrderRef,
                order_sys_id: self.OrderSysID.clone(),
                volume_traded: self.VolumeTraded,
                volume_canceled: self.volume_canceled(),
                volume_total_original: self.VolumeTotalOriginal,
                price: self.LimitPrice,
                direction: to_direction_type(self.Direction),
                status: self.pending_status(),
                trades: vec![],
            }
        }
        fn pending_status(&self) -> PendingOrderStatus {
            /*
            ///预埋
            const char TORA_TSTP_OST_Cached = '0';
            ///未知
            const char TORA_TSTP_OST_Unknown = '1';
            ///交易所已接收
            const char TORA_TSTP_OST_Accepted = '2';
            ///部分成交
            const char TORA_TSTP_OST_PartTraded = '3';
            ///全部成交
            const char TORA_TSTP_OST_AllTraded = '4';
            ///部成部撤
            const char TORA_TSTP_OST_PartTradeCanceled = '5';
            ///全部撤单
            const char TORA_TSTP_OST_AllCanceled = '6';
            ///交易所已拒绝
            const char TORA_TSTP_OST_Rejected = '7';
            ///发往交易核心
            const char TORA_TSTP_OST_SendTradeEngine = '#';
            */
            match self.OrderStatus {
                TORASTOCKAPI_TORA_TSTP_OST_PartTradeCanceled
                | TORASTOCKAPI_TORA_TSTP_OST_AllCanceled
                | TORASTOCKAPI_TORA_TSTP_OST_Rejected => PendingOrderStatus::Canceled,
                TORASTOCKAPI_TORA_TSTP_OST_AllTraded => PendingOrderStatus::Done,
                _ => PendingOrderStatus::Pending,
            }
        }
    }

    fn to_direction_type(d: TORASTOCKAPI_TTORATstpDirectionType) -> DirectionType {
        if d == TORASTOCKAPI_TORA_TSTP_D_Buy {
            DirectionType::Long
        } else if d == TORASTOCKAPI_TORA_TSTP_D_Sell {
            DirectionType::Short
        } else {
            panic!("unkown ctp direction={}", d);
        }
    }

    impl From<&TORALEV1API_CTORATstpMarketDataField> for MarketDataSnapshot {
        fn from(src: &TORALEV1API_CTORATstpMarketDataField) -> Self {
            Self {
                ask1: src.AskPrice1,
                ask1_volume1: src.AskVolume1,
                bid1: src.BidPrice1,
                bid1_volume: src.BidVolume1,
                timestamp: 0,
            }
        }
    }

    pub fn exchange_from_tora_stock_exchange_id(e: i8) -> &'static str {
        match e as u8 as char {
            '1' => "SSE",
            '2' => "SZE",
            '3' => "HK",
            '4' => "BSE",
            _ => panic!("e={}", e as u8 as char),
        }
    }

    pub fn to_tora_stock_exchange_id(exchange: &str) -> i8 {
        let r = match exchange {
            "SSE" => '1',
            "SZE" => '2',
            "HK" | "HKE" => '3',
            "BSE" => '4',
            _ => panic!(""),
        };
        r as i8
        // ///通用(内部使用)
        // #define TORA_TSTP_EXD_COMM '0'
        // ///上海交易所
        // #define TORA_TSTP_EXD_SSE '1'
        // ///深圳交易所
        // #define TORA_TSTP_EXD_SZSE '2'
        // ///香港交易所
        // #define TORA_TSTP_EXD_HK '3'
    }

    impl TraderApiType for TORASTOCKAPI_CTORATstpTraderApi {
        fn req_order_insert(
            &mut self,
            _broker_id: &str,
            _account: &str,
            exchange: &str,
            symbol: &str,
            i: &InputOrderField,
            order_ref: i32,
            n_request_id: i32,
            shareholder_accounts: &Vec<ShareholderAccount>,
        ) -> Result<(), base::error::Error> {
            let mut input = TORASTOCKAPI_CTORATstpInputOrderField::default();
            input.ExchangeID = to_tora_stock_exchange_id(exchange);
            input.OrderRef = order_ref;
            set_cstr_from_str_truncate_i8(&mut input.SecurityID, symbol);
            {
                // 关于方向
                //         //买入
                // #define TORA_TSTP_D_Buy '0'
                // ///卖出
                // #define TORA_TSTP_D_Sell '1'
                // ///ETF申购
                // #define TORA_TSTP_D_ETFPur '2'
                // ///ETF赎回
                // #define TORA_TSTP_D_ETFRed '3'
                // ///新股申购
                // #define TORA_TSTP_D_IPO '4'
                // ///正回购
                // #define TORA_TSTP_D_Repurchase '5'
                // ///逆回购
                // #define TORA_TSTP_D_ReverseRepur '6'
                // ///开放式基金申购
                // #define TORA_TSTP_D_OeFundPur '8'
                // ///开放式基金赎回
                // #define TORA_TSTP_D_OeFundRed '9'
                // ///担保品划入
                // #define TORA_TSTP_D_CollateralIn 'a'
                // ///担保品划出
                // #define TORA_TSTP_D_CollateralOut 'b'
                // ///质押入库
                // #define TORA_TSTP_D_PledgeIn 'd'
                // ///质押出库
                // #define TORA_TSTP_D_PledgeOut 'e'
                // ///配股配债
                // #define TORA_TSTP_D_Rationed 'f'
                // ///基金拆分
                // #define TORA_TSTP_D_Split 'g'
                // ///基金合并
                // #define TORA_TSTP_D_Merge 'h'
                // ///融资买入
                // #define TORA_TSTP_D_MarginBuy 'i'
                // ///融券卖出
                // #define TORA_TSTP_D_ShortSell 'j'
                // ///卖券还款
                // #define TORA_TSTP_D_SellRepayment 'k'
                // ///买券还券
                // #define TORA_TSTP_D_BuyRepayment 'l'
                // ///还券划转
                // #define TORA_TSTP_D_SecurityRepay 'm'
                // ///余券划转
                // #define TORA_TSTP_D_RemainTransfer 'n'
                // ///债券转股
                // #define TORA_TSTP_D_BondConvertStock 't'
                // ///债券回售
                // #define TORA_TSTP_D_BondPutback 'u'
                // ///ETF实物申购
                // #define TORA_TSTP_D_ETFOtPur 'v'
                // ///ETF实物赎回
                // #define TORA_TSTP_D_ETFOtRed 'w'
                // ///回售撤销
                // #define TORA_TSTP_D_PutbackRelieve 'x'
            }

            input.Direction = match i.direction {
                DirectionType::Long => TORASTOCKAPI_TORA_TSTP_D_Buy,
                DirectionType::Short => TORASTOCKAPI_TORA_TSTP_D_Sell,
            } as i8;
            input.VolumeTotalOriginal = i.volume;
            input.LimitPrice = i.price;
            input.OrderPriceType = TORASTOCKAPI_TORA_TSTP_OPT_LimitPrice as i8;
            input.TimeCondition = TORASTOCKAPI_TORA_TSTP_TC_GFD as i8;
            input.VolumeCondition = TORASTOCKAPI_TORA_TSTP_VC_AV as i8;
            let sa = shareholder_accounts
                .iter()
                .find(|x| x.exchange_id == exchange)
                .ok_or(base::error::Error::ShareholderAccountNotFound)?;
            set_cstr_from_str_truncate_i8(&mut input.ShareholderID, &sa.shareholder_id);
            let ret = self.req_order_insert(&mut input, n_request_id);
            if ret != 0 {
                return Err(base::error::Error::TraderApiErr(ret));
            }
            Ok(())
        }

        fn req_order_action(
            &mut self,
            _broker_id: &str,
            _account: &str,
            exchange: &str,
            _symbol: &str,
            i: &InputOrderActionField,
            order_action_ref: i32,
            n_request_id: i32,
        ) -> Result<(), base::error::Error> {
            let mut r = TORASTOCKAPI_CTORATstpInputOrderActionField::default();
            r.ExchangeID = to_tora_stock_exchange_id(exchange);
            r.FrontID = i.front_id;
            r.SessionID = i.session_id;
            r.OrderRef = get_ascii_str(&i.order_ref).unwrap().parse::<i32>().unwrap();
            r.ActionFlag = TORASTOCKAPI_TORA_TSTP_AF_Delete as i8;
            r.OrderActionRef = order_action_ref;
            let ret = self.req_order_action(&mut r, n_request_id);
            if ret != 0 {
                return Err(base::error::Error::TraderApiErr(ret));
            }
            Ok(())
        }
    }

    pub async fn run_md_cache(
        ca: TradingAccountConfig,
        mut rx: mpsc::Receiver<UniqueSymbol>,
        cmc: Arc<Mutex<MdCache>>,
    ) {
        info!(
            "[{}] quote_daemon started, server={:?}",
            ca.account, &ca.md_fronts
        );
        // info!("要订阅的Symbol数量={}", v_instruments.len(),);
        let flow_path = format!(".cache/tora_md_flow_{}_{}//", ca.broker_id, ca.account);
        check_make_dir(&flow_path);
        let md_front = ca.md_fronts.first().expect("至少要配置一个md front");
        let (md_sub_mode, derive_sub_mode) = if md_front.starts_with("udp") {
            (
                Box::new(TORALEV1API_TORA_TSTP_MST_MCAST),
                Box::new(TORALEV1API_TORA_TSTP_MST_TCP),
            )
        } else {
            (
                Box::new(TORALEV1API_TORA_TSTP_MST_TCP),
                Box::new(TORALEV1API_TORA_TSTP_MST_TCP),
            )
        };
        let mut mdapi = unsafe {
            Box::from_raw(TORALEV1API_CTORATstpXMdApi_CreateTstpXMdApi(
                Box::into_raw(md_sub_mode) as *const TORALEV1API_TTORATstpMDSubModeType,
                Box::into_raw(derive_sub_mode) as *const TORALEV1API_TTORATstpMDSubModeType,
            ))
        };
        if md_front.starts_with("udp") {
            // 格式形如此 udp://224.224.1.11:7880;10.188.82.150;10.188.82.8
            // split之后使用
            let arr = md_front.split(";").collect::<Vec<_>>();
            if arr.len() != 3 {
                panic!("udp组播行情地址格式不对,需要同时提供网卡地址 md_front={md_front}",);
            }
            info!(
                "Tora stock md [{}]:行情RegisterMulticast {} {} {}",
                ca.account, arr[0], arr[1], arr[2]
            );
            mdapi.register_multicast(&arr[0], &arr[1], &arr[2]);
        } else if ca.fens_md_fronts.len() > 0 {
            mdapi.register_name_server(&ca.fens_md_fronts[0]);
        } else {
            mdapi.register_front(md_front);
        }

        let mut stream = {
            let (stream, pp) = md_api::create_spi();
            mdapi.register_spi(pp);
            stream
        };
        mdapi.init();
        let mut initialized = false;
        while let Some(msg) = stream.next().await {
            use md_api::TORALEV1API_CTORATstpXMdSpiOutput::*;
            match msg {
                OnFrontConnected(ref _p) => {
                    info!("[{}] 行情服务器FrontConnected", ca.account);
                    let mut req_login: TORALEV1API_CTORATstpReqUserLoginField = Default::default();
                    let ret = mdapi.req_user_login(&mut req_login, 3);
                    if ret != 0 {
                        error!("行情ReqUserLogin={}", ret);
                    }
                }
                OnFrontDisconnected(_p) => {
                    break;
                }
                OnRspUserLogin(ref p) => {
                    if p.p_rsp_info_field.is_some() {
                        let u = p.p_rsp_user_login_field.unwrap();
                        info!(
                            "ToraStock 行情服务器登陆成功, trading_day=[{}]",
                            get_ascii_str(&u.TradingDay).unwrap()
                        );
                        initialized = true;
                    } else {
                        error!(
                            "Tora Stock Md RspUserLogin = {:?}",
                            print_rsp_info!(&p.p_rsp_info_field)
                        );
                    }
                    break;
                }
                _ => {
                    info!("msg={:?}", msg)
                }
            }
        }
        if !initialized {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            error!("[{}] tora md failed to initialize", ca.account);
            info!("Restart tora md cache in 10 seconds");
            test_spawn(ca, rx, cmc);
            return;
        }
        loop {
            tokio::select! {
                Some(msg) = stream.next() => {
                    use md_api::TORALEV1API_CTORATstpXMdSpiOutput::*;
                    match msg {
                        OnFrontDisconnected(_p) => {
                            break;
                        }
                        OnRtnMarketData(md) =>{
                            if let Some(md) = md.p_market_data_field {
                                let us = UniqueSymbol::new(exchange_from_tora_stock_exchange_id(md.ExchangeID), get_ascii_str(&md.SecurityID).unwrap());
                                let us1 = us.clone();
                                let mut cmc = cmc.lock().await;
                                cmc.hm_md
                                    .entry(us)
                                    .and_modify(|e| {
                                        e.ask1 = md.AskPrice1;
                                        e.bid1 = md.BidPrice1;
                                        e.ask1_volume1 = md.AskVolume1 as i64;
                                        e.bid1_volume = md.BidVolume1 as i64;
                                        e.timestamp = Local::now().timestamp();
                                    })
                                    .or_insert_with(|| {
                                        let md = MarketDataSnapshot {
                                            ask1: md.AskPrice1,
                                            bid1: md.BidPrice1,
                                            ask1_volume1: md.AskVolume1,
                                            bid1_volume: md.BidVolume1,
                                            timestamp: Local::now().timestamp(),
                                        };
                                        info!("[{}:{}] insert md = {:?}",us1.exchange, us1.symbol, md);
                                        md
                                    });
                            }
                        }
                        OnRtnSPMarketData(md) => {
                            if let Some(md) = md.p_market_data_field {
                                let us = UniqueSymbol::new(exchange_from_tora_stock_exchange_id(md.ExchangeID), get_ascii_str(&md.SecurityID).unwrap());
                                let us1 = us.clone();
                                let mut cmc = cmc.lock().await;
                                cmc.hm_md
                                    .entry(us)
                                    .and_modify(|e| {
                                        e.ask1 = md.AskPrice1;
                                        e.bid1 = md.BidPrice1;
                                        e.ask1_volume1 = md.AskVolume1 as i64;
                                        e.bid1_volume = md.BidVolume1 as i64;
                                        e.timestamp = Local::now().timestamp();
                                    })
                                    .or_insert_with(|| {
                                        let md = MarketDataSnapshot {
                                            ask1: md.AskPrice1,
                                            bid1: md.BidPrice1,
                                            ask1_volume1: md.AskVolume1,
                                            bid1_volume: md.BidVolume1,
                                            timestamp: Local::now().timestamp(),
                                        };
                                        info!("[{}:{}] insert md = {:?}",us1.exchange, us1.symbol, md);
                                        md
                                    });
                            }
                        }
                        OnRspSubMarketData(_p) =>{
                        }
                        other => error!("{:?}", other),
                    }
                }
                Some(us) = rx.recv() => {
                    let exchangeid = to_tora_stock_exchange_id(&us.exchange);
                    let mut sa = StringArray::new();
                    sa.push(&us.symbol);
                    let result = mdapi
                        .subscribe_market_data(&mut sa, 1, exchangeid);
                    if result != 0 {
                        error!("Tora Stock subscribe result = {}", result);
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
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

    pub async fn handle_spi_msg(
        spi_msg: &trader_api::TORASTOCKAPI_CTORATstpTraderSpiOutput,
        state: &mut AccountStateType,
        cmc: &Arc<Mutex<MdCache>>,
        api: &mut Box<dyn TraderApiType + Send>,
    ) -> Result<(), base::error::Error> {
        use trader_api::TORASTOCKAPI_CTORATstpTraderSpiOutput::*;
        match spi_msg {
            OnFrontDisconnected(p) => {
                info!(
                    "{}:{} on front disconnected {:?} 直接Exit ",
                    state.broker_id, state.account, p
                );
                return Err(base::error::Error::FrontDisconnected);
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
                    print_rsp_info!(&p.p_rsp_info_field)
                );
            }
            OnRspOrderInsert(ref p) => {
                // 需要构建order通知撤单
                info!(
                    "{}:{} RspOrderInsert={:?}",
                    state.broker_id,
                    state.account,
                    print_rsp_info!(&p.p_rsp_info_field)
                );
            }
            OnRspOrderAction(ref p) => {
                info!(
                    "{}:{} OnRspOrderAction={:?}",
                    state.broker_id,
                    state.account,
                    print_rsp_info!(&p.p_rsp_info_field)
                );
            }
            OnErrRtnOrderInsert(ref p) => {
                // 需要构建order通知撤单
                if let Some(input) = p.p_input_order_field {
                    let us = UniqueSymbol::new(
                        exchange_from_tora_stock_exchange_id(input.ExchangeID),
                        get_ascii_str(&input.SecurityID).unwrap(),
                    );
                    let mut order_ref: [i8; 13] = [0; 13];
                    set_cstr_from_str_truncate_i8(&mut order_ref, &format!("{}", input.OrderRef));
                    state.remove_po(&us, state.front_id, state.session_id, &order_ref);
                    info!(
                        "{}:{} 删除发送失败的委托 OrderRef={} result={:?}",
                        state.broker_id,
                        state.account,
                        input.OrderRef,
                        print_rsp_info!(&p.p_rsp_info_field)
                    );
                }
            }
            OnErrRtnOrderAction(ref p) => {
                // info!("ErrRtnOrderAction={:?}", print_rsp_info!(&p.p_rsp_info_field));
                info!(
                    "{}:{} ErrRtnOrderAction={:?} {:?}",
                    state.broker_id,
                    state.account,
                    p,
                    print_rsp_info!(&p.p_rsp_info_field)
                );
            }
            OnRtnOrder(ref rtn) => {
                if let Some(o) = rtn.p_order_field {
                    info!(
                        "OnRtnOrder o.sys_id={} status={} VolumeCanceled={}",
                        ascii_cstr_to_str_i8(&o.OrderSysID).unwrap(),
                        o.OrderStatus,
                        o.VolumeCanceled
                    );
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
                        if submit_status == TORASTOCKAPI_TORA_TSTP_OSS_CancelRejected as i8
                            || status_msg.contains("当前状态禁止")
                            || status_msg.contains("废单")
                        {
                            info!("非交易时间不发单, OrderSubmitStatus={}", submit_status);
                        } else {
                            // 价格变化导致的撤单，要及时重新发单
                            let us = UniqueSymbol::new(
                                exchange_from_tora_stock_exchange_id(o.ExchangeID),
                                get_ascii_str(&o.SecurityID).expect("OnRtnOrder get_ascii_str"),
                            );
                            if let Err(e) = state.set_check_target(us, None, &cmc, api).await {
                                error!("OnRtnOrder set_check_target {e}");
                            }
                        }
                    }
                }
            }
            OnRtnTrade(rtn) => {
                if let Some(trade) = rtn.p_trade_field {
                    info!(
                        "OnRtnTrade OrderSysID={} Volume={}",
                        ascii_cstr_to_str_i8(&trade.OrderSysID).unwrap(),
                        trade.Volume
                    );
                    let changed = state.update_by_trade(trade).unwrap();
                    if changed {
                        let us = UniqueSymbol::new(
                            exchange_from_tora_stock_exchange_id(trade.ExchangeID),
                            get_ascii_str(&trade.SecurityID).expect("OnRtnTrade get_ascii_str"),
                        );
                        if let Err(e) = state.set_check_target(us, None, &cmc, api).await {
                            error!("OnRtnTrade set_check_target {e}");
                        }
                    }
                }
            }
            OnRspQryTradingAccount(ref _p) => {}
            OnRspQryPosition(p) => {
                if p.p_position_field.is_none() {
                    error!(
                        "OnRspQryPosition null p_position_field is_last={}",
                        p.b_is_last
                    );
                }
            }
            OnRtnMarketStatus(_p) => {}
            other => {
                info!("tora stock unhandled spi msg = {:?}", other);
            }
        };
        Ok(())
    }

    pub async fn handle_request_msg(
        req_msg: &ReqMessage,
        state: &mut AccountStateType,
        api: &mut Box<TORASTOCKAPI_CTORATstpTraderApi>,
    ) -> Result<(), base::error::Error> {
        use ReqMessage::*;
        match req_msg {
            SetContractTarget(_target) => {}
            QueryPositionDetail => {
                info!("req_msg={:?}", req_msg);
                let mut req = TORASTOCKAPI_CTORATstpQryPositionField::default();
                let result = api.req_qry_position(&mut req, state.get_request_id());
                if result != 0 {
                    info!("tora stock req_qry_position={}", result);
                }
            }
            QueryTradingAccount => {
                let mut req = TORASTOCKAPI_CTORATstpQryTradingAccountField::default();
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &state.account);
                let result = api.req_qry_trading_account(&mut req, state.get_request_id());
                if result != 0 {
                    info!("tora stock req_qry_trading_account={}", result);
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
    ) -> Result<(), base::error::Error> {
        info!("Run tora stock trader [{}]", ca.account);
        let mut state = AccountStateType::new(&ca.broker_id, &ca.account);
        let flow_path = format!(
            ".cache/tora_stock_trade_flow_{}_{}//",
            ca.broker_id, ca.account
        );
        check_make_dir(&flow_path);
        let mut api = trader_api::create_api(&flow_path, false);
        let mut stream = {
            let (stream, pp) = trader_api::create_spi();
            api.register_spi(pp);
            stream
        };
        if let Some(f) = ca.fens_trade_fronts.first() {
            api.register_name_server(f);
            info!("Tora stock register name server {f}");
        } else if let Some(f) = ca.trade_fronts.first() {
            api.register_front(f);
            info!("Tora stock register front {f}");
        }
        api.subscribe_public_topic(TORALEV1API_TORA_TE_RESUME_TYPE_TORA_TERT_QUICK);
        api.subscribe_private_topic(TORALEV1API_TORA_TE_RESUME_TYPE_TORA_TERT_QUICK);
        api.init();
        // 处理登陆初始化查询
        let mut cached_pdl: Vec<PositionDetail> = vec![];
        let mut cached_orders: Vec<TORASTOCKAPI_CTORATstpOrderField> = vec![];
        let mut cached_trades: Vec<TORASTOCKAPI_CTORATstpTradeField> = vec![];

        loop {
            match tokio::time::timeout(tokio::time::Duration::from_secs(10), stream.next()).await {
                Ok(Some(spi_msg)) => {
                    use trader_api::TORASTOCKAPI_CTORATstpTraderSpiOutput::*;
                    match spi_msg {
                        OnFrontConnected(_p) => {
                            let account = &ca;
                            let mut req = TORASTOCKAPI_CTORATstpReqUserLoginField::default();
                            req.LogInAccountType = TORALEV1API_TORA_TSTP_LACT_UserID as i8;
                            set_cstr_from_str_truncate_i8(&mut req.LogInAccount, &account.account);
                            set_cstr_from_str_truncate_i8(&mut req.Password, &account.password);
                            set_cstr_from_str_truncate_i8(
                                &mut req.UserProductInfo,
                                &account.user_product_info,
                            );
                            set_cstr_from_str_truncate_i8(
                                &mut req.InnerIPAddress,
                                account.inner_ip_address.as_str(),
                            );
                            set_cstr_from_str_truncate_i8(
                                &mut req.TerminalInfo,
                                account.terminal_info.as_str(),
                            );
                            set_cstr_from_str_truncate_i8(
                                &mut req.MacAddress,
                                account.mac_address.as_str(),
                            );
                            api.req_user_login(&mut req, state.get_request_id());
                        }
                        OnFrontDisconnected(p) => {
                            info!("tora stock on front disconnected {:?} 直接Exit ", p);
                            std::process::exit(-1);
                        }
                        OnRspUserLogin(ref p) => {
                            if p.p_rsp_info_field.as_ref().unwrap().ErrorID == 0 {
                                let u = p.p_rsp_user_login_field.as_ref().unwrap();
                                let mut max_order_ref: [i8; 13] = [0; 13];
                                set_cstr_from_str_truncate_i8(
                                    &mut max_order_ref,
                                    &format!("{}", u.MaxOrderRef),
                                );
                                state.on_login(
                                    u.FrontID,
                                    u.SessionID,
                                    &max_order_ref,
                                    &u.TradingDay,
                                );
                                info!(
                                    "{}:{} tora stock trader 登陆成功 trading_day={} front_id={} session_id={}",
                                    state.broker_id,
                                    state.account,
                                    state.trading_day_i32,
                                    state.front_id,
                                    state.session_id
                                );
                            } else {
                                info!(
                                    "Trade RspUserLogin = {:?}",
                                    print_rsp_info!(&p.p_rsp_info_field)
                                );
                            }
                            let mut req = TORASTOCKAPI_CTORATstpQrySecurityField::default();
                            let ret = api.req_qry_security(&mut req, state.get_request_id());
                            if ret != 0 {
                                error!("tora stock req_qry_security = {ret}");
                            }
                        }
                        OnRspQrySecurity(ref p) => {
                            if let Some(i) = &p.p_security_field {
                                let xif = InstrumentField {
                                    price_tick: i.PriceTick,
                                };
                                let us = UniqueSymbol::new(
                                    exchange_from_tora_stock_exchange_id(i.ExchangeID),
                                    get_ascii_str(&i.SecurityID).unwrap(),
                                );
                                state.hm_inst.insert(us, xif);
                            }
                            if p.b_is_last {
                                let mut req = TORASTOCKAPI_CTORATstpQryPositionField::default();
                                let ret = api.req_qry_position(&mut req, state.get_request_id());
                                if ret != 0 {
                                    info!("tora stock req_qry_position={}", ret);
                                }
                            }
                        }
                        OnRspQryPosition(ref p) => {
                            if let Some(p) = p.p_position_field {
                                // info!(
                                //     "{} TodayBSPos={} HistoryPos={}",
                                //     get_ascii_str(&p.SecurityID).unwrap(),
                                //     p.TodayBSPos,
                                //     p.HistoryPos
                                // );
                                let (tpd, ypd) = make_position_detail(&p);
                                cached_pdl.push(tpd);
                                cached_pdl.push(ypd);
                            }
                            if p.b_is_last {
                                // 流控
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                let mut req = TORASTOCKAPI_CTORATstpQryOrderField::default();
                                let ret = api.req_qry_order(&mut req, state.get_request_id());
                                if ret != 0 {
                                    info!("Tora stock req_qry_order={}", ret);
                                }
                            }
                        }
                        OnRspQryOrder(ref p) => {
                            if let Some(p) = p.p_order_field {
                                // info!(
                                //     "sysid={} status={} VolumeCanceled={}",
                                //     ascii_cstr_to_str_i8(&p.OrderSysID).unwrap(),
                                //     p.OrderStatus,
                                //     p.VolumeCanceled
                                // );
                                cached_orders.push(p);
                            }
                            if p.b_is_last {
                                let mut req = TORASTOCKAPI_CTORATstpQryTradeField::default();
                                let ret = api.req_qry_trade(&mut req, state.get_request_id());
                                if ret != 0 {
                                    info!("Tora stock req_qry_trade={}", ret);
                                }
                            }
                        }
                        OnRspQryTrade(p) => {
                            if let Some(p) = p.p_trade_field {
                                cached_trades.push(p);
                            }
                            if p.b_is_last {
                                let mut req =
                                    TORASTOCKAPI_CTORATstpQryShareholderAccountField::default();
                                let ret = api
                                    .req_qry_shareholder_account(&mut req, state.get_request_id());
                                if ret != 0 {
                                    error!("tora req_qry_shareholder_account {}", ret);
                                }
                            }
                        }
                        OnRspQryShareholderAccount(ref p) => {
                            if let Some(sa) = p.p_shareholder_account_field {
                                let investor_id =
                                    ascii_cstr_to_str_i8(&sa.InvestorID).unwrap().to_string();
                                let exchange_id =
                                    exchange_from_tora_stock_exchange_id(sa.ExchangeID).to_string();
                                let shareholder_id =
                                    ascii_cstr_to_str_i8(&sa.ShareholderID).unwrap().to_string();
                                let sa = ShareholderAccount {
                                    investor_id,
                                    exchange_id,
                                    shareholder_id,
                                    shareholder_id_type: sa.ShareholderIDType,
                                    market_id: sa.MarketID,
                                };
                                state.shareholder_accounts.push(sa);
                            }
                            if p.b_is_last {
                                break;
                            }
                        }

                        OnRtnOrder(p) => {
                            if let Some(o) = p.p_order_field {
                                cached_orders.push(o);
                            }
                        }
                        OnRtnTrade(p) => {
                            if let Some(trade) = p.p_trade_field {
                                cached_trades.push(trade);
                            }
                        }
                        OnRtnTradingNotice(ref p) => {
                            info!("RtnTradingNotice = {:?}", p);
                        }
                        _ => {}
                    }
                }
                Ok(None) => {}
                Err(_e) => {
                    api.release();
                    Box::leak(api);
                    return Err(base::error::Error::InitTraderFailed);
                }
            }
        }
        // 初始化查询过程中推送的成交
        info!(
            "cached_orders={} cached_trades={}",
            cached_orders.len(),
            cached_trades.len()
        );
        if let Err(e) = state.update_on_initialized(cached_pdl, cached_orders, cached_trades) {
            error!("Cached orders update {e}");
        }
        info!("{} 初始化查询完成.", ca.account);
        let (api, _api2) = trader_api::unsafe_clone_api(api);
        let (api, mut api3) = trader_api::unsafe_clone_api(api);
        let mut api = api as Box<dyn TraderApiType + Send>;

        {
            let mut cmc = cmc.lock().await;
            for cd in state.sorted_cds.iter() {
                cmc.subscribe(&cd.us.exchange, &cd.us.symbol).await;
            }
        }

        let mut query_req: Option<(
            ReqMessage,
            oneshot::Sender<RspMessage>,
            Vec<trader_api::TORASTOCKAPI_CTORATstpTraderSpiOutput>,
        )> = None;
        loop {
            tokio::select! {
                Some(spi_msg) = stream.next() => {
                    let _ = handle_spi_msg(&spi_msg, &mut state, &cmc, &mut api).await?;
                    use trader_api::TORASTOCKAPI_CTORATstpTraderSpiOutput::*;
                    use ReqMessage::*;
                    let is_last = if let Some((req_msg, _rsp_tx, ref mut response_packets)) = &mut query_req {
                        let (is_result, is_last) = match (req_msg, &spi_msg) {
                            (SetContractTarget(_), _) => panic!("SetContractTarget do not have response"),
                            (QueryPositionDetail, OnRspQryPosition(p)) => (true, p.b_is_last),
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
                        if let Some((_req_msg, rsp_tx, response_packets)) = query_req.take() {
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
                            let _ = rsp_tx.send(Err(base::error::Error::CtpLastQueryIsProceeding));
                        } else {
                            query_req = Some((req_msg, rsp_tx, vec![]));
                            handle_request_msg(&query_req.as_ref().unwrap().0, &mut state, &mut api3).await?;
                        }
                    }
                },
                else => break,
            }
        }

        api3.join();
        api3.release();
        info!("{}:{} trader_deamon退出", ca.broker_id, ca.account);
        Ok(())
    }
}

pub mod query {
    use super::*;
    use base::*;
    use bincode::{Decode, Encode};
    use futures::StreamExt;
    use log::{error, info};

    /// 查询结果
    #[derive(Clone, Debug, Default, Encode, Decode)]
    pub struct ToraStockQueryResult {
        pub broker_id: String,
        pub account: String,
        pub trading_day: i32,
        pub timestamp: i64,
        // pub dmd_list: Vec<TORALEV1API_CTORATstpMarketDataField>,
        // pub icr_list: Vec<CThostFtdcInstrumentCommissionRateField>,
        pub instruments: Vec<TORASTOCKAPI_CTORATstpSecurityField>,
        pub positions: Vec<TORASTOCKAPI_CTORATstpPositionField>,
        // pub position_detail_list: Vec<CThostFtdcInvestorPositionDetailField>,
        pub trading_account: TORASTOCKAPI_CTORATstpTradingAccountField,
        // pub products: Vec<CThostFtdcProductField>,
        pub orders: Vec<TORASTOCKAPI_CTORATstpOrderField>,
        pub trades: Vec<TORASTOCKAPI_CTORATstpTradeField>,
    }

    pub async fn query(
        ca: &TradingAccountConfig,
    ) -> Result<ToraStockQueryResult, base::error::Error> {
        let mut request_id: i32 = 0;
        let mut get_request_id = || {
            request_id += 1;
            request_id
        };

        let flow_path = format!(
            ".cache/tora_stock_trade_flow_{}_{}//",
            ca.broker_id, ca.account
        );
        check_make_dir(&flow_path);
        let mut api = trader_api::create_api(&flow_path, false);
        let mut stream = {
            let (stream, pp) = trader_api::create_spi();
            api.register_spi(pp);
            stream
        };
        if let Some(f) = ca.fens_trade_fronts.first() {
            api.register_name_server(f);
            info!("Tora stock register name server {f}");
        } else if let Some(f) = ca.trade_fronts.first() {
            api.register_front(f);
            info!("Tora stock register front {f}");
        }
        api.subscribe_public_topic(TORALEV1API_TORA_TE_RESUME_TYPE_TORA_TERT_QUICK);
        api.subscribe_private_topic(TORALEV1API_TORA_TE_RESUME_TYPE_TORA_TERT_QUICK);
        api.init();
        let mut result = ToraStockQueryResult::default();
        result.broker_id = ca.broker_id.clone();
        result.account = ca.account.clone();

        while let Some(spi_msg) = stream.next().await {
            use trader_api::TORASTOCKAPI_CTORATstpTraderSpiOutput::*;
            match spi_msg {
                OnFrontConnected(_p) => {
                    let account = &ca;
                    let mut req = TORASTOCKAPI_CTORATstpReqUserLoginField::default();
                    req.LogInAccountType = TORALEV1API_TORA_TSTP_LACT_UserID as i8;
                    set_cstr_from_str_truncate_i8(&mut req.LogInAccount, &account.account);
                    set_cstr_from_str_truncate_i8(&mut req.Password, &account.password);
                    set_cstr_from_str_truncate_i8(
                        &mut req.UserProductInfo,
                        &account.user_product_info,
                    );
                    set_cstr_from_str_truncate_i8(
                        &mut req.InnerIPAddress,
                        account.inner_ip_address.as_str(),
                    );
                    set_cstr_from_str_truncate_i8(
                        &mut req.TerminalInfo,
                        account.terminal_info.as_str(),
                    );
                    set_cstr_from_str_truncate_i8(
                        &mut req.MacAddress,
                        account.mac_address.as_str(),
                    );
                    api.req_user_login(&mut req, get_request_id());
                }
                OnFrontDisconnected(p) => {
                    info!("tora stock on front disconnected {:?} 直接Exit ", p);
                    return Err(base::error::Error::FrontDisconnected);
                }
                OnRspUserLogin(ref p) => {
                    if p.p_rsp_info_field.as_ref().unwrap().ErrorID == 0 {
                        let u = p.p_rsp_user_login_field.as_ref().unwrap();
                        result.trading_day = get_ascii_str(&u.TradingDay)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                        info!(
                            "{}:{} tora stock trader 登陆成功 trading_day={} front_id={} session_id={}",
                            result.broker_id,
                            result.account,
                            result.trading_day,
                            u.FrontID,
                            u.SessionID
                        );
                    } else {
                        error!(
                            "Trade RspUserLogin = {:?}",
                            print_rsp_info!(&p.p_rsp_info_field)
                        );
                        return Err(base::error::Error::LoginFailed);
                    }
                    let mut req = TORASTOCKAPI_CTORATstpQrySecurityField::default();
                    let ret = api.req_qry_security(&mut req, get_request_id());
                    if ret != 0 {
                        error!("tora stock req_qry_security = {ret}");
                    }
                }
                OnRspQrySecurity(ref p) => {
                    if let Some(i) = p.p_security_field {
                        result.instruments.push(i);
                    }
                    if p.b_is_last {
                        let mut req = TORASTOCKAPI_CTORATstpQryPositionField::default();
                        let ret = api.req_qry_position(&mut req, get_request_id());
                        if ret != 0 {
                            error!("tora stock req_qry_position={}", ret);
                        }
                    }
                }
                OnRspQryPosition(ref p) => {
                    if let Some(p) = p.p_position_field {
                        result.positions.push(p);
                    }
                    if p.b_is_last {
                        // 流控
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        let mut req = TORASTOCKAPI_CTORATstpQryOrderField::default();
                        let ret = api.req_qry_order(&mut req, get_request_id());
                        if ret != 0 {
                            error!("Tora stock req_qry_order={}", ret);
                        }
                    }
                }
                OnRspQryOrder(ref p) => {
                    if let Some(p) = p.p_order_field {
                        result.orders.push(p);
                    }
                    if p.b_is_last {
                        let mut req = TORASTOCKAPI_CTORATstpQryTradeField::default();
                        let ret = api.req_qry_trade(&mut req, get_request_id());
                        if ret != 0 {
                            error!("Tora stock req_qry_trade={}", ret);
                        }
                    }
                }
                OnRspQryTrade(p) => {
                    if let Some(p) = p.p_trade_field {
                        result.trades.push(p);
                    }
                    if p.b_is_last {
                        let mut req = TORASTOCKAPI_CTORATstpQryTradingAccountField::default();
                        set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                        let ret = api.req_qry_trading_account(&mut req, get_request_id());
                        if ret != 0 {
                            error!("tora stock req_qry_trading_account {}", ret);
                        }
                    }
                }
                OnRspQryTradingAccount(p) => {
                    if let Some(p) = p.p_trading_account_field {
                        result.trading_account = p;
                    }
                    if p.b_is_last {
                        break;
                    }
                }
                OnRspQryShareholderAccount(_p) => {}
                OnRtnOrder(_p) => {}
                OnRtnTrade(_p) => {}
                OnRtnTradingNotice(ref p) => {
                    info!("RtnTradingNotice = {:?}", p);
                }
                _ => {}
            }
        }
        info!("{} 查询完成.", ca.account);
        api.join();
        api.release();
        Ok(result)
    }
}

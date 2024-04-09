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
    pub fn create_api(
        flow_path: &str,
        b_is_using_udp: bool,
        b_is_multicast: bool,
    ) -> Box<CThostFtdcMdApi> {
        let md_flow_path = std::ffi::CString::new(flow_path).unwrap();
        unsafe {
            Box::from_raw(CThostFtdcMdApi_CreateFtdcMdApi(
                md_flow_path.as_ptr(),
                b_is_using_udp,
                b_is_multicast,
            ))
        }
    }
}

pub mod trader_api {
    use crate::*;
    include!("trade_impl.rs");
    pub fn create_api(flow_path: &str, b_encrypt: bool) -> Box<CThostFtdcTraderApi> {
        let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
        unsafe {
            Box::from_raw(CThostFtdcTraderApi_CreateFtdcTraderApi(
                trade_flow_path.as_ptr(),
            ))
        }
    }
    pub fn unsafe_clone_api(
        source: Box<CThostFtdcTraderApi>,
    ) -> (Box<CThostFtdcTraderApi>, Box<CThostFtdcTraderApi>) {
        let p = Box::into_raw(source);
        unsafe {
            let p2 = p.clone();
            (Box::from_raw(p), Box::from_raw(p2))
        }
    }

    pub fn get_api_version() -> *const std::os::raw::c_char {
        unsafe { CThostFtdcTraderApi_GetApiVersion() }
    }
}

/// 典型的账户配置
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CtpAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub name_servers: Vec<String>,
    pub trade_fronts: Vec<String>,
    pub md_fronts: Vec<String>,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
    pub remark: String,
}

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

/// 判断CTP委托是否已撤单
pub fn is_order_canceled(o: &CThostFtdcOrderField) -> bool {
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
    match o.OrderStatus as u8 {
        THOST_FTDC_OST_PartTradedNotQueueing | THOST_FTDC_OST_Canceled => true,
        _ => false,
    }
}

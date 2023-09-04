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
}

/// 典型的账户配置
pub struct CtpAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub name_server: String,
    pub trade_front: String,
    pub md_front: String,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
}

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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::os::raw::c_char;

pub mod trader_api {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused_variables, unused_mut)]
    #![allow(dead_code)]

    use crate::*;
    // use share::trader::StringArray;
    include!("trade_impl.rs");
    pub fn create_api(flow_path: &str, b_encrypt: bool) -> Box<TORASPAPI_CTORATstpSPTraderApi> {
        let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
        unsafe {
            Box::from_raw(TORASPAPI_CTORATstpSPTraderApi_CreateTstpSPTraderApi(
                trade_flow_path.as_ptr(),
                b_encrypt,
            ))
        }
    }
    pub fn unsafe_clone_api(
        source: Box<TORASPAPI_CTORATstpSPTraderApi>,
    ) -> (
        Box<TORASPAPI_CTORATstpSPTraderApi>,
        Box<TORASPAPI_CTORATstpSPTraderApi>,
    ) {
        let p = Box::into_raw(source);
        unsafe {
            let p2 = p.clone();
            (Box::from_raw(p), Box::from_raw(p2))
        }
    }
}


impl CThostFtdcMdApi {
                            pub fn release(&mut self) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_Release)(self as *mut CThostFtdcMdApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_Init)(self as *mut CThostFtdcMdApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_Join)(self as *mut CThostFtdcMdApi)
                                        }
                            }
                            pub fn get_trading_day(&mut self) -> *const std::os::raw::c_char {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_GetTradingDay)(self as *mut CThostFtdcMdApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: std::ffi::CString) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_RegisterFront)(self as *mut CThostFtdcMdApi,
                                             psz_front_address.into_raw())
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: std::ffi::CString) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_RegisterNameServer)(self as *mut CThostFtdcMdApi,
                                             psz_ns_address.into_raw())
                                        }
                            }
                            pub fn register_fens_user_info(&mut self, p_fens_user_info: &mut CThostFtdcFensUserInfoField) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_RegisterFensUserInfo)(self as *mut CThostFtdcMdApi,
                                             p_fens_user_info as * mut CThostFtdcFensUserInfoField)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn CThostFtdcMdSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &C_THOST_FTDC_MD_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).CThostFtdcMdApi_RegisterSpi)(self as *mut CThostFtdcMdApi,
                                             p_spi as * mut CThostFtdcMdSpi)
                                            }
                                }
                            pub fn subscribe_market_data(&mut self, pp_instrument_id: Vec<std::ffi::CString>, n_count: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_SubscribeMarketData)(self as *mut CThostFtdcMdApi,
                                             pp_instrument_id.iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8,
                                             n_count)
                                        }
                            }
                            pub fn un_subscribe_market_data(&mut self, pp_instrument_id: Vec<std::ffi::CString>, n_count: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_UnSubscribeMarketData)(self as *mut CThostFtdcMdApi,
                                             pp_instrument_id.iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8,
                                             n_count)
                                        }
                            }
                            pub fn subscribe_for_quote_rsp(&mut self, pp_instrument_id: Vec<std::ffi::CString>, n_count: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_SubscribeForQuoteRsp)(self as *mut CThostFtdcMdApi,
                                             pp_instrument_id.iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8,
                                             n_count)
                                        }
                            }
                            pub fn un_subscribe_for_quote_rsp(&mut self, pp_instrument_id: Vec<std::ffi::CString>, n_count: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_UnSubscribeForQuoteRsp)(self as *mut CThostFtdcMdApi,
                                             pp_instrument_id.iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8,
                                             n_count)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut CThostFtdcReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_ReqUserLogin)(self as *mut CThostFtdcMdApi,
                                             p_req_user_login_field as * mut CThostFtdcReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout: &mut CThostFtdcUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_ReqUserLogout)(self as *mut CThostFtdcMdApi,
                                             p_user_logout as * mut CThostFtdcUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_multicast_instrument(&mut self, p_qry_multicast_instrument: &mut CThostFtdcQryMulticastInstrumentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcMdApi_ReqQryMulticastInstrument)(self as *mut CThostFtdcMdApi,
                                             p_qry_multicast_instrument as * mut CThostFtdcQryMulticastInstrumentField,
                                             n_request_id)
                                        }
                            }} 
                unsafe impl Send for CThostFtdcMdApi {}pub trait CThostFtdcMdSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_heart_beat_warning(&mut self, n_time_lapse : std::os::raw::c_int) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login : Option<&CThostFtdcRspUserLoginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_logout(&mut self, p_user_logout : Option<&CThostFtdcUserLogoutField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_multicast_instrument(&mut self, p_multicast_instrument : Option<&CThostFtdcMulticastInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_error(&mut self, p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_sub_market_data(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_un_sub_market_data(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_sub_for_quote_rsp(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_un_sub_for_quote_rsp(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rtn_depth_market_data(&mut self, p_depth_market_data : Option<&CThostFtdcDepthMarketDataField>) {}
fn on_rtn_for_quote_rsp(&mut self, p_for_quote_rsp : Option<&CThostFtdcForQuoteRspField>) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct CThostFtdcMdSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut CThostFtdcMdSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, n_reason : std::os::raw::c_int ),
                on_heart_beat_warning: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, n_time_lapse : std::os::raw::c_int ),
                on_rsp_user_login: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_rsp_user_login : * const CThostFtdcRspUserLoginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_logout: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_user_logout : * const CThostFtdcUserLogoutField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_multicast_instrument: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_multicast_instrument : * const CThostFtdcMulticastInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_error: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_sub_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_un_sub_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_sub_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_un_sub_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rtn_depth_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_depth_market_data : * const CThostFtdcDepthMarketDataField ),
                on_rtn_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpiFat, p_for_quote_rsp : * const CThostFtdcForQuoteRspField ),
                 } 

        #[derive(Clone, Debug, Decode, Encode)]
        pub enum CThostFtdcMdSpiOutput {OnFrontConnected(CThostFtdcMdSpiOnFrontConnectedPacket),OnFrontDisconnected(CThostFtdcMdSpiOnFrontDisconnectedPacket),OnHeartBeatWarning(CThostFtdcMdSpiOnHeartBeatWarningPacket),OnRspUserLogin(CThostFtdcMdSpiOnRspUserLoginPacket),OnRspUserLogout(CThostFtdcMdSpiOnRspUserLogoutPacket),OnRspQryMulticastInstrument(CThostFtdcMdSpiOnRspQryMulticastInstrumentPacket),OnRspError(CThostFtdcMdSpiOnRspErrorPacket),OnRspSubMarketData(CThostFtdcMdSpiOnRspSubMarketDataPacket),OnRspUnSubMarketData(CThostFtdcMdSpiOnRspUnSubMarketDataPacket),OnRspSubForQuoteRsp(CThostFtdcMdSpiOnRspSubForQuoteRspPacket),OnRspUnSubForQuoteRsp(CThostFtdcMdSpiOnRspUnSubForQuoteRspPacket),OnRtnDepthMarketData(CThostFtdcMdSpiOnRtnDepthMarketDataPacket),OnRtnForQuoteRsp(CThostFtdcMdSpiOnRtnForQuoteRspPacket), } 

            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnHeartBeatWarningPacket {
                pub n_time_lapse : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspUserLoginPacket {
                pub p_rsp_user_login : Option<CThostFtdcRspUserLoginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspUserLogoutPacket {
                pub p_user_logout : Option<CThostFtdcUserLogoutField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspQryMulticastInstrumentPacket {
                pub p_multicast_instrument : Option<CThostFtdcMulticastInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspErrorPacket {
                pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspSubMarketDataPacket {
                pub p_specific_instrument : Option<CThostFtdcSpecificInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspUnSubMarketDataPacket {
                pub p_specific_instrument : Option<CThostFtdcSpecificInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspSubForQuoteRspPacket {
                pub p_specific_instrument : Option<CThostFtdcSpecificInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRspUnSubForQuoteRspPacket {
                pub p_specific_instrument : Option<CThostFtdcSpecificInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRtnDepthMarketDataPacket {
                pub p_depth_market_data : Option<CThostFtdcDepthMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcMdSpiOnRtnForQuoteRspPacket {
                pub p_for_quote_rsp : Option<CThostFtdcForQuoteRspField>,
            }  
static C_THOST_FTDC_MD_SPI_VTABLE: CThostFtdcMdSpiVTable = CThostFtdcMdSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_heart_beat_warning: spi_on_heart_beat_warning,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_qry_multicast_instrument: spi_on_rsp_qry_multicast_instrument,
            on_rsp_error: spi_on_rsp_error,
            on_rsp_sub_market_data: spi_on_rsp_sub_market_data,
            on_rsp_un_sub_market_data: spi_on_rsp_un_sub_market_data,
            on_rsp_sub_for_quote_rsp: spi_on_rsp_sub_for_quote_rsp,
            on_rsp_un_sub_for_quote_rsp: spi_on_rsp_un_sub_for_quote_rsp,
            on_rtn_depth_market_data: spi_on_rtn_depth_market_data,
            on_rtn_for_quote_rsp: spi_on_rtn_for_quote_rsp,
             };
extern "C" fn spi_on_front_connected(spi: *mut CThostFtdcMdSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut CThostFtdcMdSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_heart_beat_warning(spi: *mut CThostFtdcMdSpiFat, n_time_lapse : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_heart_beat_warning(n_time_lapse)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut CThostFtdcMdSpiFat, p_rsp_user_login : * const CThostFtdcRspUserLoginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut CThostFtdcMdSpiFat, p_user_logout : * const CThostFtdcUserLogoutField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_multicast_instrument(spi: *mut CThostFtdcMdSpiFat, p_multicast_instrument : * const CThostFtdcMulticastInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_multicast_instrument(p_multicast_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_error(spi: *mut CThostFtdcMdSpiFat, p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_error(p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_sub_market_data(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_market_data(p_specific_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_un_sub_market_data(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_market_data(p_specific_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_sub_for_quote_rsp(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_for_quote_rsp(p_specific_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_un_sub_for_quote_rsp(spi: *mut CThostFtdcMdSpiFat, p_specific_instrument : * const CThostFtdcSpecificInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_for_quote_rsp(p_specific_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rtn_depth_market_data(spi: *mut CThostFtdcMdSpiFat, p_depth_market_data : * const CThostFtdcDepthMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_depth_market_data(p_depth_market_data.as_ref())
                    }
                }extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut CThostFtdcMdSpiFat, p_for_quote_rsp : * const CThostFtdcForQuoteRspField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_for_quote_rsp(p_for_quote_rsp.as_ref())
                    }
                }

        #[repr(C)]
        pub struct CThostFtdcMdSpiFat {
            vtable: *const CThostFtdcMdSpiVTable,
            pub md_spi_ptr: *mut dyn CThostFtdcMdSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct CThostFtdcMdSpiInner {
            buf: std::collections::VecDeque<CThostFtdcMdSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl CThostFtdcMdSpiInner {
            fn push(&mut self, msg: CThostFtdcMdSpiOutput) {
                self.buf.push_back(msg);
                if let Some(waker) = self.waker.take() {
                    waker.wake()
                }
            }
        }
        
        pub struct CThostFtdcMdSpiStream {
            inner: Arc<Mutex<CThostFtdcMdSpiInner>>,
        }
        
        impl Stream for CThostFtdcMdSpiStream {
            type Item = CThostFtdcMdSpiOutput;
        
            fn poll_next(
                self: Pin<&mut Self>,
                cx: &mut futures::task::Context<'_>,
            ) -> futures::task::Poll<Option<Self::Item>> {
                use futures::task::Poll;
                let mut inner = self.inner.lock().unwrap();
                if let Some(i) = inner.buf.pop_front() {
                    Poll::Ready(Some(i))
                } else {
                    inner.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            }
        
            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, None)
            }
        }
        
        pub fn create_spi() -> (Box<CThostFtdcMdSpiStream>, *mut CThostFtdcMdSpiStream) {
            let i = CThostFtdcMdSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = CThostFtdcMdSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl CThostFtdcMdSpi_trait for CThostFtdcMdSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnFrontConnected( CThostFtdcMdSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnFrontDisconnected( CThostFtdcMdSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_heart_beat_warning(&mut self, n_time_lapse : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnHeartBeatWarning( CThostFtdcMdSpiOnHeartBeatWarningPacket { n_time_lapse:n_time_lapse } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login : Option<&CThostFtdcRspUserLoginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspUserLogin( CThostFtdcMdSpiOnRspUserLoginPacket { p_rsp_user_login:p_rsp_user_login.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout : Option<&CThostFtdcUserLogoutField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspUserLogout( CThostFtdcMdSpiOnRspUserLogoutPacket { p_user_logout:p_user_logout.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_multicast_instrument(&mut self, p_multicast_instrument : Option<&CThostFtdcMulticastInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspQryMulticastInstrument( CThostFtdcMdSpiOnRspQryMulticastInstrumentPacket { p_multicast_instrument:p_multicast_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_error(&mut self, p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspError( CThostFtdcMdSpiOnRspErrorPacket { p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_sub_market_data(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspSubMarketData( CThostFtdcMdSpiOnRspSubMarketDataPacket { p_specific_instrument:p_specific_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_un_sub_market_data(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspUnSubMarketData( CThostFtdcMdSpiOnRspUnSubMarketDataPacket { p_specific_instrument:p_specific_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_sub_for_quote_rsp(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspSubForQuoteRsp( CThostFtdcMdSpiOnRspSubForQuoteRspPacket { p_specific_instrument:p_specific_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_un_sub_for_quote_rsp(&mut self, p_specific_instrument : Option<&CThostFtdcSpecificInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRspUnSubForQuoteRsp( CThostFtdcMdSpiOnRspUnSubForQuoteRspPacket { p_specific_instrument:p_specific_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rtn_depth_market_data(&mut self, p_depth_market_data : Option<&CThostFtdcDepthMarketDataField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRtnDepthMarketData( CThostFtdcMdSpiOnRtnDepthMarketDataPacket { p_depth_market_data:p_depth_market_data.cloned() } ))
                }
            fn on_rtn_for_quote_rsp(&mut self, p_for_quote_rsp : Option<&CThostFtdcForQuoteRspField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcMdSpiOutput::OnRtnForQuoteRsp( CThostFtdcMdSpiOnRtnForQuoteRspPacket { p_for_quote_rsp:p_for_quote_rsp.cloned() } ))
                }
             }

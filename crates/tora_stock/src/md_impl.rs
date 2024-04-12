impl TORALEV1API_CTORATstpXMdApi {
                            pub fn release(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_Release)(self as *mut TORALEV1API_CTORATstpXMdApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_Init)(self as *mut TORALEV1API_CTORATstpXMdApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_Join)(self as *mut TORALEV1API_CTORATstpXMdApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: &str) -> () {
                                    let psz_front_address = CString::new(psz_front_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_RegisterFront)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             psz_front_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: &str) -> () {
                                    let psz_ns_address = CString::new(psz_ns_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_RegisterNameServer)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             psz_ns_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_multicast(&mut self, psz_multicast_address: &str, psz_interface_ip: &str, psz_source_ip: &str) -> () {
                                    let psz_multicast_address = CString::new(psz_multicast_address).unwrap();
let psz_interface_ip = CString::new(psz_interface_ip).unwrap();
let psz_source_ip = CString::new(psz_source_ip).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_RegisterMulticast)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             psz_multicast_address.as_ptr() as *mut c_char,
                                             psz_interface_ip.as_ptr() as *mut c_char,
                                             std::ptr::null::<*mut c_char>() as *mut c_char)
                                        }
                            }
                            pub fn register_derive_server(&mut self, psz_derive_address: &str) -> () {
                                    let psz_derive_address = CString::new(psz_derive_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_RegisterDeriveServer)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             psz_derive_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_derive_multicast(&mut self, psz_multicast_address: &str, psz_interface_ip: &str, psz_source_ip: &str) -> () {
                                    let psz_multicast_address = CString::new(psz_multicast_address).unwrap();
let psz_interface_ip = CString::new(psz_interface_ip).unwrap();
let psz_source_ip = CString::new(psz_source_ip).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_RegisterDeriveMulticast)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             psz_multicast_address.as_ptr() as *mut c_char,
                                             psz_interface_ip.as_ptr() as *mut c_char,
                                             std::ptr::null::<*mut c_char>() as *mut c_char)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn TORALEV1API_CTORATstpXMdSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &TORALEV1API_CTORA_TSTP_X_MD_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_RegisterSpi)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_spi as * mut TORALEV1API_CTORATstpXMdSpi)
                                            }
                                }
                            pub fn req_get_connection_info(&mut self, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqGetConnectionInfo)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut TORALEV1API_CTORATstpReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqUserLogin)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_req_user_login_field as * mut TORALEV1API_CTORATstpReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout_field: &mut TORALEV1API_CTORATstpUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqUserLogout)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_user_logout_field as * mut TORALEV1API_CTORATstpUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn subscribe_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_ph_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribePHMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_ph_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribePHMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_special_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSpecialMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_special_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSpecialMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_simplify_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSimplifyMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_simplify_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSimplifyMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_security_status(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSecurityStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_security_status(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSecurityStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_market_status(&mut self, market_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeMarketStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             market_id)
                                        }
                            }
                            pub fn un_subscribe_market_status(&mut self, market_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeMarketStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             market_id)
                                        }
                            }
                            pub fn subscribe_imc_params(&mut self, market_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeImcParams)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             market_id)
                                        }
                            }
                            pub fn un_subscribe_imc_params(&mut self, market_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeImcParams)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             market_id)
                                        }
                            }
                            pub fn req_inquiry_market_data_mirror(&mut self, p_inquiry_market_data_field: &mut TORALEV1API_CTORATstpInquiryMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqInquiryMarketDataMirror)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_inquiry_market_data_field as * mut TORALEV1API_CTORATstpInquiryMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_ph_market_data_mirror(&mut self, p_inquiry_market_data_field: &mut TORALEV1API_CTORATstpInquiryMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqInquiryPHMarketDataMirror)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_inquiry_market_data_field as * mut TORALEV1API_CTORATstpInquiryMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_special_market_data_mirror(&mut self, p_inquiry_special_market_data_field: &mut TORALEV1API_CTORATstpInquirySpecialMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqInquirySpecialMarketDataMirror)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_inquiry_special_market_data_field as * mut TORALEV1API_CTORATstpInquirySpecialMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn subscribe_sp_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSPMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_sp_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSPMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_sp_simplify_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSPSimplifyMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_sp_simplify_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSPSimplifyMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_sp_security_status(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSPSecurityStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_sp_security_status(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSPSecurityStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_sp_market_status(&mut self, market_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeSPMarketStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             market_id)
                                        }
                            }
                            pub fn un_subscribe_sp_market_status(&mut self, market_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeSPMarketStatus)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             market_id)
                                        }
                            }
                            pub fn req_inquiry_sp_market_data_mirror(&mut self, p_inquiry_market_data_field: &mut TORALEV1API_CTORATstpInquiryMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_ReqInquirySPMarketDataMirror)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             p_inquiry_market_data_field as * mut TORALEV1API_CTORATstpInquiryMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn subscribe_rapid_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeRapidMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_rapid_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeRapidMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn subscribe_funds_flow_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_SubscribeFundsFlowMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }
                            pub fn un_subscribe_funds_flow_market_data(&mut self, pp_security_id: &mut StringArray, n_count: std::os::raw::c_int, exchange_id: std::os::raw::c_char) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORALEV1API_CTORATstpXMdApi_UnSubscribeFundsFlowMarketData)(self as *mut TORALEV1API_CTORATstpXMdApi,
                                             pp_security_id.to_char_pp(),
                                             n_count,
                                             exchange_id)
                                        }
                            }} 
                unsafe impl Send for TORALEV1API_CTORATstpXMdApi {}pub trait TORALEV1API_CTORATstpXMdSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_rsp_get_connection_info(&mut self, p_connection_info_field : Option<&TORALEV1API_CTORATstpConnectionInfoField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&TORALEV1API_CTORATstpRspUserLoginField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&TORALEV1API_CTORATstpUserLogoutField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_sub_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_ph_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_ph_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_special_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_special_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_imc_params(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_imc_params(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_inquiry_market_data_mirror(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_inquiry_ph_market_data_mirror(&mut self, p_ph_market_data_field : Option<&TORALEV1API_CTORATstpPHMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_inquiry_special_market_data_mirror(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpSpecialMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_sub_sp_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_sp_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_sp_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_sp_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_sp_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_sp_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_sub_sp_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_sp_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_inquiry_sp_market_data_mirror(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rtn_market_data(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>) {}
fn on_rtn_ph_market_data(&mut self, p_ph_market_data_field : Option<&TORALEV1API_CTORATstpPHMarketDataField>) {}
fn on_rtn_special_market_data(&mut self, p_special_market_data_field : Option<&TORALEV1API_CTORATstpSpecialMarketDataField>) {}
fn on_rtn_simplify_market_data(&mut self, p_simplify_market_data_field : Option<&TORALEV1API_CTORATstpSimplifyMarketDataField>) {}
fn on_rtn_security_status(&mut self, p_security_status_field : Option<&TORALEV1API_CTORATstpSecurityStatusField>) {}
fn on_rtn_market_status(&mut self, p_market_status_field : Option<&TORALEV1API_CTORATstpMarketStatusField>) {}
fn on_rtn_imc_params(&mut self, p_imc_params_field : Option<&TORALEV1API_CTORATstpImcParamsField>) {}
fn on_rtn_sp_market_data(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>) {}
fn on_rtn_sp_simplify_market_data(&mut self, p_simplify_market_data_field : Option<&TORALEV1API_CTORATstpSimplifyMarketDataField>) {}
fn on_rtn_sp_security_status(&mut self, p_security_status_field : Option<&TORALEV1API_CTORATstpSecurityStatusField>) {}
fn on_rtn_sp_market_status(&mut self, p_market_status_field : Option<&TORALEV1API_CTORATstpMarketStatusField>) {}
fn on_rsp_sub_rapid_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_rapid_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rtn_rapid_market_data(&mut self, p_rapid_market_data_field : Option<&TORALEV1API_CTORATstpRapidMarketDataField>) {}
fn on_rsp_sub_funds_flow_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rsp_un_sub_funds_flow_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) {}
fn on_rtn_funds_flow_market_data(&mut self, p_funds_flow_market_data_field : Option<&TORALEV1API_CTORATstpFundsFlowMarketDataField>) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct TORALEV1API_CTORATstpXMdSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, n_reason : std::os::raw::c_int ),
                on_rsp_get_connection_info: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_connection_info_field : * const TORALEV1API_CTORATstpConnectionInfoField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_login: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_rsp_user_login_field : * const TORALEV1API_CTORATstpRspUserLoginField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_logout: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_user_logout_field : * const TORALEV1API_CTORATstpUserLogoutField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_sub_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_ph_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_ph_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_special_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_special_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_simplify_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_simplify_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_security_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_security_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_market_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_market_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_imc_params: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_imc_params: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_inquiry_market_data_mirror: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_inquiry_ph_market_data_mirror: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_ph_market_data_field : * const TORALEV1API_CTORATstpPHMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_inquiry_special_market_data_mirror: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpSpecialMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_sub_sp_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_sp_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_sp_simplify_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_sp_simplify_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_sp_security_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_sp_security_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_sub_sp_market_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_sp_market_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_inquiry_sp_market_data_mirror: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rtn_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField ),
                on_rtn_ph_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_ph_market_data_field : * const TORALEV1API_CTORATstpPHMarketDataField ),
                on_rtn_special_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_special_market_data_field : * const TORALEV1API_CTORATstpSpecialMarketDataField ),
                on_rtn_simplify_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_simplify_market_data_field : * const TORALEV1API_CTORATstpSimplifyMarketDataField ),
                on_rtn_security_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_security_status_field : * const TORALEV1API_CTORATstpSecurityStatusField ),
                on_rtn_market_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_status_field : * const TORALEV1API_CTORATstpMarketStatusField ),
                on_rtn_imc_params: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_imc_params_field : * const TORALEV1API_CTORATstpImcParamsField ),
                on_rtn_sp_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField ),
                on_rtn_sp_simplify_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_simplify_market_data_field : * const TORALEV1API_CTORATstpSimplifyMarketDataField ),
                on_rtn_sp_security_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_security_status_field : * const TORALEV1API_CTORATstpSecurityStatusField ),
                on_rtn_sp_market_status: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_status_field : * const TORALEV1API_CTORATstpMarketStatusField ),
                on_rsp_sub_rapid_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_rapid_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rtn_rapid_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_rapid_market_data_field : * const TORALEV1API_CTORATstpRapidMarketDataField ),
                on_rsp_sub_funds_flow_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rsp_un_sub_funds_flow_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField ),
                on_rtn_funds_flow_market_data: extern "C" fn(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_funds_flow_market_data_field : * const TORALEV1API_CTORATstpFundsFlowMarketDataField ),
                 } 

        #[derive(Clone, Debug, Decode, Encode)]
        pub enum TORALEV1API_CTORATstpXMdSpiOutput {OnFrontConnected(TORALEV1API_CTORATstpXMdSpiOnFrontConnectedPacket),OnFrontDisconnected(TORALEV1API_CTORATstpXMdSpiOnFrontDisconnectedPacket),OnRspGetConnectionInfo(TORALEV1API_CTORATstpXMdSpiOnRspGetConnectionInfoPacket),OnRspUserLogin(TORALEV1API_CTORATstpXMdSpiOnRspUserLoginPacket),OnRspUserLogout(TORALEV1API_CTORATstpXMdSpiOnRspUserLogoutPacket),OnRspSubMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubMarketDataPacket),OnRspUnSubMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubMarketDataPacket),OnRspSubPHMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubPHMarketDataPacket),OnRspUnSubPHMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubPHMarketDataPacket),OnRspSubSpecialMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubSpecialMarketDataPacket),OnRspUnSubSpecialMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSpecialMarketDataPacket),OnRspSubSimplifyMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubSimplifyMarketDataPacket),OnRspUnSubSimplifyMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSimplifyMarketDataPacket),OnRspSubSecurityStatus(TORALEV1API_CTORATstpXMdSpiOnRspSubSecurityStatusPacket),OnRspUnSubSecurityStatus(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSecurityStatusPacket),OnRspSubMarketStatus(TORALEV1API_CTORATstpXMdSpiOnRspSubMarketStatusPacket),OnRspUnSubMarketStatus(TORALEV1API_CTORATstpXMdSpiOnRspUnSubMarketStatusPacket),OnRspSubImcParams(TORALEV1API_CTORATstpXMdSpiOnRspSubImcParamsPacket),OnRspUnSubImcParams(TORALEV1API_CTORATstpXMdSpiOnRspUnSubImcParamsPacket),OnRspInquiryMarketDataMirror(TORALEV1API_CTORATstpXMdSpiOnRspInquiryMarketDataMirrorPacket),OnRspInquiryPHMarketDataMirror(TORALEV1API_CTORATstpXMdSpiOnRspInquiryPHMarketDataMirrorPacket),OnRspInquirySpecialMarketDataMirror(TORALEV1API_CTORATstpXMdSpiOnRspInquirySpecialMarketDataMirrorPacket),OnRspSubSPMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubSPMarketDataPacket),OnRspUnSubSPMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPMarketDataPacket),OnRspSubSPSimplifyMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubSPSimplifyMarketDataPacket),OnRspUnSubSPSimplifyMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPSimplifyMarketDataPacket),OnRspSubSPSecurityStatus(TORALEV1API_CTORATstpXMdSpiOnRspSubSPSecurityStatusPacket),OnRspUnSubSPSecurityStatus(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPSecurityStatusPacket),OnRspSubSPMarketStatus(TORALEV1API_CTORATstpXMdSpiOnRspSubSPMarketStatusPacket),OnRspUnSubSPMarketStatus(TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPMarketStatusPacket),OnRspInquirySPMarketDataMirror(TORALEV1API_CTORATstpXMdSpiOnRspInquirySPMarketDataMirrorPacket),OnRtnMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnMarketDataPacket),OnRtnPHMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnPHMarketDataPacket),OnRtnSpecialMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnSpecialMarketDataPacket),OnRtnSimplifyMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnSimplifyMarketDataPacket),OnRtnSecurityStatus(TORALEV1API_CTORATstpXMdSpiOnRtnSecurityStatusPacket),OnRtnMarketStatus(TORALEV1API_CTORATstpXMdSpiOnRtnMarketStatusPacket),OnRtnImcParams(TORALEV1API_CTORATstpXMdSpiOnRtnImcParamsPacket),OnRtnSPMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnSPMarketDataPacket),OnRtnSPSimplifyMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnSPSimplifyMarketDataPacket),OnRtnSPSecurityStatus(TORALEV1API_CTORATstpXMdSpiOnRtnSPSecurityStatusPacket),OnRtnSPMarketStatus(TORALEV1API_CTORATstpXMdSpiOnRtnSPMarketStatusPacket),OnRspSubRapidMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubRapidMarketDataPacket),OnRspUnSubRapidMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubRapidMarketDataPacket),OnRtnRapidMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnRapidMarketDataPacket),OnRspSubFundsFlowMarketData(TORALEV1API_CTORATstpXMdSpiOnRspSubFundsFlowMarketDataPacket),OnRspUnSubFundsFlowMarketData(TORALEV1API_CTORATstpXMdSpiOnRspUnSubFundsFlowMarketDataPacket),OnRtnFundsFlowMarketData(TORALEV1API_CTORATstpXMdSpiOnRtnFundsFlowMarketDataPacket), } 

            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspGetConnectionInfoPacket {
                pub p_connection_info_field : Option<TORALEV1API_CTORATstpConnectionInfoField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUserLoginPacket {
                pub p_rsp_user_login_field : Option<TORALEV1API_CTORATstpRspUserLoginField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUserLogoutPacket {
                pub p_user_logout_field : Option<TORALEV1API_CTORATstpUserLogoutField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubPHMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubPHMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSpecialMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSpecialMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSimplifyMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSimplifyMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSecurityStatusPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSecurityStatusPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubMarketStatusPacket {
                pub p_specific_market_field : Option<TORALEV1API_CTORATstpSpecificMarketField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubMarketStatusPacket {
                pub p_specific_market_field : Option<TORALEV1API_CTORATstpSpecificMarketField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubImcParamsPacket {
                pub p_specific_market_field : Option<TORALEV1API_CTORATstpSpecificMarketField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubImcParamsPacket {
                pub p_specific_market_field : Option<TORALEV1API_CTORATstpSpecificMarketField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspInquiryMarketDataMirrorPacket {
                pub p_market_data_field : Option<TORALEV1API_CTORATstpMarketDataField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspInquiryPHMarketDataMirrorPacket {
                pub p_ph_market_data_field : Option<TORALEV1API_CTORATstpPHMarketDataField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspInquirySpecialMarketDataMirrorPacket {
                pub p_market_data_field : Option<TORALEV1API_CTORATstpSpecialMarketDataField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSPMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSPSimplifyMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPSimplifyMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSPSecurityStatusPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPSecurityStatusPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubSPMarketStatusPacket {
                pub p_specific_market_field : Option<TORALEV1API_CTORATstpSpecificMarketField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPMarketStatusPacket {
                pub p_specific_market_field : Option<TORALEV1API_CTORATstpSpecificMarketField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspInquirySPMarketDataMirrorPacket {
                pub p_market_data_field : Option<TORALEV1API_CTORATstpMarketDataField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnMarketDataPacket {
                pub p_market_data_field : Option<TORALEV1API_CTORATstpMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnPHMarketDataPacket {
                pub p_ph_market_data_field : Option<TORALEV1API_CTORATstpPHMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSpecialMarketDataPacket {
                pub p_special_market_data_field : Option<TORALEV1API_CTORATstpSpecialMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSimplifyMarketDataPacket {
                pub p_simplify_market_data_field : Option<TORALEV1API_CTORATstpSimplifyMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSecurityStatusPacket {
                pub p_security_status_field : Option<TORALEV1API_CTORATstpSecurityStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnMarketStatusPacket {
                pub p_market_status_field : Option<TORALEV1API_CTORATstpMarketStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnImcParamsPacket {
                pub p_imc_params_field : Option<TORALEV1API_CTORATstpImcParamsField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSPMarketDataPacket {
                pub p_market_data_field : Option<TORALEV1API_CTORATstpMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSPSimplifyMarketDataPacket {
                pub p_simplify_market_data_field : Option<TORALEV1API_CTORATstpSimplifyMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSPSecurityStatusPacket {
                pub p_security_status_field : Option<TORALEV1API_CTORATstpSecurityStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnSPMarketStatusPacket {
                pub p_market_status_field : Option<TORALEV1API_CTORATstpMarketStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubRapidMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubRapidMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnRapidMarketDataPacket {
                pub p_rapid_market_data_field : Option<TORALEV1API_CTORATstpRapidMarketDataField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspSubFundsFlowMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRspUnSubFundsFlowMarketDataPacket {
                pub p_specific_security_field : Option<TORALEV1API_CTORATstpSpecificSecurityField>,pub p_rsp_info_field : Option<TORALEV1API_CTORATstpRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORALEV1API_CTORATstpXMdSpiOnRtnFundsFlowMarketDataPacket {
                pub p_funds_flow_market_data_field : Option<TORALEV1API_CTORATstpFundsFlowMarketDataField>,
            }  
static TORALEV1API_CTORA_TSTP_X_MD_SPI_VTABLE: TORALEV1API_CTORATstpXMdSpiVTable = TORALEV1API_CTORATstpXMdSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_rsp_get_connection_info: spi_on_rsp_get_connection_info,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_sub_market_data: spi_on_rsp_sub_market_data,
            on_rsp_un_sub_market_data: spi_on_rsp_un_sub_market_data,
            on_rsp_sub_ph_market_data: spi_on_rsp_sub_ph_market_data,
            on_rsp_un_sub_ph_market_data: spi_on_rsp_un_sub_ph_market_data,
            on_rsp_sub_special_market_data: spi_on_rsp_sub_special_market_data,
            on_rsp_un_sub_special_market_data: spi_on_rsp_un_sub_special_market_data,
            on_rsp_sub_simplify_market_data: spi_on_rsp_sub_simplify_market_data,
            on_rsp_un_sub_simplify_market_data: spi_on_rsp_un_sub_simplify_market_data,
            on_rsp_sub_security_status: spi_on_rsp_sub_security_status,
            on_rsp_un_sub_security_status: spi_on_rsp_un_sub_security_status,
            on_rsp_sub_market_status: spi_on_rsp_sub_market_status,
            on_rsp_un_sub_market_status: spi_on_rsp_un_sub_market_status,
            on_rsp_sub_imc_params: spi_on_rsp_sub_imc_params,
            on_rsp_un_sub_imc_params: spi_on_rsp_un_sub_imc_params,
            on_rsp_inquiry_market_data_mirror: spi_on_rsp_inquiry_market_data_mirror,
            on_rsp_inquiry_ph_market_data_mirror: spi_on_rsp_inquiry_ph_market_data_mirror,
            on_rsp_inquiry_special_market_data_mirror: spi_on_rsp_inquiry_special_market_data_mirror,
            on_rsp_sub_sp_market_data: spi_on_rsp_sub_sp_market_data,
            on_rsp_un_sub_sp_market_data: spi_on_rsp_un_sub_sp_market_data,
            on_rsp_sub_sp_simplify_market_data: spi_on_rsp_sub_sp_simplify_market_data,
            on_rsp_un_sub_sp_simplify_market_data: spi_on_rsp_un_sub_sp_simplify_market_data,
            on_rsp_sub_sp_security_status: spi_on_rsp_sub_sp_security_status,
            on_rsp_un_sub_sp_security_status: spi_on_rsp_un_sub_sp_security_status,
            on_rsp_sub_sp_market_status: spi_on_rsp_sub_sp_market_status,
            on_rsp_un_sub_sp_market_status: spi_on_rsp_un_sub_sp_market_status,
            on_rsp_inquiry_sp_market_data_mirror: spi_on_rsp_inquiry_sp_market_data_mirror,
            on_rtn_market_data: spi_on_rtn_market_data,
            on_rtn_ph_market_data: spi_on_rtn_ph_market_data,
            on_rtn_special_market_data: spi_on_rtn_special_market_data,
            on_rtn_simplify_market_data: spi_on_rtn_simplify_market_data,
            on_rtn_security_status: spi_on_rtn_security_status,
            on_rtn_market_status: spi_on_rtn_market_status,
            on_rtn_imc_params: spi_on_rtn_imc_params,
            on_rtn_sp_market_data: spi_on_rtn_sp_market_data,
            on_rtn_sp_simplify_market_data: spi_on_rtn_sp_simplify_market_data,
            on_rtn_sp_security_status: spi_on_rtn_sp_security_status,
            on_rtn_sp_market_status: spi_on_rtn_sp_market_status,
            on_rsp_sub_rapid_market_data: spi_on_rsp_sub_rapid_market_data,
            on_rsp_un_sub_rapid_market_data: spi_on_rsp_un_sub_rapid_market_data,
            on_rtn_rapid_market_data: spi_on_rtn_rapid_market_data,
            on_rsp_sub_funds_flow_market_data: spi_on_rsp_sub_funds_flow_market_data,
            on_rsp_un_sub_funds_flow_market_data: spi_on_rsp_un_sub_funds_flow_market_data,
            on_rtn_funds_flow_market_data: spi_on_rtn_funds_flow_market_data,
             };
extern "C" fn spi_on_front_connected(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_rsp_get_connection_info(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_connection_info_field : * const TORALEV1API_CTORATstpConnectionInfoField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_get_connection_info(p_connection_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_rsp_user_login_field : * const TORALEV1API_CTORATstpRspUserLoginField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_user_logout_field : * const TORALEV1API_CTORATstpUserLogoutField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_sub_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_ph_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_ph_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_ph_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_ph_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_special_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_special_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_special_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_special_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_simplify_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_simplify_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_simplify_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_simplify_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_security_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_security_status(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_security_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_security_status(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_market_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_market_status(p_specific_market_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_market_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_market_status(p_specific_market_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_imc_params(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_imc_params(p_specific_market_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_imc_params(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_imc_params(p_specific_market_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_inquiry_market_data_mirror(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_market_data_mirror(p_market_data_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_inquiry_ph_market_data_mirror(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_ph_market_data_field : * const TORALEV1API_CTORATstpPHMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_ph_market_data_mirror(p_ph_market_data_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_inquiry_special_market_data_mirror(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpSpecialMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_special_market_data_mirror(p_market_data_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_sub_sp_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_sp_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_sp_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_sp_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_sp_simplify_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_sp_simplify_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_sp_simplify_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_sp_simplify_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_sp_security_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_sp_security_status(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_sp_security_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_sp_security_status(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_sp_market_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_sp_market_status(p_specific_market_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_sp_market_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_market_field : * const TORALEV1API_CTORATstpSpecificMarketField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_sp_market_status(p_specific_market_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_inquiry_sp_market_data_mirror(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_sp_market_data_mirror(p_market_data_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rtn_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_market_data(p_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_ph_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_ph_market_data_field : * const TORALEV1API_CTORATstpPHMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_ph_market_data(p_ph_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_special_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_special_market_data_field : * const TORALEV1API_CTORATstpSpecialMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_special_market_data(p_special_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_simplify_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_simplify_market_data_field : * const TORALEV1API_CTORATstpSimplifyMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_simplify_market_data(p_simplify_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_security_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_security_status_field : * const TORALEV1API_CTORATstpSecurityStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_security_status(p_security_status_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_market_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_status_field : * const TORALEV1API_CTORATstpMarketStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_market_status(p_market_status_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_imc_params(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_imc_params_field : * const TORALEV1API_CTORATstpImcParamsField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_imc_params(p_imc_params_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_sp_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_data_field : * const TORALEV1API_CTORATstpMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_sp_market_data(p_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_sp_simplify_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_simplify_market_data_field : * const TORALEV1API_CTORATstpSimplifyMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_sp_simplify_market_data(p_simplify_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_sp_security_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_security_status_field : * const TORALEV1API_CTORATstpSecurityStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_sp_security_status(p_security_status_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_sp_market_status(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_market_status_field : * const TORALEV1API_CTORATstpMarketStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_sp_market_status(p_market_status_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_rapid_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_rapid_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_rapid_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_rapid_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_rapid_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_rapid_market_data_field : * const TORALEV1API_CTORATstpRapidMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_rapid_market_data(p_rapid_market_data_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_sub_funds_flow_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_sub_funds_flow_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_un_sub_funds_flow_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_specific_security_field : * const TORALEV1API_CTORATstpSpecificSecurityField,p_rsp_info_field : * const TORALEV1API_CTORATstpRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_un_sub_funds_flow_market_data(p_specific_security_field.as_ref(),p_rsp_info_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_funds_flow_market_data(spi: *mut TORALEV1API_CTORATstpXMdSpiFat, p_funds_flow_market_data_field : * const TORALEV1API_CTORATstpFundsFlowMarketDataField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_funds_flow_market_data(p_funds_flow_market_data_field.as_ref())
                    }
                }

        #[repr(C)]
        pub struct TORALEV1API_CTORATstpXMdSpiFat {
            vtable: *const TORALEV1API_CTORATstpXMdSpiVTable,
            pub md_spi_ptr: *mut dyn TORALEV1API_CTORATstpXMdSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct TORALEV1API_CTORATstpXMdSpiInner {
            buf: std::collections::VecDeque<TORALEV1API_CTORATstpXMdSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl TORALEV1API_CTORATstpXMdSpiInner {
            fn push(&mut self, msg: TORALEV1API_CTORATstpXMdSpiOutput) {
                self.buf.push_back(msg);
                if let Some(ref waker) = &self.waker {
                    waker.clone().wake()
                }
            }
        }
        
        pub struct TORALEV1API_CTORATstpXMdSpiStream {
            inner: Arc<Mutex<TORALEV1API_CTORATstpXMdSpiInner>>,
        }
        
        impl Stream for TORALEV1API_CTORATstpXMdSpiStream {
            type Item = TORALEV1API_CTORATstpXMdSpiOutput;
        
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
        
        pub fn create_spi() -> (Box<TORALEV1API_CTORATstpXMdSpiStream>, *mut TORALEV1API_CTORATstpXMdSpiStream) {
            let i = TORALEV1API_CTORATstpXMdSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = TORALEV1API_CTORATstpXMdSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl TORALEV1API_CTORATstpXMdSpi_trait for TORALEV1API_CTORATstpXMdSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnFrontConnected( TORALEV1API_CTORATstpXMdSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnFrontDisconnected( TORALEV1API_CTORATstpXMdSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_rsp_get_connection_info(&mut self, p_connection_info_field : Option<&TORALEV1API_CTORATstpConnectionInfoField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspGetConnectionInfo( TORALEV1API_CTORATstpXMdSpiOnRspGetConnectionInfoPacket { p_connection_info_field:p_connection_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&TORALEV1API_CTORATstpRspUserLoginField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUserLogin( TORALEV1API_CTORATstpXMdSpiOnRspUserLoginPacket { p_rsp_user_login_field:p_rsp_user_login_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&TORALEV1API_CTORATstpUserLogoutField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUserLogout( TORALEV1API_CTORATstpXMdSpiOnRspUserLogoutPacket { p_user_logout_field:p_user_logout_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_sub_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_ph_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubPHMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubPHMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_ph_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubPHMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubPHMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_special_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSpecialMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubSpecialMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_special_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSpecialMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSpecialMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSimplifyMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubSimplifyMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSimplifyMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSimplifyMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSecurityStatus( TORALEV1API_CTORATstpXMdSpiOnRspSubSecurityStatusPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSecurityStatus( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSecurityStatusPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubMarketStatus( TORALEV1API_CTORATstpXMdSpiOnRspSubMarketStatusPacket { p_specific_market_field:p_specific_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubMarketStatus( TORALEV1API_CTORATstpXMdSpiOnRspUnSubMarketStatusPacket { p_specific_market_field:p_specific_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_imc_params(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubImcParams( TORALEV1API_CTORATstpXMdSpiOnRspSubImcParamsPacket { p_specific_market_field:p_specific_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_imc_params(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubImcParams( TORALEV1API_CTORATstpXMdSpiOnRspUnSubImcParamsPacket { p_specific_market_field:p_specific_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_inquiry_market_data_mirror(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspInquiryMarketDataMirror( TORALEV1API_CTORATstpXMdSpiOnRspInquiryMarketDataMirrorPacket { p_market_data_field:p_market_data_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_inquiry_ph_market_data_mirror(&mut self, p_ph_market_data_field : Option<&TORALEV1API_CTORATstpPHMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspInquiryPHMarketDataMirror( TORALEV1API_CTORATstpXMdSpiOnRspInquiryPHMarketDataMirrorPacket { p_ph_market_data_field:p_ph_market_data_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_inquiry_special_market_data_mirror(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpSpecialMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspInquirySpecialMarketDataMirror( TORALEV1API_CTORATstpXMdSpiOnRspInquirySpecialMarketDataMirrorPacket { p_market_data_field:p_market_data_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_sub_sp_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSPMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubSPMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_sp_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSPMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_sp_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSPSimplifyMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubSPSimplifyMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_sp_simplify_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSPSimplifyMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPSimplifyMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_sp_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSPSecurityStatus( TORALEV1API_CTORATstpXMdSpiOnRspSubSPSecurityStatusPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_sp_security_status(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSPSecurityStatus( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPSecurityStatusPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_sub_sp_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubSPMarketStatus( TORALEV1API_CTORATstpXMdSpiOnRspSubSPMarketStatusPacket { p_specific_market_field:p_specific_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_sp_market_status(&mut self, p_specific_market_field : Option<&TORALEV1API_CTORATstpSpecificMarketField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubSPMarketStatus( TORALEV1API_CTORATstpXMdSpiOnRspUnSubSPMarketStatusPacket { p_specific_market_field:p_specific_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_inquiry_sp_market_data_mirror(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspInquirySPMarketDataMirror( TORALEV1API_CTORATstpXMdSpiOnRspInquirySPMarketDataMirrorPacket { p_market_data_field:p_market_data_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rtn_market_data(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnMarketDataPacket { p_market_data_field:p_market_data_field.cloned() } ))
                }
            fn on_rtn_ph_market_data(&mut self, p_ph_market_data_field : Option<&TORALEV1API_CTORATstpPHMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnPHMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnPHMarketDataPacket { p_ph_market_data_field:p_ph_market_data_field.cloned() } ))
                }
            fn on_rtn_special_market_data(&mut self, p_special_market_data_field : Option<&TORALEV1API_CTORATstpSpecialMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSpecialMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnSpecialMarketDataPacket { p_special_market_data_field:p_special_market_data_field.cloned() } ))
                }
            fn on_rtn_simplify_market_data(&mut self, p_simplify_market_data_field : Option<&TORALEV1API_CTORATstpSimplifyMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSimplifyMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnSimplifyMarketDataPacket { p_simplify_market_data_field:p_simplify_market_data_field.cloned() } ))
                }
            fn on_rtn_security_status(&mut self, p_security_status_field : Option<&TORALEV1API_CTORATstpSecurityStatusField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSecurityStatus( TORALEV1API_CTORATstpXMdSpiOnRtnSecurityStatusPacket { p_security_status_field:p_security_status_field.cloned() } ))
                }
            fn on_rtn_market_status(&mut self, p_market_status_field : Option<&TORALEV1API_CTORATstpMarketStatusField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnMarketStatus( TORALEV1API_CTORATstpXMdSpiOnRtnMarketStatusPacket { p_market_status_field:p_market_status_field.cloned() } ))
                }
            fn on_rtn_imc_params(&mut self, p_imc_params_field : Option<&TORALEV1API_CTORATstpImcParamsField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnImcParams( TORALEV1API_CTORATstpXMdSpiOnRtnImcParamsPacket { p_imc_params_field:p_imc_params_field.cloned() } ))
                }
            fn on_rtn_sp_market_data(&mut self, p_market_data_field : Option<&TORALEV1API_CTORATstpMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSPMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnSPMarketDataPacket { p_market_data_field:p_market_data_field.cloned() } ))
                }
            fn on_rtn_sp_simplify_market_data(&mut self, p_simplify_market_data_field : Option<&TORALEV1API_CTORATstpSimplifyMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSPSimplifyMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnSPSimplifyMarketDataPacket { p_simplify_market_data_field:p_simplify_market_data_field.cloned() } ))
                }
            fn on_rtn_sp_security_status(&mut self, p_security_status_field : Option<&TORALEV1API_CTORATstpSecurityStatusField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSPSecurityStatus( TORALEV1API_CTORATstpXMdSpiOnRtnSPSecurityStatusPacket { p_security_status_field:p_security_status_field.cloned() } ))
                }
            fn on_rtn_sp_market_status(&mut self, p_market_status_field : Option<&TORALEV1API_CTORATstpMarketStatusField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnSPMarketStatus( TORALEV1API_CTORATstpXMdSpiOnRtnSPMarketStatusPacket { p_market_status_field:p_market_status_field.cloned() } ))
                }
            fn on_rsp_sub_rapid_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubRapidMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubRapidMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_rapid_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubRapidMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubRapidMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rtn_rapid_market_data(&mut self, p_rapid_market_data_field : Option<&TORALEV1API_CTORATstpRapidMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnRapidMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnRapidMarketDataPacket { p_rapid_market_data_field:p_rapid_market_data_field.cloned() } ))
                }
            fn on_rsp_sub_funds_flow_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspSubFundsFlowMarketData( TORALEV1API_CTORATstpXMdSpiOnRspSubFundsFlowMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rsp_un_sub_funds_flow_market_data(&mut self, p_specific_security_field : Option<&TORALEV1API_CTORATstpSpecificSecurityField>,p_rsp_info_field : Option<&TORALEV1API_CTORATstpRspInfoField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRspUnSubFundsFlowMarketData( TORALEV1API_CTORATstpXMdSpiOnRspUnSubFundsFlowMarketDataPacket { p_specific_security_field:p_specific_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned() } ))
                }
            fn on_rtn_funds_flow_market_data(&mut self, p_funds_flow_market_data_field : Option<&TORALEV1API_CTORATstpFundsFlowMarketDataField>) 
 {
            self.inner.lock().unwrap().push(TORALEV1API_CTORATstpXMdSpiOutput::OnRtnFundsFlowMarketData( TORALEV1API_CTORATstpXMdSpiOnRtnFundsFlowMarketDataPacket { p_funds_flow_market_data_field:p_funds_flow_market_data_field.cloned() } ))
                }
             }

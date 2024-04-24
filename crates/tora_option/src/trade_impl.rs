impl TORASPAPI_CTORATstpSPTraderApi {
                            pub fn release(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_Release)(self as *mut TORASPAPI_CTORATstpSPTraderApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_Init)(self as *mut TORASPAPI_CTORATstpSPTraderApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_Join)(self as *mut TORASPAPI_CTORATstpSPTraderApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: &str) -> () {
                                    let psz_front_address = CString::new(psz_front_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_RegisterFront)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             psz_front_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: &str) -> () {
                                    let psz_ns_address = CString::new(psz_ns_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_RegisterNameServer)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             psz_ns_address.as_ptr() as *mut c_char)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn TORASPAPI_CTORATstpSPTraderSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &TORASPAPI_CTORA_TSTP_SP_TRADER_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_RegisterSpi)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_spi as * mut TORASPAPI_CTORATstpSPTraderSpi)
                                            }
                                }
                            pub fn subscribe_private_topic(&mut self, n_resume_type: TORASPAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_SubscribePrivateTopic)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn subscribe_public_topic(&mut self, n_resume_type: TORASPAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_SubscribePublicTopic)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut TORASPAPI_CTORATstpSPReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqUserLogin)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_user_login_field as * mut TORASPAPI_CTORATstpSPReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout_field: &mut TORASPAPI_CTORATstpSPUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqUserLogout)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_user_logout_field as * mut TORASPAPI_CTORATstpSPUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_password_update(&mut self, p_user_password_update_field: &mut TORASPAPI_CTORATstpSPUserPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqUserPasswordUpdate)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_user_password_update_field as * mut TORASPAPI_CTORATstpSPUserPasswordUpdateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_input_device_serial(&mut self, p_req_input_device_serial_field: &mut TORASPAPI_CTORATstpSPReqInputDeviceSerialField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInputDeviceSerial)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_input_device_serial_field as * mut TORASPAPI_CTORATstpSPReqInputDeviceSerialField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_insert(&mut self, p_input_order_field: &mut TORASPAPI_CTORATstpSPInputOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqOrderInsert)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_order_field as * mut TORASPAPI_CTORATstpSPInputOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_action(&mut self, p_input_order_action_field: &mut TORASPAPI_CTORATstpSPInputOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqOrderAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_order_action_field as * mut TORASPAPI_CTORATstpSPInputOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_exercise_insert(&mut self, p_input_exercise_field: &mut TORASPAPI_CTORATstpSPInputExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqExerciseInsert)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_exercise_field as * mut TORASPAPI_CTORATstpSPInputExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_exercise_action(&mut self, p_input_exercise_action_field: &mut TORASPAPI_CTORATstpSPInputExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqExerciseAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_exercise_action_field as * mut TORASPAPI_CTORATstpSPInputExerciseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_lock_insert(&mut self, p_input_lock_field: &mut TORASPAPI_CTORATstpSPInputLockField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqLockInsert)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_lock_field as * mut TORASPAPI_CTORATstpSPInputLockField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_lock_action(&mut self, p_input_lock_action_field: &mut TORASPAPI_CTORATstpSPInputLockActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqLockAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_lock_action_field as * mut TORASPAPI_CTORATstpSPInputLockActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_order_insert(&mut self, p_input_comb_order_field: &mut TORASPAPI_CTORATstpSPInputCombOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqCombOrderInsert)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_order_field as * mut TORASPAPI_CTORATstpSPInputCombOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_order_action(&mut self, p_input_comb_order_action_field: &mut TORASPAPI_CTORATstpSPInputCombOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqCombOrderAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_order_action_field as * mut TORASPAPI_CTORATstpSPInputCombOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_insert(&mut self, p_input_cond_order_field: &mut TORASPAPI_CTORATstpSPInputCondOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqCondOrderInsert)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_cond_order_field as * mut TORASPAPI_CTORATstpSPInputCondOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_action(&mut self, p_input_cond_order_action_field: &mut TORASPAPI_CTORATstpSPInputCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqCondOrderAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_cond_order_action_field as * mut TORASPAPI_CTORATstpSPInputCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_exercise_insert(&mut self, p_input_comb_exercise_field: &mut TORASPAPI_CTORATstpSPInputCombExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqCombExerciseInsert)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_exercise_field as * mut TORASPAPI_CTORATstpSPInputCombExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_exercise_action(&mut self, p_input_comb_exercise_action_field: &mut TORASPAPI_CTORATstpSPInputCombExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqCombExerciseAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_exercise_action_field as * mut TORASPAPI_CTORATstpSPInputCombExerciseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_max_lock_volume(&mut self, p_req_inquiry_max_lock_volume_field: &mut TORASPAPI_CTORATstpSPReqInquiryMaxLockVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInquiryMaxLockVolume)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_max_lock_volume_field as * mut TORASPAPI_CTORATstpSPReqInquiryMaxLockVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_max_cover_volume(&mut self, p_req_inquiry_max_cover_volume_field: &mut TORASPAPI_CTORATstpSPReqInquiryMaxCoverVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInquiryMaxCoverVolume)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_max_cover_volume_field as * mut TORASPAPI_CTORATstpSPReqInquiryMaxCoverVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_split_comb_margin_difference(&mut self, p_req_inquiry_split_comb_margin_difference_field: &mut TORASPAPI_CTORATstpSPReqInquirySplitCombMarginDifferenceField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInquirySplitCombMarginDifference)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_split_comb_margin_difference_field as * mut TORASPAPI_CTORATstpSPReqInquirySplitCombMarginDifferenceField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_fund(&mut self, p_input_transfer_fund_field: &mut TORASPAPI_CTORATstpSPInputTransferFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqTransferFund)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_transfer_fund_field as * mut TORASPAPI_CTORATstpSPInputTransferFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_stock_position(&mut self, p_input_transfer_stock_position_field: &mut TORASPAPI_CTORATstpSPInputTransferStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqTransferStockPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_transfer_stock_position_field as * mut TORASPAPI_CTORATstpSPInputTransferStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_jz_fund(&mut self, p_req_inquiry_jz_fund_field: &mut TORASPAPI_CTORATstpSPReqInquiryJZFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInquiryJZFund)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_jz_fund_field as * mut TORASPAPI_CTORATstpSPReqInquiryJZFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_bank_account_fund(&mut self, p_req_inquiry_bank_account_fund_field: &mut TORASPAPI_CTORATstpSPReqInquiryBankAccountFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInquiryBankAccountFund)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_bank_account_fund_field as * mut TORASPAPI_CTORATstpSPReqInquiryBankAccountFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_stock_position(&mut self, p_req_inquiry_stock_position_field: &mut TORASPAPI_CTORATstpSPReqInquiryStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqInquiryStockPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_stock_position_field as * mut TORASPAPI_CTORATstpSPReqInquiryStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange(&mut self, p_qry_exchange_field: &mut TORASPAPI_CTORATstpSPQryExchangeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryExchange)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exchange_field as * mut TORASPAPI_CTORATstpSPQryExchangeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_market_data(&mut self, p_qry_market_data_field: &mut TORASPAPI_CTORATstpSPQryMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryMarketData)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_market_data_field as * mut TORASPAPI_CTORATstpSPQryMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_security(&mut self, p_qry_security_field: &mut TORASPAPI_CTORATstpSPQrySecurityField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQrySecurity)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_security_field as * mut TORASPAPI_CTORATstpSPQrySecurityField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_bu_proxy(&mut self, p_qry_bu_proxy_field: &mut TORASPAPI_CTORATstpSPQryBUProxyField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryBUProxy)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_bu_proxy_field as * mut TORASPAPI_CTORATstpSPQryBUProxyField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_user(&mut self, p_qry_user_field: &mut TORASPAPI_CTORATstpSPQryUserField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryUser)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_user_field as * mut TORASPAPI_CTORATstpSPQryUserField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor(&mut self, p_qry_investor_field: &mut TORASPAPI_CTORATstpSPQryInvestorField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestor)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_field as * mut TORASPAPI_CTORATstpSPQryInvestorField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_shareholder_account(&mut self, p_qry_shareholder_account_field: &mut TORASPAPI_CTORATstpSPQryShareholderAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryShareholderAccount)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_shareholder_account_field as * mut TORASPAPI_CTORATstpSPQryShareholderAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_account(&mut self, p_qry_trading_account_field: &mut TORASPAPI_CTORATstpSPQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryTradingAccount)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trading_account_field as * mut TORASPAPI_CTORATstpSPQryTradingAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order(&mut self, p_qry_order_field: &mut TORASPAPI_CTORATstpSPQryOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryOrder)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_order_field as * mut TORASPAPI_CTORATstpSPQryOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trade(&mut self, p_qry_trade_field: &mut TORASPAPI_CTORATstpSPQryTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryTrade)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trade_field as * mut TORASPAPI_CTORATstpSPQryTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position(&mut self, p_qry_position_field: &mut TORASPAPI_CTORATstpSPQryPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_position_field as * mut TORASPAPI_CTORATstpSPQryPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_fee(&mut self, p_qry_trading_fee_field: &mut TORASPAPI_CTORATstpSPQryTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryTradingFee)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trading_fee_field as * mut TORASPAPI_CTORATstpSPQryTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_trading_fee(&mut self, p_qry_investor_trading_fee_field: &mut TORASPAPI_CTORATstpSPQryInvestorTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorTradingFee)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_trading_fee_field as * mut TORASPAPI_CTORATstpSPQryInvestorTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_margin_fee(&mut self, p_qry_investor_margin_fee_field: &mut TORASPAPI_CTORATstpSPQryInvestorMarginFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorMarginFee)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_margin_fee_field as * mut TORASPAPI_CTORATstpSPQryInvestorMarginFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_fund_detail(&mut self, p_qry_order_fund_detail_field: &mut TORASPAPI_CTORATstpSPQryOrderFundDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryOrderFundDetail)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_order_fund_detail_field as * mut TORASPAPI_CTORATstpSPQryOrderFundDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_fund_transfer_detail(&mut self, p_qry_fund_transfer_detail_field: &mut TORASPAPI_CTORATstpSPQryFundTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryFundTransferDetail)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_fund_transfer_detail_field as * mut TORASPAPI_CTORATstpSPQryFundTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position_transfer_detail(&mut self, p_qry_position_transfer_detail_field: &mut TORASPAPI_CTORATstpSPQryPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryPositionTransferDetail)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_position_transfer_detail_field as * mut TORASPAPI_CTORATstpSPQryPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_action(&mut self, p_qry_order_action_field: &mut TORASPAPI_CTORATstpSPQryOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryOrderAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_order_action_field as * mut TORASPAPI_CTORATstpSPQryOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_position(&mut self, p_qry_stock_position_field: &mut TORASPAPI_CTORATstpSPQryStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryStockPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_position_field as * mut TORASPAPI_CTORATstpSPQryStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lock(&mut self, p_qry_lock_field: &mut TORASPAPI_CTORATstpSPQryLockField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryLock)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_lock_field as * mut TORASPAPI_CTORATstpSPQryLockField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exercise(&mut self, p_qry_exercise_field: &mut TORASPAPI_CTORATstpSPQryExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryExercise)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exercise_field as * mut TORASPAPI_CTORATstpSPQryExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lock_position(&mut self, p_qry_lock_position_field: &mut TORASPAPI_CTORATstpSPQryLockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryLockPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_lock_position_field as * mut TORASPAPI_CTORATstpSPQryLockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exercise_action(&mut self, p_qry_exercise_action_field: &mut TORASPAPI_CTORATstpSPQryExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryExerciseAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exercise_action_field as * mut TORASPAPI_CTORATstpSPQryExerciseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lock_action(&mut self, p_qry_lock_action_field: &mut TORASPAPI_CTORATstpSPQryLockActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryLockAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_lock_action_field as * mut TORASPAPI_CTORATstpSPQryLockActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_position_transfer_detail(&mut self, p_qry_stock_position_transfer_detail_field: &mut TORASPAPI_CTORATstpSPQryStockPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryStockPositionTransferDetail)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_position_transfer_detail_field as * mut TORASPAPI_CTORATstpSPQryStockPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_notice(&mut self, p_qry_trading_notice_field: &mut TORASPAPI_CTORATstpSPQryTradingNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryTradingNotice)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trading_notice_field as * mut TORASPAPI_CTORATstpSPQryTradingNoticeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_disposal(&mut self, p_qry_stock_disposal_field: &mut TORASPAPI_CTORATstpSPQryStockDisposalField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryStockDisposal)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_disposal_field as * mut TORASPAPI_CTORATstpSPQryStockDisposalField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_disposal_action(&mut self, p_qry_stock_disposal_action_field: &mut TORASPAPI_CTORATstpSPQryStockDisposalActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryStockDisposalAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_disposal_action_field as * mut TORASPAPI_CTORATstpSPQryStockDisposalActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cond_order(&mut self, p_qry_cond_order_field: &mut TORASPAPI_CTORATstpSPQryCondOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCondOrder)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_cond_order_field as * mut TORASPAPI_CTORATstpSPQryCondOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cond_order_action(&mut self, p_qry_cond_order_action_field: &mut TORASPAPI_CTORATstpSPQryCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCondOrderAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_cond_order_action_field as * mut TORASPAPI_CTORATstpSPQryCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_limit_position(&mut self, p_qry_investor_limit_position_field: &mut TORASPAPI_CTORATstpSPQryInvestorLimitPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorLimitPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_limit_position_field as * mut TORASPAPI_CTORATstpSPQryInvestorLimitPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_limit_amount(&mut self, p_qry_investor_limit_amount_field: &mut TORASPAPI_CTORATstpSPQryInvestorLimitAmountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorLimitAmount)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_limit_amount_field as * mut TORASPAPI_CTORATstpSPQryInvestorLimitAmountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_order_action(&mut self, p_qry_comb_order_action_field: &mut TORASPAPI_CTORATstpSPQryCombOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombOrderAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_order_action_field as * mut TORASPAPI_CTORATstpSPQryCombOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_order(&mut self, p_qry_comb_order_field: &mut TORASPAPI_CTORATstpSPQryCombOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombOrder)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_order_field as * mut TORASPAPI_CTORATstpSPQryCombOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_position(&mut self, p_qry_comb_position_field: &mut TORASPAPI_CTORATstpSPQryCombPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_position_field as * mut TORASPAPI_CTORATstpSPQryCombPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_pos_detail(&mut self, p_qry_comb_pos_detail_field: &mut TORASPAPI_CTORATstpSPQryCombPosDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombPosDetail)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_pos_detail_field as * mut TORASPAPI_CTORATstpSPQryCombPosDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exercise_appointment(&mut self, p_qry_exercise_appointment_field: &mut TORASPAPI_CTORATstpSPQryExerciseAppointmentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryExerciseAppointment)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exercise_appointment_field as * mut TORASPAPI_CTORATstpSPQryExerciseAppointmentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_insufficient_covered_stock_position(&mut self, p_qry_insufficient_covered_stock_position_field: &mut TORASPAPI_CTORATstpSPQryInsufficientCoveredStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryInsufficientCoveredStockPosition)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_insufficient_covered_stock_position_field as * mut TORASPAPI_CTORATstpSPQryInsufficientCoveredStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_security(&mut self, p_qry_comb_security_field: &mut TORASPAPI_CTORATstpSPQryCombSecurityField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombSecurity)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_security_field as * mut TORASPAPI_CTORATstpSPQryCombSecurityField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_exercise(&mut self, p_qry_comb_exercise_field: &mut TORASPAPI_CTORATstpSPQryCombExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombExercise)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_exercise_field as * mut TORASPAPI_CTORATstpSPQryCombExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_exercise_action(&mut self, p_qry_comb_exercise_action_field: &mut TORASPAPI_CTORATstpSPQryCombExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASPAPI_CTORATstpSPTraderApi_ReqQryCombExerciseAction)(self as *mut TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_exercise_action_field as * mut TORASPAPI_CTORATstpSPQryCombExerciseActionField,
                                             n_request_id)
                                        }
                            }} 
                unsafe impl Send for TORASPAPI_CTORATstpSPTraderApi {}pub trait TORASPAPI_CTORATstpSPTraderSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_rsp_error(&mut self, p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&TORASPAPI_CTORATstpSPRspUserLoginField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&TORASPAPI_CTORATstpSPUserLogoutField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&TORASPAPI_CTORATstpSPUserPasswordUpdateField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&TORASPAPI_CTORATstpSPRspInputDeviceSerialField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_order(&mut self, p_order : Option<&TORASPAPI_CTORATstpSPOrderField>) {}
fn on_err_rtn_order_insert(&mut self, p_input_order : Option<&TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_order_action(&mut self, p_input_order_action : Option<&TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_trade(&mut self, p_trade : Option<&TORASPAPI_CTORATstpSPTradeField>) {}
fn on_rsp_exercise_insert(&mut self, p_input_exercise_field : Option<&TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_exercise(&mut self, p_exercise : Option<&TORASPAPI_CTORATstpSPExerciseField>) {}
fn on_err_rtn_exercise_insert(&mut self, p_input_exercise : Option<&TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_exercise_action(&mut self, p_input_exercise_action_field : Option<&TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_exercise_action(&mut self, p_input_exercise_action : Option<&TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_lock_insert(&mut self, p_input_lock_field : Option<&TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_lock(&mut self, p_lock : Option<&TORASPAPI_CTORATstpSPLockField>) {}
fn on_err_rtn_lock_insert(&mut self, p_input_lock : Option<&TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_lock_action(&mut self, p_input_lock_action_field : Option<&TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_lock_action(&mut self, p_input_lock_action : Option<&TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_stock_disposal(&mut self, p_stock_disposal : Option<&TORASPAPI_CTORATstpSPStockDisposalField>) {}
fn on_rsp_comb_order_insert(&mut self, p_input_comb_order_field : Option<&TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_comb_order(&mut self, p_comb_order : Option<&TORASPAPI_CTORATstpSPCombOrderField>) {}
fn on_err_rtn_comb_order_insert(&mut self, p_input_comb_order : Option<&TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_comb_order_action(&mut self, p_input_comb_order_action_field : Option<&TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_comb_order_action(&mut self, p_input_comb_order_action : Option<&TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_cond_order(&mut self, p_condition_order : Option<&TORASPAPI_CTORATstpSPConditionOrderField>) {}
fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order : Option<&TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action : Option<&TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_comb_exercise_insert(&mut self, p_input_comb_exercise_field : Option<&TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_comb_exercise(&mut self, p_comb_exercise : Option<&TORASPAPI_CTORATstpSPCombExerciseField>) {}
fn on_err_rtn_comb_exercise_insert(&mut self, p_input_comb_exercise : Option<&TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_comb_exercise_action(&mut self, p_input_comb_exercise_action_field : Option<&TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_comb_exercise_action(&mut self, p_input_comb_exercise_action : Option<&TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_max_lock_volume(&mut self, p_rsp_inquiry_max_lock_volume_field : Option<&TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_max_cover_volume(&mut self, p_rsp_inquiry_max_cover_volume_field : Option<&TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_split_comb_margin_difference(&mut self, p_rsp_inquiry_split_comb_margin_difference_field : Option<&TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_fund(&mut self, p_transfer_fund : Option<&TORASPAPI_CTORATstpSPTransferFundField>) {}
fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund : Option<&TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_position(&mut self, p_transfer_position : Option<&TORASPAPI_CTORATstpSPTransferPositionField>) {}
fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position : Option<&TORASPAPI_CTORATstpSPInputTransferPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_transfer_stock_position(&mut self, p_input_transfer_stock_position_field : Option<&TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_stock_position(&mut self, p_transfer_stock_position : Option<&TORASPAPI_CTORATstpSPTransferStockPositionField>) {}
fn on_err_rtn_transfer_stock_position(&mut self, p_input_transfer_stock_position : Option<&TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&TORASPAPI_CTORATstpSPRspInquiryJZFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_stock_position(&mut self, p_rsp_inquiry_stock_position_field : Option<&TORASPAPI_CTORATstpSPRspInquiryStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_market_status(&mut self, p_market_status : Option<&TORASPAPI_CTORATstpSPMarketStatusField>) {}
fn on_rtn_trading_notice(&mut self, p_trading_notice : Option<&TORASPAPI_CTORATstpSPTradingNoticeField>) {}
fn on_rsp_qry_exchange(&mut self, p_exchange : Option<&TORASPAPI_CTORATstpSPExchangeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_market_data(&mut self, p_market_data : Option<&TORASPAPI_CTORATstpSPMarketDataField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_security(&mut self, p_security : Option<&TORASPAPI_CTORATstpSPSecurityField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_bu_proxy(&mut self, p_bu_proxy : Option<&TORASPAPI_CTORATstpSPBUProxyField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_user(&mut self, p_user : Option<&TORASPAPI_CTORATstpSPUserField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor(&mut self, p_investor : Option<&TORASPAPI_CTORATstpSPInvestorField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account : Option<&TORASPAPI_CTORATstpSPShareholderAccountField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_account(&mut self, p_trading_account : Option<&TORASPAPI_CTORATstpSPTradingAccountField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order(&mut self, p_order : Option<&TORASPAPI_CTORATstpSPOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trade(&mut self, p_trade : Option<&TORASPAPI_CTORATstpSPTradeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position(&mut self, p_position : Option<&TORASPAPI_CTORATstpSPPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_fee(&mut self, p_trading_fee : Option<&TORASPAPI_CTORATstpSPTradingFeeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee : Option<&TORASPAPI_CTORATstpSPInvestorTradingFeeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_margin_fee(&mut self, p_investor_margin_fee : Option<&TORASPAPI_CTORATstpSPInvestorMarginFeeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail : Option<&TORASPAPI_CTORATstpSPOrderFundDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail : Option<&TORASPAPI_CTORATstpSPFundTransferDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail : Option<&TORASPAPI_CTORATstpSPPositionTransferDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_action(&mut self, p_order_action : Option<&TORASPAPI_CTORATstpSPOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_position(&mut self, p_stock_position : Option<&TORASPAPI_CTORATstpSPStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lock(&mut self, p_lock : Option<&TORASPAPI_CTORATstpSPLockField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exercise(&mut self, p_exercise : Option<&TORASPAPI_CTORATstpSPExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lock_position(&mut self, p_lock_position : Option<&TORASPAPI_CTORATstpSPLockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exercise_action(&mut self, p_exercise_action : Option<&TORASPAPI_CTORATstpSPExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lock_action(&mut self, p_lock_action : Option<&TORASPAPI_CTORATstpSPLockActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_position_transfer_detail(&mut self, p_stock_position_transfer_detail : Option<&TORASPAPI_CTORATstpSPStockPositionTransferDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_notice(&mut self, p_trading_notice : Option<&TORASPAPI_CTORATstpSPTradingNoticeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_disposal(&mut self, p_stock_disposal : Option<&TORASPAPI_CTORATstpSPStockDisposalField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_disposal_action(&mut self, p_stock_disposal_action : Option<&TORASPAPI_CTORATstpSPStockDisposalActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cond_order(&mut self, p_cond_order : Option<&TORASPAPI_CTORATstpSPCondOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action : Option<&TORASPAPI_CTORATstpSPCondOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_limit_position(&mut self, p_investor_limit_position : Option<&TORASPAPI_CTORATstpSPInvestorLimitPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_limit_amount(&mut self, p_investor_limit_amount : Option<&TORASPAPI_CTORATstpSPInvestorLimitAmountField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_order_action(&mut self, p_comb_order_action : Option<&TORASPAPI_CTORATstpSPCombOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_order(&mut self, p_comb_order : Option<&TORASPAPI_CTORATstpSPCombOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_position(&mut self, p_comb_position : Option<&TORASPAPI_CTORATstpSPCombPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_pos_detail(&mut self, p_comb_pos_detail : Option<&TORASPAPI_CTORATstpSPCombPosDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exercise_appointment(&mut self, p_exercise_appointment : Option<&TORASPAPI_CTORATstpSPExerciseAppointmentField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_insufficient_covered_stock_position(&mut self, p_insufficient_covered_stock_position : Option<&TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_security(&mut self, p_comb_security : Option<&TORASPAPI_CTORATstpSPCombSecurityField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_exercise(&mut self, p_comb_exercise : Option<&TORASPAPI_CTORATstpSPCombExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_exercise_action(&mut self, p_comb_exercise_action : Option<&TORASPAPI_CTORATstpSPCombExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct TORASPAPI_CTORATstpSPTraderSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, n_reason : std::os::raw::c_int ),
                on_rsp_error: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_login: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_user_login_field : * const TORASPAPI_CTORATstpSPRspUserLoginField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_logout: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_user_logout_field : * const TORASPAPI_CTORATstpSPUserLogoutField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_password_update: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_user_password_update_field : * const TORASPAPI_CTORATstpSPUserPasswordUpdateField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_input_device_serial: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_input_device_serial_field : * const TORASPAPI_CTORATstpSPRspInputDeviceSerialField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_order_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_field : * const TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_order: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const TORASPAPI_CTORATstpSPOrderField ),
                on_err_rtn_order_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order : * const TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action_field : * const TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action : * const TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_trade: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const TORASPAPI_CTORATstpSPTradeField ),
                on_rsp_exercise_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_field : * const TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_exercise: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const TORASPAPI_CTORATstpSPExerciseField ),
                on_err_rtn_exercise_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise : * const TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_exercise_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action_field : * const TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_exercise_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action : * const TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_lock_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_field : * const TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_lock: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const TORASPAPI_CTORATstpSPLockField ),
                on_err_rtn_lock_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock : * const TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_lock_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action_field : * const TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_lock_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action : * const TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_stock_disposal: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const TORASPAPI_CTORATstpSPStockDisposalField ),
                on_rsp_comb_order_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_field : * const TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_comb_order: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const TORASPAPI_CTORATstpSPCombOrderField ),
                on_err_rtn_comb_order_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order : * const TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_comb_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action_field : * const TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_comb_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action : * const TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_field : * const TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_cond_order: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_condition_order : * const TORASPAPI_CTORATstpSPConditionOrderField ),
                on_err_rtn_cond_order_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order : * const TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action_field : * const TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_cond_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action : * const TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_comb_exercise_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_field : * const TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_comb_exercise: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const TORASPAPI_CTORATstpSPCombExerciseField ),
                on_err_rtn_comb_exercise_insert: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise : * const TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_comb_exercise_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action_field : * const TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_comb_exercise_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action : * const TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_max_lock_volume: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_lock_volume_field : * const TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_max_cover_volume: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_cover_volume_field : * const TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_split_comb_margin_difference: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_split_comb_margin_difference_field : * const TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_transfer_fund: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund_field : * const TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_fund: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_fund : * const TORASPAPI_CTORATstpSPTransferFundField ),
                on_err_rtn_transfer_fund: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund : * const TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_position : * const TORASPAPI_CTORATstpSPTransferPositionField ),
                on_err_rtn_transfer_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_position : * const TORASPAPI_CTORATstpSPInputTransferPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_transfer_stock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position_field : * const TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_stock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_stock_position : * const TORASPAPI_CTORATstpSPTransferStockPositionField ),
                on_err_rtn_transfer_stock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position : * const TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_jz_fund: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const TORASPAPI_CTORATstpSPRspInquiryJZFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_bank_account_fund: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_stock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_stock_position_field : * const TORASPAPI_CTORATstpSPRspInquiryStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_market_status: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_market_status : * const TORASPAPI_CTORATstpSPMarketStatusField ),
                on_rtn_trading_notice: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const TORASPAPI_CTORATstpSPTradingNoticeField ),
                on_rsp_qry_exchange: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exchange : * const TORASPAPI_CTORATstpSPExchangeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_market_data: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_market_data : * const TORASPAPI_CTORATstpSPMarketDataField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_security: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_security : * const TORASPAPI_CTORATstpSPSecurityField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_bu_proxy: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_bu_proxy : * const TORASPAPI_CTORATstpSPBUProxyField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_user: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_user : * const TORASPAPI_CTORATstpSPUserField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor : * const TORASPAPI_CTORATstpSPInvestorField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_shareholder_account: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_shareholder_account : * const TORASPAPI_CTORATstpSPShareholderAccountField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_account: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_account : * const TORASPAPI_CTORATstpSPTradingAccountField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const TORASPAPI_CTORATstpSPOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trade: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const TORASPAPI_CTORATstpSPTradeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_position : * const TORASPAPI_CTORATstpSPPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_fee: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_fee : * const TORASPAPI_CTORATstpSPTradingFeeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_trading_fee: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_trading_fee : * const TORASPAPI_CTORATstpSPInvestorTradingFeeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_margin_fee: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_margin_fee : * const TORASPAPI_CTORATstpSPInvestorMarginFeeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_fund_detail: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order_fund_detail : * const TORASPAPI_CTORATstpSPOrderFundDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_fund_transfer_detail: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_fund_transfer_detail : * const TORASPAPI_CTORATstpSPFundTransferDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position_transfer_detail: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_position_transfer_detail : * const TORASPAPI_CTORATstpSPPositionTransferDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order_action : * const TORASPAPI_CTORATstpSPOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position : * const TORASPAPI_CTORATstpSPStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lock: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const TORASPAPI_CTORATstpSPLockField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exercise: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const TORASPAPI_CTORATstpSPExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_position : * const TORASPAPI_CTORATstpSPLockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exercise_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_action : * const TORASPAPI_CTORATstpSPExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lock_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_action : * const TORASPAPI_CTORATstpSPLockActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_position_transfer_detail: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position_transfer_detail : * const TORASPAPI_CTORATstpSPStockPositionTransferDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_notice: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const TORASPAPI_CTORATstpSPTradingNoticeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_disposal: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const TORASPAPI_CTORATstpSPStockDisposalField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_disposal_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal_action : * const TORASPAPI_CTORATstpSPStockDisposalActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cond_order: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order : * const TORASPAPI_CTORATstpSPCondOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cond_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order_action : * const TORASPAPI_CTORATstpSPCondOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_limit_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_position : * const TORASPAPI_CTORATstpSPInvestorLimitPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_limit_amount: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_amount : * const TORASPAPI_CTORATstpSPInvestorLimitAmountField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_order_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order_action : * const TORASPAPI_CTORATstpSPCombOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_order: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const TORASPAPI_CTORATstpSPCombOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_position : * const TORASPAPI_CTORATstpSPCombPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_pos_detail: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_pos_detail : * const TORASPAPI_CTORATstpSPCombPosDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exercise_appointment: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_appointment : * const TORASPAPI_CTORATstpSPExerciseAppointmentField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_insufficient_covered_stock_position: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_insufficient_covered_stock_position : * const TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_security: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_security : * const TORASPAPI_CTORATstpSPCombSecurityField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_exercise: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const TORASPAPI_CTORATstpSPCombExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_exercise_action: extern "C" fn(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise_action : * const TORASPAPI_CTORATstpSPCombExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                 } 

        #[derive(Clone, Debug)]
        pub enum TORASPAPI_CTORATstpSPTraderSpiOutput {OnFrontConnected(TORASPAPI_CTORATstpSPTraderSpiOnFrontConnectedPacket),OnFrontDisconnected(TORASPAPI_CTORATstpSPTraderSpiOnFrontDisconnectedPacket),OnRspError(TORASPAPI_CTORATstpSPTraderSpiOnRspErrorPacket),OnRspUserLogin(TORASPAPI_CTORATstpSPTraderSpiOnRspUserLoginPacket),OnRspUserLogout(TORASPAPI_CTORATstpSPTraderSpiOnRspUserLogoutPacket),OnRspUserPasswordUpdate(TORASPAPI_CTORATstpSPTraderSpiOnRspUserPasswordUpdatePacket),OnRspInputDeviceSerial(TORASPAPI_CTORATstpSPTraderSpiOnRspInputDeviceSerialPacket),OnRspOrderInsert(TORASPAPI_CTORATstpSPTraderSpiOnRspOrderInsertPacket),OnRtnOrder(TORASPAPI_CTORATstpSPTraderSpiOnRtnOrderPacket),OnErrRtnOrderInsert(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderInsertPacket),OnRspOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnRspOrderActionPacket),OnErrRtnOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderActionPacket),OnRtnTrade(TORASPAPI_CTORATstpSPTraderSpiOnRtnTradePacket),OnRspExerciseInsert(TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseInsertPacket),OnRtnExercise(TORASPAPI_CTORATstpSPTraderSpiOnRtnExercisePacket),OnErrRtnExerciseInsert(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseInsertPacket),OnRspExerciseAction(TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseActionPacket),OnErrRtnExerciseAction(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseActionPacket),OnRspLockInsert(TORASPAPI_CTORATstpSPTraderSpiOnRspLockInsertPacket),OnRtnLock(TORASPAPI_CTORATstpSPTraderSpiOnRtnLockPacket),OnErrRtnLockInsert(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockInsertPacket),OnRspLockAction(TORASPAPI_CTORATstpSPTraderSpiOnRspLockActionPacket),OnErrRtnLockAction(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockActionPacket),OnRtnStockDisposal(TORASPAPI_CTORATstpSPTraderSpiOnRtnStockDisposalPacket),OnRspCombOrderInsert(TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderInsertPacket),OnRtnCombOrder(TORASPAPI_CTORATstpSPTraderSpiOnRtnCombOrderPacket),OnErrRtnCombOrderInsert(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderInsertPacket),OnRspCombOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderActionPacket),OnErrRtnCombOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderActionPacket),OnRspCondOrderInsert(TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderInsertPacket),OnRtnCondOrder(TORASPAPI_CTORATstpSPTraderSpiOnRtnCondOrderPacket),OnErrRtnCondOrderInsert(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderInsertPacket),OnRspCondOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderActionPacket),OnErrRtnCondOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderActionPacket),OnRspCombExerciseInsert(TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseInsertPacket),OnRtnCombExercise(TORASPAPI_CTORATstpSPTraderSpiOnRtnCombExercisePacket),OnErrRtnCombExerciseInsert(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseInsertPacket),OnRspCombExerciseAction(TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseActionPacket),OnErrRtnCombExerciseAction(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseActionPacket),OnRspInquiryMaxLockVolume(TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxLockVolumePacket),OnRspInquiryMaxCoverVolume(TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxCoverVolumePacket),OnRspInquirySplitCombMarginDifference(TORASPAPI_CTORATstpSPTraderSpiOnRspInquirySplitCombMarginDifferencePacket),OnRspTransferFund(TORASPAPI_CTORATstpSPTraderSpiOnRspTransferFundPacket),OnRtnTransferFund(TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferFundPacket),OnErrRtnTransferFund(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferFundPacket),OnRtnTransferPosition(TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferPositionPacket),OnErrRtnTransferPosition(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferPositionPacket),OnRspTransferStockPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspTransferStockPositionPacket),OnRtnTransferStockPosition(TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferStockPositionPacket),OnErrRtnTransferStockPosition(TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferStockPositionPacket),OnRspInquiryJZFund(TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryJZFundPacket),OnRspInquiryBankAccountFund(TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryBankAccountFundPacket),OnRspInquiryStockPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryStockPositionPacket),OnRtnMarketStatus(TORASPAPI_CTORATstpSPTraderSpiOnRtnMarketStatusPacket),OnRtnTradingNotice(TORASPAPI_CTORATstpSPTraderSpiOnRtnTradingNoticePacket),OnRspQryExchange(TORASPAPI_CTORATstpSPTraderSpiOnRspQryExchangePacket),OnRspQryMarketData(TORASPAPI_CTORATstpSPTraderSpiOnRspQryMarketDataPacket),OnRspQrySecurity(TORASPAPI_CTORATstpSPTraderSpiOnRspQrySecurityPacket),OnRspQryBUProxy(TORASPAPI_CTORATstpSPTraderSpiOnRspQryBUProxyPacket),OnRspQryUser(TORASPAPI_CTORATstpSPTraderSpiOnRspQryUserPacket),OnRspQryInvestor(TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorPacket),OnRspQryShareholderAccount(TORASPAPI_CTORATstpSPTraderSpiOnRspQryShareholderAccountPacket),OnRspQryTradingAccount(TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingAccountPacket),OnRspQryOrder(TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderPacket),OnRspQryTrade(TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradePacket),OnRspQryPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionPacket),OnRspQryTradingFee(TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingFeePacket),OnRspQryInvestorTradingFee(TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorTradingFeePacket),OnRspQryInvestorMarginFee(TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorMarginFeePacket),OnRspQryOrderFundDetail(TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderFundDetailPacket),OnRspQryFundTransferDetail(TORASPAPI_CTORATstpSPTraderSpiOnRspQryFundTransferDetailPacket),OnRspQryPositionTransferDetail(TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionTransferDetailPacket),OnRspQryOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderActionPacket),OnRspQryStockPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionPacket),OnRspQryLock(TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPacket),OnRspQryExercise(TORASPAPI_CTORATstpSPTraderSpiOnRspQryExercisePacket),OnRspQryLockPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPositionPacket),OnRspQryExerciseAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseActionPacket),OnRspQryLockAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockActionPacket),OnRspQryStockPositionTransferDetail(TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionTransferDetailPacket),OnRspQryTradingNotice(TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingNoticePacket),OnRspQryStockDisposal(TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalPacket),OnRspQryStockDisposalAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalActionPacket),OnRspQryCondOrder(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderPacket),OnRspQryCondOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderActionPacket),OnRspQryInvestorLimitPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitPositionPacket),OnRspQryInvestorLimitAmount(TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitAmountPacket),OnRspQryCombOrderAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderActionPacket),OnRspQryCombOrder(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderPacket),OnRspQryCombPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPositionPacket),OnRspQryCombPosDetail(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPosDetailPacket),OnRspQryExerciseAppointment(TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseAppointmentPacket),OnRspQryInsufficientCoveredStockPosition(TORASPAPI_CTORATstpSPTraderSpiOnRspQryInsufficientCoveredStockPositionPacket),OnRspQryCombSecurity(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombSecurityPacket),OnRspQryCombExercise(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExercisePacket),OnRspQryCombExerciseAction(TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExerciseActionPacket), } 

            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspErrorPacket {
                pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspUserLoginPacket {
                pub p_rsp_user_login_field : Option<TORASPAPI_CTORATstpSPRspUserLoginField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspUserLogoutPacket {
                pub p_user_logout_field : Option<TORASPAPI_CTORATstpSPUserLogoutField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspUserPasswordUpdatePacket {
                pub p_user_password_update_field : Option<TORASPAPI_CTORATstpSPUserPasswordUpdateField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInputDeviceSerialPacket {
                pub p_rsp_input_device_serial_field : Option<TORASPAPI_CTORATstpSPRspInputDeviceSerialField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspOrderInsertPacket {
                pub p_input_order_field : Option<TORASPAPI_CTORATstpSPInputOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnOrderPacket {
                pub p_order : Option<TORASPAPI_CTORATstpSPOrderField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderInsertPacket {
                pub p_input_order : Option<TORASPAPI_CTORATstpSPInputOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspOrderActionPacket {
                pub p_input_order_action_field : Option<TORASPAPI_CTORATstpSPInputOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderActionPacket {
                pub p_input_order_action : Option<TORASPAPI_CTORATstpSPInputOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnTradePacket {
                pub p_trade : Option<TORASPAPI_CTORATstpSPTradeField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseInsertPacket {
                pub p_input_exercise_field : Option<TORASPAPI_CTORATstpSPInputExerciseField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnExercisePacket {
                pub p_exercise : Option<TORASPAPI_CTORATstpSPExerciseField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseInsertPacket {
                pub p_input_exercise : Option<TORASPAPI_CTORATstpSPInputExerciseField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseActionPacket {
                pub p_input_exercise_action_field : Option<TORASPAPI_CTORATstpSPInputExerciseActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseActionPacket {
                pub p_input_exercise_action : Option<TORASPAPI_CTORATstpSPInputExerciseActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspLockInsertPacket {
                pub p_input_lock_field : Option<TORASPAPI_CTORATstpSPInputLockField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnLockPacket {
                pub p_lock : Option<TORASPAPI_CTORATstpSPLockField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockInsertPacket {
                pub p_input_lock : Option<TORASPAPI_CTORATstpSPInputLockField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspLockActionPacket {
                pub p_input_lock_action_field : Option<TORASPAPI_CTORATstpSPInputLockActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockActionPacket {
                pub p_input_lock_action : Option<TORASPAPI_CTORATstpSPInputLockActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnStockDisposalPacket {
                pub p_stock_disposal : Option<TORASPAPI_CTORATstpSPStockDisposalField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderInsertPacket {
                pub p_input_comb_order_field : Option<TORASPAPI_CTORATstpSPInputCombOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnCombOrderPacket {
                pub p_comb_order : Option<TORASPAPI_CTORATstpSPCombOrderField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderInsertPacket {
                pub p_input_comb_order : Option<TORASPAPI_CTORATstpSPInputCombOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderActionPacket {
                pub p_input_comb_order_action_field : Option<TORASPAPI_CTORATstpSPInputCombOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderActionPacket {
                pub p_input_comb_order_action : Option<TORASPAPI_CTORATstpSPInputCombOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderInsertPacket {
                pub p_input_cond_order_field : Option<TORASPAPI_CTORATstpSPInputCondOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnCondOrderPacket {
                pub p_condition_order : Option<TORASPAPI_CTORATstpSPConditionOrderField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderInsertPacket {
                pub p_input_cond_order : Option<TORASPAPI_CTORATstpSPInputCondOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderActionPacket {
                pub p_input_cond_order_action_field : Option<TORASPAPI_CTORATstpSPInputCondOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderActionPacket {
                pub p_input_cond_order_action : Option<TORASPAPI_CTORATstpSPInputCondOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseInsertPacket {
                pub p_input_comb_exercise_field : Option<TORASPAPI_CTORATstpSPInputCombExerciseField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnCombExercisePacket {
                pub p_comb_exercise : Option<TORASPAPI_CTORATstpSPCombExerciseField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseInsertPacket {
                pub p_input_comb_exercise : Option<TORASPAPI_CTORATstpSPInputCombExerciseField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseActionPacket {
                pub p_input_comb_exercise_action_field : Option<TORASPAPI_CTORATstpSPInputCombExerciseActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseActionPacket {
                pub p_input_comb_exercise_action : Option<TORASPAPI_CTORATstpSPInputCombExerciseActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxLockVolumePacket {
                pub p_rsp_inquiry_max_lock_volume_field : Option<TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxCoverVolumePacket {
                pub p_rsp_inquiry_max_cover_volume_field : Option<TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInquirySplitCombMarginDifferencePacket {
                pub p_rsp_inquiry_split_comb_margin_difference_field : Option<TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspTransferFundPacket {
                pub p_input_transfer_fund_field : Option<TORASPAPI_CTORATstpSPInputTransferFundField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferFundPacket {
                pub p_transfer_fund : Option<TORASPAPI_CTORATstpSPTransferFundField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferFundPacket {
                pub p_input_transfer_fund : Option<TORASPAPI_CTORATstpSPInputTransferFundField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferPositionPacket {
                pub p_transfer_position : Option<TORASPAPI_CTORATstpSPTransferPositionField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferPositionPacket {
                pub p_input_transfer_position : Option<TORASPAPI_CTORATstpSPInputTransferPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspTransferStockPositionPacket {
                pub p_input_transfer_stock_position_field : Option<TORASPAPI_CTORATstpSPInputTransferStockPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferStockPositionPacket {
                pub p_transfer_stock_position : Option<TORASPAPI_CTORATstpSPTransferStockPositionField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferStockPositionPacket {
                pub p_input_transfer_stock_position : Option<TORASPAPI_CTORATstpSPInputTransferStockPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryJZFundPacket {
                pub p_rsp_inquiry_jz_fund_field : Option<TORASPAPI_CTORATstpSPRspInquiryJZFundField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryBankAccountFundPacket {
                pub p_rsp_inquiry_bank_account_fund_field : Option<TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryStockPositionPacket {
                pub p_rsp_inquiry_stock_position_field : Option<TORASPAPI_CTORATstpSPRspInquiryStockPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnMarketStatusPacket {
                pub p_market_status : Option<TORASPAPI_CTORATstpSPMarketStatusField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRtnTradingNoticePacket {
                pub p_trading_notice : Option<TORASPAPI_CTORATstpSPTradingNoticeField>,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryExchangePacket {
                pub p_exchange : Option<TORASPAPI_CTORATstpSPExchangeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryMarketDataPacket {
                pub p_market_data : Option<TORASPAPI_CTORATstpSPMarketDataField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQrySecurityPacket {
                pub p_security : Option<TORASPAPI_CTORATstpSPSecurityField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryBUProxyPacket {
                pub p_bu_proxy : Option<TORASPAPI_CTORATstpSPBUProxyField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryUserPacket {
                pub p_user : Option<TORASPAPI_CTORATstpSPUserField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorPacket {
                pub p_investor : Option<TORASPAPI_CTORATstpSPInvestorField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryShareholderAccountPacket {
                pub p_shareholder_account : Option<TORASPAPI_CTORATstpSPShareholderAccountField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingAccountPacket {
                pub p_trading_account : Option<TORASPAPI_CTORATstpSPTradingAccountField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderPacket {
                pub p_order : Option<TORASPAPI_CTORATstpSPOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradePacket {
                pub p_trade : Option<TORASPAPI_CTORATstpSPTradeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionPacket {
                pub p_position : Option<TORASPAPI_CTORATstpSPPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingFeePacket {
                pub p_trading_fee : Option<TORASPAPI_CTORATstpSPTradingFeeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorTradingFeePacket {
                pub p_investor_trading_fee : Option<TORASPAPI_CTORATstpSPInvestorTradingFeeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorMarginFeePacket {
                pub p_investor_margin_fee : Option<TORASPAPI_CTORATstpSPInvestorMarginFeeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderFundDetailPacket {
                pub p_order_fund_detail : Option<TORASPAPI_CTORATstpSPOrderFundDetailField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryFundTransferDetailPacket {
                pub p_fund_transfer_detail : Option<TORASPAPI_CTORATstpSPFundTransferDetailField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionTransferDetailPacket {
                pub p_position_transfer_detail : Option<TORASPAPI_CTORATstpSPPositionTransferDetailField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderActionPacket {
                pub p_order_action : Option<TORASPAPI_CTORATstpSPOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionPacket {
                pub p_stock_position : Option<TORASPAPI_CTORATstpSPStockPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPacket {
                pub p_lock : Option<TORASPAPI_CTORATstpSPLockField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryExercisePacket {
                pub p_exercise : Option<TORASPAPI_CTORATstpSPExerciseField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPositionPacket {
                pub p_lock_position : Option<TORASPAPI_CTORATstpSPLockPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseActionPacket {
                pub p_exercise_action : Option<TORASPAPI_CTORATstpSPExerciseActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockActionPacket {
                pub p_lock_action : Option<TORASPAPI_CTORATstpSPLockActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionTransferDetailPacket {
                pub p_stock_position_transfer_detail : Option<TORASPAPI_CTORATstpSPStockPositionTransferDetailField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingNoticePacket {
                pub p_trading_notice : Option<TORASPAPI_CTORATstpSPTradingNoticeField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalPacket {
                pub p_stock_disposal : Option<TORASPAPI_CTORATstpSPStockDisposalField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalActionPacket {
                pub p_stock_disposal_action : Option<TORASPAPI_CTORATstpSPStockDisposalActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderPacket {
                pub p_cond_order : Option<TORASPAPI_CTORATstpSPCondOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderActionPacket {
                pub p_cond_order_action : Option<TORASPAPI_CTORATstpSPCondOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitPositionPacket {
                pub p_investor_limit_position : Option<TORASPAPI_CTORATstpSPInvestorLimitPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitAmountPacket {
                pub p_investor_limit_amount : Option<TORASPAPI_CTORATstpSPInvestorLimitAmountField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderActionPacket {
                pub p_comb_order_action : Option<TORASPAPI_CTORATstpSPCombOrderActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderPacket {
                pub p_comb_order : Option<TORASPAPI_CTORATstpSPCombOrderField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPositionPacket {
                pub p_comb_position : Option<TORASPAPI_CTORATstpSPCombPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPosDetailPacket {
                pub p_comb_pos_detail : Option<TORASPAPI_CTORATstpSPCombPosDetailField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseAppointmentPacket {
                pub p_exercise_appointment : Option<TORASPAPI_CTORATstpSPExerciseAppointmentField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryInsufficientCoveredStockPositionPacket {
                pub p_insufficient_covered_stock_position : Option<TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombSecurityPacket {
                pub p_comb_security : Option<TORASPAPI_CTORATstpSPCombSecurityField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExercisePacket {
                pub p_comb_exercise : Option<TORASPAPI_CTORATstpSPCombExerciseField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExerciseActionPacket {
                pub p_comb_exercise_action : Option<TORASPAPI_CTORATstpSPCombExerciseActionField>,pub p_rsp_info : Option<TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }  
static TORASPAPI_CTORA_TSTP_SP_TRADER_SPI_VTABLE: TORASPAPI_CTORATstpSPTraderSpiVTable = TORASPAPI_CTORATstpSPTraderSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_rsp_error: spi_on_rsp_error,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_user_password_update: spi_on_rsp_user_password_update,
            on_rsp_input_device_serial: spi_on_rsp_input_device_serial,
            on_rsp_order_insert: spi_on_rsp_order_insert,
            on_rtn_order: spi_on_rtn_order,
            on_err_rtn_order_insert: spi_on_err_rtn_order_insert,
            on_rsp_order_action: spi_on_rsp_order_action,
            on_err_rtn_order_action: spi_on_err_rtn_order_action,
            on_rtn_trade: spi_on_rtn_trade,
            on_rsp_exercise_insert: spi_on_rsp_exercise_insert,
            on_rtn_exercise: spi_on_rtn_exercise,
            on_err_rtn_exercise_insert: spi_on_err_rtn_exercise_insert,
            on_rsp_exercise_action: spi_on_rsp_exercise_action,
            on_err_rtn_exercise_action: spi_on_err_rtn_exercise_action,
            on_rsp_lock_insert: spi_on_rsp_lock_insert,
            on_rtn_lock: spi_on_rtn_lock,
            on_err_rtn_lock_insert: spi_on_err_rtn_lock_insert,
            on_rsp_lock_action: spi_on_rsp_lock_action,
            on_err_rtn_lock_action: spi_on_err_rtn_lock_action,
            on_rtn_stock_disposal: spi_on_rtn_stock_disposal,
            on_rsp_comb_order_insert: spi_on_rsp_comb_order_insert,
            on_rtn_comb_order: spi_on_rtn_comb_order,
            on_err_rtn_comb_order_insert: spi_on_err_rtn_comb_order_insert,
            on_rsp_comb_order_action: spi_on_rsp_comb_order_action,
            on_err_rtn_comb_order_action: spi_on_err_rtn_comb_order_action,
            on_rsp_cond_order_insert: spi_on_rsp_cond_order_insert,
            on_rtn_cond_order: spi_on_rtn_cond_order,
            on_err_rtn_cond_order_insert: spi_on_err_rtn_cond_order_insert,
            on_rsp_cond_order_action: spi_on_rsp_cond_order_action,
            on_err_rtn_cond_order_action: spi_on_err_rtn_cond_order_action,
            on_rsp_comb_exercise_insert: spi_on_rsp_comb_exercise_insert,
            on_rtn_comb_exercise: spi_on_rtn_comb_exercise,
            on_err_rtn_comb_exercise_insert: spi_on_err_rtn_comb_exercise_insert,
            on_rsp_comb_exercise_action: spi_on_rsp_comb_exercise_action,
            on_err_rtn_comb_exercise_action: spi_on_err_rtn_comb_exercise_action,
            on_rsp_inquiry_max_lock_volume: spi_on_rsp_inquiry_max_lock_volume,
            on_rsp_inquiry_max_cover_volume: spi_on_rsp_inquiry_max_cover_volume,
            on_rsp_inquiry_split_comb_margin_difference: spi_on_rsp_inquiry_split_comb_margin_difference,
            on_rsp_transfer_fund: spi_on_rsp_transfer_fund,
            on_rtn_transfer_fund: spi_on_rtn_transfer_fund,
            on_err_rtn_transfer_fund: spi_on_err_rtn_transfer_fund,
            on_rtn_transfer_position: spi_on_rtn_transfer_position,
            on_err_rtn_transfer_position: spi_on_err_rtn_transfer_position,
            on_rsp_transfer_stock_position: spi_on_rsp_transfer_stock_position,
            on_rtn_transfer_stock_position: spi_on_rtn_transfer_stock_position,
            on_err_rtn_transfer_stock_position: spi_on_err_rtn_transfer_stock_position,
            on_rsp_inquiry_jz_fund: spi_on_rsp_inquiry_jz_fund,
            on_rsp_inquiry_bank_account_fund: spi_on_rsp_inquiry_bank_account_fund,
            on_rsp_inquiry_stock_position: spi_on_rsp_inquiry_stock_position,
            on_rtn_market_status: spi_on_rtn_market_status,
            on_rtn_trading_notice: spi_on_rtn_trading_notice,
            on_rsp_qry_exchange: spi_on_rsp_qry_exchange,
            on_rsp_qry_market_data: spi_on_rsp_qry_market_data,
            on_rsp_qry_security: spi_on_rsp_qry_security,
            on_rsp_qry_bu_proxy: spi_on_rsp_qry_bu_proxy,
            on_rsp_qry_user: spi_on_rsp_qry_user,
            on_rsp_qry_investor: spi_on_rsp_qry_investor,
            on_rsp_qry_shareholder_account: spi_on_rsp_qry_shareholder_account,
            on_rsp_qry_trading_account: spi_on_rsp_qry_trading_account,
            on_rsp_qry_order: spi_on_rsp_qry_order,
            on_rsp_qry_trade: spi_on_rsp_qry_trade,
            on_rsp_qry_position: spi_on_rsp_qry_position,
            on_rsp_qry_trading_fee: spi_on_rsp_qry_trading_fee,
            on_rsp_qry_investor_trading_fee: spi_on_rsp_qry_investor_trading_fee,
            on_rsp_qry_investor_margin_fee: spi_on_rsp_qry_investor_margin_fee,
            on_rsp_qry_order_fund_detail: spi_on_rsp_qry_order_fund_detail,
            on_rsp_qry_fund_transfer_detail: spi_on_rsp_qry_fund_transfer_detail,
            on_rsp_qry_position_transfer_detail: spi_on_rsp_qry_position_transfer_detail,
            on_rsp_qry_order_action: spi_on_rsp_qry_order_action,
            on_rsp_qry_stock_position: spi_on_rsp_qry_stock_position,
            on_rsp_qry_lock: spi_on_rsp_qry_lock,
            on_rsp_qry_exercise: spi_on_rsp_qry_exercise,
            on_rsp_qry_lock_position: spi_on_rsp_qry_lock_position,
            on_rsp_qry_exercise_action: spi_on_rsp_qry_exercise_action,
            on_rsp_qry_lock_action: spi_on_rsp_qry_lock_action,
            on_rsp_qry_stock_position_transfer_detail: spi_on_rsp_qry_stock_position_transfer_detail,
            on_rsp_qry_trading_notice: spi_on_rsp_qry_trading_notice,
            on_rsp_qry_stock_disposal: spi_on_rsp_qry_stock_disposal,
            on_rsp_qry_stock_disposal_action: spi_on_rsp_qry_stock_disposal_action,
            on_rsp_qry_cond_order: spi_on_rsp_qry_cond_order,
            on_rsp_qry_cond_order_action: spi_on_rsp_qry_cond_order_action,
            on_rsp_qry_investor_limit_position: spi_on_rsp_qry_investor_limit_position,
            on_rsp_qry_investor_limit_amount: spi_on_rsp_qry_investor_limit_amount,
            on_rsp_qry_comb_order_action: spi_on_rsp_qry_comb_order_action,
            on_rsp_qry_comb_order: spi_on_rsp_qry_comb_order,
            on_rsp_qry_comb_position: spi_on_rsp_qry_comb_position,
            on_rsp_qry_comb_pos_detail: spi_on_rsp_qry_comb_pos_detail,
            on_rsp_qry_exercise_appointment: spi_on_rsp_qry_exercise_appointment,
            on_rsp_qry_insufficient_covered_stock_position: spi_on_rsp_qry_insufficient_covered_stock_position,
            on_rsp_qry_comb_security: spi_on_rsp_qry_comb_security,
            on_rsp_qry_comb_exercise: spi_on_rsp_qry_comb_exercise,
            on_rsp_qry_comb_exercise_action: spi_on_rsp_qry_comb_exercise_action,
             };
extern "C" fn spi_on_front_connected(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_rsp_error(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_error(p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_user_login_field : * const TORASPAPI_CTORATstpSPRspUserLoginField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_user_logout_field : * const TORASPAPI_CTORATstpSPUserLogoutField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_password_update(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_user_password_update_field : * const TORASPAPI_CTORATstpSPUserPasswordUpdateField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_password_update(p_user_password_update_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_input_device_serial(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_input_device_serial_field : * const TORASPAPI_CTORATstpSPRspInputDeviceSerialField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_input_device_serial(p_rsp_input_device_serial_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_order_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_field : * const TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_insert(p_input_order_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_order(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const TORASPAPI_CTORATstpSPOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_order(p_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_order_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order : * const TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_insert(p_input_order.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action_field : * const TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_action(p_input_order_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action : * const TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_action(p_input_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_trade(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const TORASPAPI_CTORATstpSPTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trade(p_trade.as_ref())
                    }
                }extern "C" fn spi_on_rsp_exercise_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_field : * const TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_exercise_insert(p_input_exercise_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_exercise(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const TORASPAPI_CTORATstpSPExerciseField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_exercise(p_exercise.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_exercise_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise : * const TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_exercise_insert(p_input_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_exercise_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action_field : * const TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_exercise_action(p_input_exercise_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_exercise_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action : * const TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_exercise_action(p_input_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_lock_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_field : * const TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_lock_insert(p_input_lock_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_lock(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const TORASPAPI_CTORATstpSPLockField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_lock(p_lock.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_lock_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock : * const TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_lock_insert(p_input_lock.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_lock_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action_field : * const TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_lock_action(p_input_lock_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_lock_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action : * const TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_lock_action(p_input_lock_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_stock_disposal(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const TORASPAPI_CTORATstpSPStockDisposalField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_stock_disposal(p_stock_disposal.as_ref())
                    }
                }extern "C" fn spi_on_rsp_comb_order_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_field : * const TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_order_insert(p_input_comb_order_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_comb_order(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const TORASPAPI_CTORATstpSPCombOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_comb_order(p_comb_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_comb_order_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order : * const TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_order_insert(p_input_comb_order.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_comb_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action_field : * const TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_order_action(p_input_comb_order_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_comb_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action : * const TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_order_action(p_input_comb_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_field : * const TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_insert(p_input_cond_order_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_cond_order(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_condition_order : * const TORASPAPI_CTORATstpSPConditionOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_cond_order(p_condition_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order : * const TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_insert(p_input_cond_order.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action_field : * const TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_action(p_input_cond_order_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action : * const TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_action(p_input_cond_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_comb_exercise_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_field : * const TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_exercise_insert(p_input_comb_exercise_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_comb_exercise(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const TORASPAPI_CTORATstpSPCombExerciseField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_comb_exercise(p_comb_exercise.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_comb_exercise_insert(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise : * const TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_exercise_insert(p_input_comb_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_comb_exercise_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action_field : * const TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_exercise_action(p_input_comb_exercise_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_comb_exercise_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action : * const TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_exercise_action(p_input_comb_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_max_lock_volume(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_lock_volume_field : * const TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_max_lock_volume(p_rsp_inquiry_max_lock_volume_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_max_cover_volume(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_cover_volume_field : * const TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_max_cover_volume(p_rsp_inquiry_max_cover_volume_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_split_comb_margin_difference(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_split_comb_margin_difference_field : * const TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_split_comb_margin_difference(p_rsp_inquiry_split_comb_margin_difference_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_transfer_fund(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund_field : * const TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_fund(p_input_transfer_fund_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_fund(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_fund : * const TORASPAPI_CTORATstpSPTransferFundField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_fund(p_transfer_fund.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_transfer_fund(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund : * const TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_fund(p_input_transfer_fund.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_position : * const TORASPAPI_CTORATstpSPTransferPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_position(p_transfer_position.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_transfer_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_position : * const TORASPAPI_CTORATstpSPInputTransferPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_position(p_input_transfer_position.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_transfer_stock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position_field : * const TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_stock_position(p_input_transfer_stock_position_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_stock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_stock_position : * const TORASPAPI_CTORATstpSPTransferStockPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_stock_position(p_transfer_stock_position.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_transfer_stock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position : * const TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_stock_position(p_input_transfer_stock_position.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_jz_fund(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const TORASPAPI_CTORATstpSPRspInquiryJZFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_jz_fund(p_rsp_inquiry_jz_fund_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_bank_account_fund(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_bank_account_fund(p_rsp_inquiry_bank_account_fund_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_stock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_stock_position_field : * const TORASPAPI_CTORATstpSPRspInquiryStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_stock_position(p_rsp_inquiry_stock_position_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_market_status(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_market_status : * const TORASPAPI_CTORATstpSPMarketStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_market_status(p_market_status.as_ref())
                    }
                }extern "C" fn spi_on_rtn_trading_notice(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const TORASPAPI_CTORATstpSPTradingNoticeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trading_notice(p_trading_notice.as_ref())
                    }
                }extern "C" fn spi_on_rsp_qry_exchange(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exchange : * const TORASPAPI_CTORATstpSPExchangeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange(p_exchange.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_market_data(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_market_data : * const TORASPAPI_CTORATstpSPMarketDataField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_market_data(p_market_data.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_security(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_security : * const TORASPAPI_CTORATstpSPSecurityField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_security(p_security.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_bu_proxy(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_bu_proxy : * const TORASPAPI_CTORATstpSPBUProxyField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_bu_proxy(p_bu_proxy.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_user(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_user : * const TORASPAPI_CTORATstpSPUserField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_user(p_user.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor : * const TORASPAPI_CTORATstpSPInvestorField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor(p_investor.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_shareholder_account(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_shareholder_account : * const TORASPAPI_CTORATstpSPShareholderAccountField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_shareholder_account(p_shareholder_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_account : * const TORASPAPI_CTORATstpSPTradingAccountField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_account(p_trading_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const TORASPAPI_CTORATstpSPOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order(p_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trade(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const TORASPAPI_CTORATstpSPTradeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trade(p_trade.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_position : * const TORASPAPI_CTORATstpSPPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position(p_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_fee(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_fee : * const TORASPAPI_CTORATstpSPTradingFeeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_fee(p_trading_fee.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_trading_fee(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_trading_fee : * const TORASPAPI_CTORATstpSPInvestorTradingFeeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_trading_fee(p_investor_trading_fee.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_margin_fee(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_margin_fee : * const TORASPAPI_CTORATstpSPInvestorMarginFeeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_margin_fee(p_investor_margin_fee.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_fund_detail(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order_fund_detail : * const TORASPAPI_CTORATstpSPOrderFundDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_fund_detail(p_order_fund_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_fund_transfer_detail(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_fund_transfer_detail : * const TORASPAPI_CTORATstpSPFundTransferDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_fund_transfer_detail(p_fund_transfer_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position_transfer_detail(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_position_transfer_detail : * const TORASPAPI_CTORATstpSPPositionTransferDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position_transfer_detail(p_position_transfer_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_order_action : * const TORASPAPI_CTORATstpSPOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_action(p_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position : * const TORASPAPI_CTORATstpSPStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_position(p_stock_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lock(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const TORASPAPI_CTORATstpSPLockField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lock(p_lock.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exercise(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const TORASPAPI_CTORATstpSPExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exercise(p_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_position : * const TORASPAPI_CTORATstpSPLockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lock_position(p_lock_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exercise_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_action : * const TORASPAPI_CTORATstpSPExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exercise_action(p_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lock_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_action : * const TORASPAPI_CTORATstpSPLockActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lock_action(p_lock_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_position_transfer_detail(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position_transfer_detail : * const TORASPAPI_CTORATstpSPStockPositionTransferDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_position_transfer_detail(p_stock_position_transfer_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const TORASPAPI_CTORATstpSPTradingNoticeField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_notice(p_trading_notice.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_disposal(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const TORASPAPI_CTORATstpSPStockDisposalField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_disposal(p_stock_disposal.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_disposal_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal_action : * const TORASPAPI_CTORATstpSPStockDisposalActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_disposal_action(p_stock_disposal_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cond_order(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order : * const TORASPAPI_CTORATstpSPCondOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cond_order(p_cond_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cond_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order_action : * const TORASPAPI_CTORATstpSPCondOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cond_order_action(p_cond_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_limit_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_position : * const TORASPAPI_CTORATstpSPInvestorLimitPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_limit_position(p_investor_limit_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_limit_amount(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_amount : * const TORASPAPI_CTORATstpSPInvestorLimitAmountField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_limit_amount(p_investor_limit_amount.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_order_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order_action : * const TORASPAPI_CTORATstpSPCombOrderActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_order_action(p_comb_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_order(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const TORASPAPI_CTORATstpSPCombOrderField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_order(p_comb_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_position : * const TORASPAPI_CTORATstpSPCombPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_position(p_comb_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_pos_detail(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_pos_detail : * const TORASPAPI_CTORATstpSPCombPosDetailField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_pos_detail(p_comb_pos_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exercise_appointment(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_appointment : * const TORASPAPI_CTORATstpSPExerciseAppointmentField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exercise_appointment(p_exercise_appointment.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_insufficient_covered_stock_position(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_insufficient_covered_stock_position : * const TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_insufficient_covered_stock_position(p_insufficient_covered_stock_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_security(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_security : * const TORASPAPI_CTORATstpSPCombSecurityField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_security(p_comb_security.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_exercise(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const TORASPAPI_CTORATstpSPCombExerciseField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_exercise(p_comb_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_exercise_action(spi: *mut TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise_action : * const TORASPAPI_CTORATstpSPCombExerciseActionField,p_rsp_info : * const TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_exercise_action(p_comb_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }

        #[repr(C)]
        pub struct TORASPAPI_CTORATstpSPTraderSpiFat {
            vtable: *const TORASPAPI_CTORATstpSPTraderSpiVTable,
            pub md_spi_ptr: *mut dyn TORASPAPI_CTORATstpSPTraderSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct TORASPAPI_CTORATstpSPTraderSpiInner {
            buf: std::collections::VecDeque<TORASPAPI_CTORATstpSPTraderSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl TORASPAPI_CTORATstpSPTraderSpiInner {
            fn push(&mut self, msg: TORASPAPI_CTORATstpSPTraderSpiOutput) {
                self.buf.push_back(msg);
                if let Some(ref waker) = &self.waker {
                    waker.clone().wake()
                }
            }
        }
        
        pub struct TORASPAPI_CTORATstpSPTraderSpiStream {
            inner: Arc<Mutex<TORASPAPI_CTORATstpSPTraderSpiInner>>,
        }
        
        impl Stream for TORASPAPI_CTORATstpSPTraderSpiStream {
            type Item = TORASPAPI_CTORATstpSPTraderSpiOutput;
        
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
        
        pub fn create_spi() -> (Box<TORASPAPI_CTORATstpSPTraderSpiStream>, *mut TORASPAPI_CTORATstpSPTraderSpiStream) {
            let i = TORASPAPI_CTORATstpSPTraderSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = TORASPAPI_CTORATstpSPTraderSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl TORASPAPI_CTORATstpSPTraderSpi_trait for TORASPAPI_CTORATstpSPTraderSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnFrontConnected( TORASPAPI_CTORATstpSPTraderSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnFrontDisconnected( TORASPAPI_CTORATstpSPTraderSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_rsp_error(&mut self, p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspError( TORASPAPI_CTORATstpSPTraderSpiOnRspErrorPacket { p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&TORASPAPI_CTORATstpSPRspUserLoginField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspUserLogin( TORASPAPI_CTORATstpSPTraderSpiOnRspUserLoginPacket { p_rsp_user_login_field:p_rsp_user_login_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&TORASPAPI_CTORATstpSPUserLogoutField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspUserLogout( TORASPAPI_CTORATstpSPTraderSpiOnRspUserLogoutPacket { p_user_logout_field:p_user_logout_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&TORASPAPI_CTORATstpSPUserPasswordUpdateField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspUserPasswordUpdate( TORASPAPI_CTORATstpSPTraderSpiOnRspUserPasswordUpdatePacket { p_user_password_update_field:p_user_password_update_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&TORASPAPI_CTORATstpSPRspInputDeviceSerialField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInputDeviceSerial( TORASPAPI_CTORATstpSPTraderSpiOnRspInputDeviceSerialPacket { p_rsp_input_device_serial_field:p_rsp_input_device_serial_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspOrderInsert( TORASPAPI_CTORATstpSPTraderSpiOnRspOrderInsertPacket { p_input_order_field:p_input_order_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_order(&mut self, p_order : Option<&TORASPAPI_CTORATstpSPOrderField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnOrder( TORASPAPI_CTORATstpSPTraderSpiOnRtnOrderPacket { p_order:p_order.cloned() } ))
                }
            fn on_err_rtn_order_insert(&mut self, p_input_order : Option<&TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnOrderInsert( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderInsertPacket { p_input_order:p_input_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnRspOrderActionPacket { p_input_order_action_field:p_input_order_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_order_action(&mut self, p_input_order_action : Option<&TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderActionPacket { p_input_order_action:p_input_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_trade(&mut self, p_trade : Option<&TORASPAPI_CTORATstpSPTradeField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTrade( TORASPAPI_CTORATstpSPTraderSpiOnRtnTradePacket { p_trade:p_trade.cloned() } ))
                }
            fn on_rsp_exercise_insert(&mut self, p_input_exercise_field : Option<&TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspExerciseInsert( TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseInsertPacket { p_input_exercise_field:p_input_exercise_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_exercise(&mut self, p_exercise : Option<&TORASPAPI_CTORATstpSPExerciseField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnExercise( TORASPAPI_CTORATstpSPTraderSpiOnRtnExercisePacket { p_exercise:p_exercise.cloned() } ))
                }
            fn on_err_rtn_exercise_insert(&mut self, p_input_exercise : Option<&TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnExerciseInsert( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseInsertPacket { p_input_exercise:p_input_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_exercise_action(&mut self, p_input_exercise_action_field : Option<&TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspExerciseAction( TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseActionPacket { p_input_exercise_action_field:p_input_exercise_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_exercise_action(&mut self, p_input_exercise_action : Option<&TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnExerciseAction( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseActionPacket { p_input_exercise_action:p_input_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_lock_insert(&mut self, p_input_lock_field : Option<&TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspLockInsert( TORASPAPI_CTORATstpSPTraderSpiOnRspLockInsertPacket { p_input_lock_field:p_input_lock_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_lock(&mut self, p_lock : Option<&TORASPAPI_CTORATstpSPLockField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnLock( TORASPAPI_CTORATstpSPTraderSpiOnRtnLockPacket { p_lock:p_lock.cloned() } ))
                }
            fn on_err_rtn_lock_insert(&mut self, p_input_lock : Option<&TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnLockInsert( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockInsertPacket { p_input_lock:p_input_lock.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_lock_action(&mut self, p_input_lock_action_field : Option<&TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspLockAction( TORASPAPI_CTORATstpSPTraderSpiOnRspLockActionPacket { p_input_lock_action_field:p_input_lock_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_lock_action(&mut self, p_input_lock_action : Option<&TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnLockAction( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockActionPacket { p_input_lock_action:p_input_lock_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_stock_disposal(&mut self, p_stock_disposal : Option<&TORASPAPI_CTORATstpSPStockDisposalField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnStockDisposal( TORASPAPI_CTORATstpSPTraderSpiOnRtnStockDisposalPacket { p_stock_disposal:p_stock_disposal.cloned() } ))
                }
            fn on_rsp_comb_order_insert(&mut self, p_input_comb_order_field : Option<&TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombOrderInsert( TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderInsertPacket { p_input_comb_order_field:p_input_comb_order_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_comb_order(&mut self, p_comb_order : Option<&TORASPAPI_CTORATstpSPCombOrderField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnCombOrder( TORASPAPI_CTORATstpSPTraderSpiOnRtnCombOrderPacket { p_comb_order:p_comb_order.cloned() } ))
                }
            fn on_err_rtn_comb_order_insert(&mut self, p_input_comb_order : Option<&TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombOrderInsert( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderInsertPacket { p_input_comb_order:p_input_comb_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_comb_order_action(&mut self, p_input_comb_order_action_field : Option<&TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderActionPacket { p_input_comb_order_action_field:p_input_comb_order_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_comb_order_action(&mut self, p_input_comb_order_action : Option<&TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderActionPacket { p_input_comb_order_action:p_input_comb_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCondOrderInsert( TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderInsertPacket { p_input_cond_order_field:p_input_cond_order_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_cond_order(&mut self, p_condition_order : Option<&TORASPAPI_CTORATstpSPConditionOrderField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnCondOrder( TORASPAPI_CTORATstpSPTraderSpiOnRtnCondOrderPacket { p_condition_order:p_condition_order.cloned() } ))
                }
            fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order : Option<&TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCondOrderInsert( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderInsertPacket { p_input_cond_order:p_input_cond_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCondOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderActionPacket { p_input_cond_order_action_field:p_input_cond_order_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action : Option<&TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCondOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderActionPacket { p_input_cond_order_action:p_input_cond_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_comb_exercise_insert(&mut self, p_input_comb_exercise_field : Option<&TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombExerciseInsert( TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseInsertPacket { p_input_comb_exercise_field:p_input_comb_exercise_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_comb_exercise(&mut self, p_comb_exercise : Option<&TORASPAPI_CTORATstpSPCombExerciseField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnCombExercise( TORASPAPI_CTORATstpSPTraderSpiOnRtnCombExercisePacket { p_comb_exercise:p_comb_exercise.cloned() } ))
                }
            fn on_err_rtn_comb_exercise_insert(&mut self, p_input_comb_exercise : Option<&TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombExerciseInsert( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseInsertPacket { p_input_comb_exercise:p_input_comb_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_comb_exercise_action(&mut self, p_input_comb_exercise_action_field : Option<&TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombExerciseAction( TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseActionPacket { p_input_comb_exercise_action_field:p_input_comb_exercise_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_comb_exercise_action(&mut self, p_input_comb_exercise_action : Option<&TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombExerciseAction( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseActionPacket { p_input_comb_exercise_action:p_input_comb_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_max_lock_volume(&mut self, p_rsp_inquiry_max_lock_volume_field : Option<&TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryMaxLockVolume( TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxLockVolumePacket { p_rsp_inquiry_max_lock_volume_field:p_rsp_inquiry_max_lock_volume_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_max_cover_volume(&mut self, p_rsp_inquiry_max_cover_volume_field : Option<&TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryMaxCoverVolume( TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxCoverVolumePacket { p_rsp_inquiry_max_cover_volume_field:p_rsp_inquiry_max_cover_volume_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_split_comb_margin_difference(&mut self, p_rsp_inquiry_split_comb_margin_difference_field : Option<&TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquirySplitCombMarginDifference( TORASPAPI_CTORATstpSPTraderSpiOnRspInquirySplitCombMarginDifferencePacket { p_rsp_inquiry_split_comb_margin_difference_field:p_rsp_inquiry_split_comb_margin_difference_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspTransferFund( TORASPAPI_CTORATstpSPTraderSpiOnRspTransferFundPacket { p_input_transfer_fund_field:p_input_transfer_fund_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_fund(&mut self, p_transfer_fund : Option<&TORASPAPI_CTORATstpSPTransferFundField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTransferFund( TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferFundPacket { p_transfer_fund:p_transfer_fund.cloned() } ))
                }
            fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund : Option<&TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnTransferFund( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferFundPacket { p_input_transfer_fund:p_input_transfer_fund.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_position(&mut self, p_transfer_position : Option<&TORASPAPI_CTORATstpSPTransferPositionField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTransferPosition( TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferPositionPacket { p_transfer_position:p_transfer_position.cloned() } ))
                }
            fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position : Option<&TORASPAPI_CTORATstpSPInputTransferPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnTransferPosition( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferPositionPacket { p_input_transfer_position:p_input_transfer_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_transfer_stock_position(&mut self, p_input_transfer_stock_position_field : Option<&TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspTransferStockPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspTransferStockPositionPacket { p_input_transfer_stock_position_field:p_input_transfer_stock_position_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_stock_position(&mut self, p_transfer_stock_position : Option<&TORASPAPI_CTORATstpSPTransferStockPositionField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTransferStockPosition( TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferStockPositionPacket { p_transfer_stock_position:p_transfer_stock_position.cloned() } ))
                }
            fn on_err_rtn_transfer_stock_position(&mut self, p_input_transfer_stock_position : Option<&TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnTransferStockPosition( TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferStockPositionPacket { p_input_transfer_stock_position:p_input_transfer_stock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&TORASPAPI_CTORATstpSPRspInquiryJZFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryJZFund( TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryJZFundPacket { p_rsp_inquiry_jz_fund_field:p_rsp_inquiry_jz_fund_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryBankAccountFund( TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryBankAccountFundPacket { p_rsp_inquiry_bank_account_fund_field:p_rsp_inquiry_bank_account_fund_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_stock_position(&mut self, p_rsp_inquiry_stock_position_field : Option<&TORASPAPI_CTORATstpSPRspInquiryStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryStockPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryStockPositionPacket { p_rsp_inquiry_stock_position_field:p_rsp_inquiry_stock_position_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_market_status(&mut self, p_market_status : Option<&TORASPAPI_CTORATstpSPMarketStatusField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnMarketStatus( TORASPAPI_CTORATstpSPTraderSpiOnRtnMarketStatusPacket { p_market_status:p_market_status.cloned() } ))
                }
            fn on_rtn_trading_notice(&mut self, p_trading_notice : Option<&TORASPAPI_CTORATstpSPTradingNoticeField>) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTradingNotice( TORASPAPI_CTORATstpSPTraderSpiOnRtnTradingNoticePacket { p_trading_notice:p_trading_notice.cloned() } ))
                }
            fn on_rsp_qry_exchange(&mut self, p_exchange : Option<&TORASPAPI_CTORATstpSPExchangeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExchange( TORASPAPI_CTORATstpSPTraderSpiOnRspQryExchangePacket { p_exchange:p_exchange.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_market_data(&mut self, p_market_data : Option<&TORASPAPI_CTORATstpSPMarketDataField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryMarketData( TORASPAPI_CTORATstpSPTraderSpiOnRspQryMarketDataPacket { p_market_data:p_market_data.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_security(&mut self, p_security : Option<&TORASPAPI_CTORATstpSPSecurityField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQrySecurity( TORASPAPI_CTORATstpSPTraderSpiOnRspQrySecurityPacket { p_security:p_security.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_bu_proxy(&mut self, p_bu_proxy : Option<&TORASPAPI_CTORATstpSPBUProxyField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryBUProxy( TORASPAPI_CTORATstpSPTraderSpiOnRspQryBUProxyPacket { p_bu_proxy:p_bu_proxy.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_user(&mut self, p_user : Option<&TORASPAPI_CTORATstpSPUserField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryUser( TORASPAPI_CTORATstpSPTraderSpiOnRspQryUserPacket { p_user:p_user.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor(&mut self, p_investor : Option<&TORASPAPI_CTORATstpSPInvestorField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestor( TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorPacket { p_investor:p_investor.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account : Option<&TORASPAPI_CTORATstpSPShareholderAccountField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryShareholderAccount( TORASPAPI_CTORATstpSPTraderSpiOnRspQryShareholderAccountPacket { p_shareholder_account:p_shareholder_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_account(&mut self, p_trading_account : Option<&TORASPAPI_CTORATstpSPTradingAccountField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTradingAccount( TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingAccountPacket { p_trading_account:p_trading_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order(&mut self, p_order : Option<&TORASPAPI_CTORATstpSPOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryOrder( TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderPacket { p_order:p_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trade(&mut self, p_trade : Option<&TORASPAPI_CTORATstpSPTradeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTrade( TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradePacket { p_trade:p_trade.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position(&mut self, p_position : Option<&TORASPAPI_CTORATstpSPPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionPacket { p_position:p_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_fee(&mut self, p_trading_fee : Option<&TORASPAPI_CTORATstpSPTradingFeeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTradingFee( TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingFeePacket { p_trading_fee:p_trading_fee.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee : Option<&TORASPAPI_CTORATstpSPInvestorTradingFeeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorTradingFee( TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorTradingFeePacket { p_investor_trading_fee:p_investor_trading_fee.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_margin_fee(&mut self, p_investor_margin_fee : Option<&TORASPAPI_CTORATstpSPInvestorMarginFeeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorMarginFee( TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorMarginFeePacket { p_investor_margin_fee:p_investor_margin_fee.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail : Option<&TORASPAPI_CTORATstpSPOrderFundDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryOrderFundDetail( TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderFundDetailPacket { p_order_fund_detail:p_order_fund_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail : Option<&TORASPAPI_CTORATstpSPFundTransferDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryFundTransferDetail( TORASPAPI_CTORATstpSPTraderSpiOnRspQryFundTransferDetailPacket { p_fund_transfer_detail:p_fund_transfer_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail : Option<&TORASPAPI_CTORATstpSPPositionTransferDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryPositionTransferDetail( TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionTransferDetailPacket { p_position_transfer_detail:p_position_transfer_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_action(&mut self, p_order_action : Option<&TORASPAPI_CTORATstpSPOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderActionPacket { p_order_action:p_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_position(&mut self, p_stock_position : Option<&TORASPAPI_CTORATstpSPStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionPacket { p_stock_position:p_stock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lock(&mut self, p_lock : Option<&TORASPAPI_CTORATstpSPLockField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryLock( TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPacket { p_lock:p_lock.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exercise(&mut self, p_exercise : Option<&TORASPAPI_CTORATstpSPExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExercise( TORASPAPI_CTORATstpSPTraderSpiOnRspQryExercisePacket { p_exercise:p_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lock_position(&mut self, p_lock_position : Option<&TORASPAPI_CTORATstpSPLockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryLockPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPositionPacket { p_lock_position:p_lock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exercise_action(&mut self, p_exercise_action : Option<&TORASPAPI_CTORATstpSPExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExerciseAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseActionPacket { p_exercise_action:p_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lock_action(&mut self, p_lock_action : Option<&TORASPAPI_CTORATstpSPLockActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryLockAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockActionPacket { p_lock_action:p_lock_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_position_transfer_detail(&mut self, p_stock_position_transfer_detail : Option<&TORASPAPI_CTORATstpSPStockPositionTransferDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockPositionTransferDetail( TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionTransferDetailPacket { p_stock_position_transfer_detail:p_stock_position_transfer_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_notice(&mut self, p_trading_notice : Option<&TORASPAPI_CTORATstpSPTradingNoticeField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTradingNotice( TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingNoticePacket { p_trading_notice:p_trading_notice.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_disposal(&mut self, p_stock_disposal : Option<&TORASPAPI_CTORATstpSPStockDisposalField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockDisposal( TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalPacket { p_stock_disposal:p_stock_disposal.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_disposal_action(&mut self, p_stock_disposal_action : Option<&TORASPAPI_CTORATstpSPStockDisposalActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockDisposalAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalActionPacket { p_stock_disposal_action:p_stock_disposal_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cond_order(&mut self, p_cond_order : Option<&TORASPAPI_CTORATstpSPCondOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCondOrder( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderPacket { p_cond_order:p_cond_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action : Option<&TORASPAPI_CTORATstpSPCondOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCondOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderActionPacket { p_cond_order_action:p_cond_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_limit_position(&mut self, p_investor_limit_position : Option<&TORASPAPI_CTORATstpSPInvestorLimitPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorLimitPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitPositionPacket { p_investor_limit_position:p_investor_limit_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_limit_amount(&mut self, p_investor_limit_amount : Option<&TORASPAPI_CTORATstpSPInvestorLimitAmountField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorLimitAmount( TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitAmountPacket { p_investor_limit_amount:p_investor_limit_amount.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_order_action(&mut self, p_comb_order_action : Option<&TORASPAPI_CTORATstpSPCombOrderActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombOrderAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderActionPacket { p_comb_order_action:p_comb_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_order(&mut self, p_comb_order : Option<&TORASPAPI_CTORATstpSPCombOrderField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombOrder( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderPacket { p_comb_order:p_comb_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_position(&mut self, p_comb_position : Option<&TORASPAPI_CTORATstpSPCombPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPositionPacket { p_comb_position:p_comb_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_pos_detail(&mut self, p_comb_pos_detail : Option<&TORASPAPI_CTORATstpSPCombPosDetailField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombPosDetail( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPosDetailPacket { p_comb_pos_detail:p_comb_pos_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exercise_appointment(&mut self, p_exercise_appointment : Option<&TORASPAPI_CTORATstpSPExerciseAppointmentField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExerciseAppointment( TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseAppointmentPacket { p_exercise_appointment:p_exercise_appointment.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_insufficient_covered_stock_position(&mut self, p_insufficient_covered_stock_position : Option<&TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInsufficientCoveredStockPosition( TORASPAPI_CTORATstpSPTraderSpiOnRspQryInsufficientCoveredStockPositionPacket { p_insufficient_covered_stock_position:p_insufficient_covered_stock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_security(&mut self, p_comb_security : Option<&TORASPAPI_CTORATstpSPCombSecurityField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombSecurity( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombSecurityPacket { p_comb_security:p_comb_security.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_exercise(&mut self, p_comb_exercise : Option<&TORASPAPI_CTORATstpSPCombExerciseField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombExercise( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExercisePacket { p_comb_exercise:p_comb_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_exercise_action(&mut self, p_comb_exercise_action : Option<&TORASPAPI_CTORATstpSPCombExerciseActionField>,p_rsp_info : Option<&TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombExerciseAction( TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExerciseActionPacket { p_comb_exercise_action:p_comb_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
             }

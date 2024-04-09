impl CThostFtdcTraderApi {
                            pub fn release(&mut self) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_Release)(self as *mut CThostFtdcTraderApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_Init)(self as *mut CThostFtdcTraderApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_Join)(self as *mut CThostFtdcTraderApi)
                                        }
                            }
                            pub fn get_trading_day(&mut self) -> *const std::os::raw::c_char {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_GetTradingDay)(self as *mut CThostFtdcTraderApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: std::ffi::CString) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_RegisterFront)(self as *mut CThostFtdcTraderApi,
                                             psz_front_address.into_raw())
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: std::ffi::CString) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_RegisterNameServer)(self as *mut CThostFtdcTraderApi,
                                             psz_ns_address.into_raw())
                                        }
                            }
                            pub fn register_fens_user_info(&mut self, p_fens_user_info: &mut CThostFtdcFensUserInfoField) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_RegisterFensUserInfo)(self as *mut CThostFtdcTraderApi,
                                             p_fens_user_info as * mut CThostFtdcFensUserInfoField)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn CThostFtdcTraderSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &C_THOST_FTDC_TRADER_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).CThostFtdcTraderApi_RegisterSpi)(self as *mut CThostFtdcTraderApi,
                                             p_spi as * mut CThostFtdcTraderSpi)
                                            }
                                }
                            pub fn subscribe_private_topic(&mut self, n_resume_type: THOST_TE_RESUME_TYPE) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_SubscribePrivateTopic)(self as *mut CThostFtdcTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn subscribe_public_topic(&mut self, n_resume_type: THOST_TE_RESUME_TYPE) -> () {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_SubscribePublicTopic)(self as *mut CThostFtdcTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn req_authenticate(&mut self, p_req_authenticate_field: &mut CThostFtdcReqAuthenticateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqAuthenticate)(self as *mut CThostFtdcTraderApi,
                                             p_req_authenticate_field as * mut CThostFtdcReqAuthenticateField,
                                             n_request_id)
                                        }
                            }
                            pub fn register_user_system_info(&mut self, p_user_system_info: &mut CThostFtdcUserSystemInfoField) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_RegisterUserSystemInfo)(self as *mut CThostFtdcTraderApi,
                                             p_user_system_info as * mut CThostFtdcUserSystemInfoField)
                                        }
                            }
                            pub fn submit_user_system_info(&mut self, p_user_system_info: &mut CThostFtdcUserSystemInfoField) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_SubmitUserSystemInfo)(self as *mut CThostFtdcTraderApi,
                                             p_user_system_info as * mut CThostFtdcUserSystemInfoField)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut CThostFtdcReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLogin)(self as *mut CThostFtdcTraderApi,
                                             p_req_user_login_field as * mut CThostFtdcReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout: &mut CThostFtdcUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLogout)(self as *mut CThostFtdcTraderApi,
                                             p_user_logout as * mut CThostFtdcUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_password_update(&mut self, p_user_password_update: &mut CThostFtdcUserPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserPasswordUpdate)(self as *mut CThostFtdcTraderApi,
                                             p_user_password_update as * mut CThostFtdcUserPasswordUpdateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_trading_account_password_update(&mut self, p_trading_account_password_update: &mut CThostFtdcTradingAccountPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqTradingAccountPasswordUpdate)(self as *mut CThostFtdcTraderApi,
                                             p_trading_account_password_update as * mut CThostFtdcTradingAccountPasswordUpdateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_auth_method(&mut self, p_req_user_auth_method: &mut CThostFtdcReqUserAuthMethodField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserAuthMethod)(self as *mut CThostFtdcTraderApi,
                                             p_req_user_auth_method as * mut CThostFtdcReqUserAuthMethodField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_gen_user_captcha(&mut self, p_req_gen_user_captcha: &mut CThostFtdcReqGenUserCaptchaField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqGenUserCaptcha)(self as *mut CThostFtdcTraderApi,
                                             p_req_gen_user_captcha as * mut CThostFtdcReqGenUserCaptchaField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_gen_user_text(&mut self, p_req_gen_user_text: &mut CThostFtdcReqGenUserTextField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqGenUserText)(self as *mut CThostFtdcTraderApi,
                                             p_req_gen_user_text as * mut CThostFtdcReqGenUserTextField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_login_with_captcha(&mut self, p_req_user_login_with_captcha: &mut CThostFtdcReqUserLoginWithCaptchaField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLoginWithCaptcha)(self as *mut CThostFtdcTraderApi,
                                             p_req_user_login_with_captcha as * mut CThostFtdcReqUserLoginWithCaptchaField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_login_with_text(&mut self, p_req_user_login_with_text: &mut CThostFtdcReqUserLoginWithTextField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLoginWithText)(self as *mut CThostFtdcTraderApi,
                                             p_req_user_login_with_text as * mut CThostFtdcReqUserLoginWithTextField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_login_with_otp(&mut self, p_req_user_login_with_otp: &mut CThostFtdcReqUserLoginWithOTPField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLoginWithOTP)(self as *mut CThostFtdcTraderApi,
                                             p_req_user_login_with_otp as * mut CThostFtdcReqUserLoginWithOTPField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_insert(&mut self, p_input_order: &mut CThostFtdcInputOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqOrderInsert)(self as *mut CThostFtdcTraderApi,
                                             p_input_order as * mut CThostFtdcInputOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_parked_order_insert(&mut self, p_parked_order: &mut CThostFtdcParkedOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqParkedOrderInsert)(self as *mut CThostFtdcTraderApi,
                                             p_parked_order as * mut CThostFtdcParkedOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_parked_order_action(&mut self, p_parked_order_action: &mut CThostFtdcParkedOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqParkedOrderAction)(self as *mut CThostFtdcTraderApi,
                                             p_parked_order_action as * mut CThostFtdcParkedOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_action(&mut self, p_input_order_action: &mut CThostFtdcInputOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqOrderAction)(self as *mut CThostFtdcTraderApi,
                                             p_input_order_action as * mut CThostFtdcInputOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_max_order_volume(&mut self, p_qry_max_order_volume: &mut CThostFtdcQryMaxOrderVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryMaxOrderVolume)(self as *mut CThostFtdcTraderApi,
                                             p_qry_max_order_volume as * mut CThostFtdcQryMaxOrderVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_settlement_info_confirm(&mut self, p_settlement_info_confirm: &mut CThostFtdcSettlementInfoConfirmField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqSettlementInfoConfirm)(self as *mut CThostFtdcTraderApi,
                                             p_settlement_info_confirm as * mut CThostFtdcSettlementInfoConfirmField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_remove_parked_order(&mut self, p_remove_parked_order: &mut CThostFtdcRemoveParkedOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqRemoveParkedOrder)(self as *mut CThostFtdcTraderApi,
                                             p_remove_parked_order as * mut CThostFtdcRemoveParkedOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_remove_parked_order_action(&mut self, p_remove_parked_order_action: &mut CThostFtdcRemoveParkedOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqRemoveParkedOrderAction)(self as *mut CThostFtdcTraderApi,
                                             p_remove_parked_order_action as * mut CThostFtdcRemoveParkedOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_exec_order_insert(&mut self, p_input_exec_order: &mut CThostFtdcInputExecOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqExecOrderInsert)(self as *mut CThostFtdcTraderApi,
                                             p_input_exec_order as * mut CThostFtdcInputExecOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_exec_order_action(&mut self, p_input_exec_order_action: &mut CThostFtdcInputExecOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqExecOrderAction)(self as *mut CThostFtdcTraderApi,
                                             p_input_exec_order_action as * mut CThostFtdcInputExecOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_for_quote_insert(&mut self, p_input_for_quote: &mut CThostFtdcInputForQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqForQuoteInsert)(self as *mut CThostFtdcTraderApi,
                                             p_input_for_quote as * mut CThostFtdcInputForQuoteField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_quote_insert(&mut self, p_input_quote: &mut CThostFtdcInputQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQuoteInsert)(self as *mut CThostFtdcTraderApi,
                                             p_input_quote as * mut CThostFtdcInputQuoteField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_quote_action(&mut self, p_input_quote_action: &mut CThostFtdcInputQuoteActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQuoteAction)(self as *mut CThostFtdcTraderApi,
                                             p_input_quote_action as * mut CThostFtdcInputQuoteActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_batch_order_action(&mut self, p_input_batch_order_action: &mut CThostFtdcInputBatchOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqBatchOrderAction)(self as *mut CThostFtdcTraderApi,
                                             p_input_batch_order_action as * mut CThostFtdcInputBatchOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_option_self_close_insert(&mut self, p_input_option_self_close: &mut CThostFtdcInputOptionSelfCloseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqOptionSelfCloseInsert)(self as *mut CThostFtdcTraderApi,
                                             p_input_option_self_close as * mut CThostFtdcInputOptionSelfCloseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_option_self_close_action(&mut self, p_input_option_self_close_action: &mut CThostFtdcInputOptionSelfCloseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqOptionSelfCloseAction)(self as *mut CThostFtdcTraderApi,
                                             p_input_option_self_close_action as * mut CThostFtdcInputOptionSelfCloseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_action_insert(&mut self, p_input_comb_action: &mut CThostFtdcInputCombActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqCombActionInsert)(self as *mut CThostFtdcTraderApi,
                                             p_input_comb_action as * mut CThostFtdcInputCombActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order(&mut self, p_qry_order: &mut CThostFtdcQryOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOrder)(self as *mut CThostFtdcTraderApi,
                                             p_qry_order as * mut CThostFtdcQryOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trade(&mut self, p_qry_trade: &mut CThostFtdcQryTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTrade)(self as *mut CThostFtdcTraderApi,
                                             p_qry_trade as * mut CThostFtdcQryTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_position(&mut self, p_qry_investor_position: &mut CThostFtdcQryInvestorPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPosition)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_position as * mut CThostFtdcQryInvestorPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_account(&mut self, p_qry_trading_account: &mut CThostFtdcQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTradingAccount)(self as *mut CThostFtdcTraderApi,
                                             p_qry_trading_account as * mut CThostFtdcQryTradingAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor(&mut self, p_qry_investor: &mut CThostFtdcQryInvestorField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestor)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor as * mut CThostFtdcQryInvestorField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_code(&mut self, p_qry_trading_code: &mut CThostFtdcQryTradingCodeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTradingCode)(self as *mut CThostFtdcTraderApi,
                                             p_qry_trading_code as * mut CThostFtdcQryTradingCodeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_instrument_margin_rate(&mut self, p_qry_instrument_margin_rate: &mut CThostFtdcQryInstrumentMarginRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrumentMarginRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_instrument_margin_rate as * mut CThostFtdcQryInstrumentMarginRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_instrument_commission_rate(&mut self, p_qry_instrument_commission_rate: &mut CThostFtdcQryInstrumentCommissionRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrumentCommissionRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_instrument_commission_rate as * mut CThostFtdcQryInstrumentCommissionRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange(&mut self, p_qry_exchange: &mut CThostFtdcQryExchangeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchange)(self as *mut CThostFtdcTraderApi,
                                             p_qry_exchange as * mut CThostFtdcQryExchangeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_product(&mut self, p_qry_product: &mut CThostFtdcQryProductField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryProduct)(self as *mut CThostFtdcTraderApi,
                                             p_qry_product as * mut CThostFtdcQryProductField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_instrument(&mut self, p_qry_instrument: &mut CThostFtdcQryInstrumentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrument)(self as *mut CThostFtdcTraderApi,
                                             p_qry_instrument as * mut CThostFtdcQryInstrumentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_depth_market_data(&mut self, p_qry_depth_market_data: &mut CThostFtdcQryDepthMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryDepthMarketData)(self as *mut CThostFtdcTraderApi,
                                             p_qry_depth_market_data as * mut CThostFtdcQryDepthMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trader_offer(&mut self, p_qry_trader_offer: &mut CThostFtdcQryTraderOfferField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTraderOffer)(self as *mut CThostFtdcTraderApi,
                                             p_qry_trader_offer as * mut CThostFtdcQryTraderOfferField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_settlement_info(&mut self, p_qry_settlement_info: &mut CThostFtdcQrySettlementInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySettlementInfo)(self as *mut CThostFtdcTraderApi,
                                             p_qry_settlement_info as * mut CThostFtdcQrySettlementInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_transfer_bank(&mut self, p_qry_transfer_bank: &mut CThostFtdcQryTransferBankField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTransferBank)(self as *mut CThostFtdcTraderApi,
                                             p_qry_transfer_bank as * mut CThostFtdcQryTransferBankField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_position_detail(&mut self, p_qry_investor_position_detail: &mut CThostFtdcQryInvestorPositionDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPositionDetail)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_position_detail as * mut CThostFtdcQryInvestorPositionDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_notice(&mut self, p_qry_notice: &mut CThostFtdcQryNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryNotice)(self as *mut CThostFtdcTraderApi,
                                             p_qry_notice as * mut CThostFtdcQryNoticeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_settlement_info_confirm(&mut self, p_qry_settlement_info_confirm: &mut CThostFtdcQrySettlementInfoConfirmField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySettlementInfoConfirm)(self as *mut CThostFtdcTraderApi,
                                             p_qry_settlement_info_confirm as * mut CThostFtdcQrySettlementInfoConfirmField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_position_combine_detail(&mut self, p_qry_investor_position_combine_detail: &mut CThostFtdcQryInvestorPositionCombineDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPositionCombineDetail)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_position_combine_detail as * mut CThostFtdcQryInvestorPositionCombineDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cfmmc_trading_account_key(&mut self, p_qry_cfmmc_trading_account_key: &mut CThostFtdcQryCFMMCTradingAccountKeyField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCFMMCTradingAccountKey)(self as *mut CThostFtdcTraderApi,
                                             p_qry_cfmmc_trading_account_key as * mut CThostFtdcQryCFMMCTradingAccountKeyField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_e_warrant_offset(&mut self, p_qry_e_warrant_offset: &mut CThostFtdcQryEWarrantOffsetField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryEWarrantOffset)(self as *mut CThostFtdcTraderApi,
                                             p_qry_e_warrant_offset as * mut CThostFtdcQryEWarrantOffsetField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_product_group_margin(&mut self, p_qry_investor_product_group_margin: &mut CThostFtdcQryInvestorProductGroupMarginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorProductGroupMargin)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_product_group_margin as * mut CThostFtdcQryInvestorProductGroupMarginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange_margin_rate(&mut self, p_qry_exchange_margin_rate: &mut CThostFtdcQryExchangeMarginRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchangeMarginRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_exchange_margin_rate as * mut CThostFtdcQryExchangeMarginRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange_margin_rate_adjust(&mut self, p_qry_exchange_margin_rate_adjust: &mut CThostFtdcQryExchangeMarginRateAdjustField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchangeMarginRateAdjust)(self as *mut CThostFtdcTraderApi,
                                             p_qry_exchange_margin_rate_adjust as * mut CThostFtdcQryExchangeMarginRateAdjustField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange_rate(&mut self, p_qry_exchange_rate: &mut CThostFtdcQryExchangeRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchangeRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_exchange_rate as * mut CThostFtdcQryExchangeRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_sec_agent_acid_map(&mut self, p_qry_sec_agent_acid_map: &mut CThostFtdcQrySecAgentACIDMapField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentACIDMap)(self as *mut CThostFtdcTraderApi,
                                             p_qry_sec_agent_acid_map as * mut CThostFtdcQrySecAgentACIDMapField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_product_exch_rate(&mut self, p_qry_product_exch_rate: &mut CThostFtdcQryProductExchRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryProductExchRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_product_exch_rate as * mut CThostFtdcQryProductExchRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_product_group(&mut self, p_qry_product_group: &mut CThostFtdcQryProductGroupField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryProductGroup)(self as *mut CThostFtdcTraderApi,
                                             p_qry_product_group as * mut CThostFtdcQryProductGroupField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_mm_instrument_commission_rate(&mut self, p_qry_mm_instrument_commission_rate: &mut CThostFtdcQryMMInstrumentCommissionRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryMMInstrumentCommissionRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_mm_instrument_commission_rate as * mut CThostFtdcQryMMInstrumentCommissionRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_mm_option_instr_comm_rate(&mut self, p_qry_mm_option_instr_comm_rate: &mut CThostFtdcQryMMOptionInstrCommRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryMMOptionInstrCommRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_mm_option_instr_comm_rate as * mut CThostFtdcQryMMOptionInstrCommRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_instrument_order_comm_rate(&mut self, p_qry_instrument_order_comm_rate: &mut CThostFtdcQryInstrumentOrderCommRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrumentOrderCommRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_instrument_order_comm_rate as * mut CThostFtdcQryInstrumentOrderCommRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_sec_agent_trading_account(&mut self, p_qry_trading_account: &mut CThostFtdcQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentTradingAccount)(self as *mut CThostFtdcTraderApi,
                                             p_qry_trading_account as * mut CThostFtdcQryTradingAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_sec_agent_check_mode(&mut self, p_qry_sec_agent_check_mode: &mut CThostFtdcQrySecAgentCheckModeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentCheckMode)(self as *mut CThostFtdcTraderApi,
                                             p_qry_sec_agent_check_mode as * mut CThostFtdcQrySecAgentCheckModeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_sec_agent_trade_info(&mut self, p_qry_sec_agent_trade_info: &mut CThostFtdcQrySecAgentTradeInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentTradeInfo)(self as *mut CThostFtdcTraderApi,
                                             p_qry_sec_agent_trade_info as * mut CThostFtdcQrySecAgentTradeInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_option_instr_trade_cost(&mut self, p_qry_option_instr_trade_cost: &mut CThostFtdcQryOptionInstrTradeCostField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOptionInstrTradeCost)(self as *mut CThostFtdcTraderApi,
                                             p_qry_option_instr_trade_cost as * mut CThostFtdcQryOptionInstrTradeCostField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_option_instr_comm_rate(&mut self, p_qry_option_instr_comm_rate: &mut CThostFtdcQryOptionInstrCommRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOptionInstrCommRate)(self as *mut CThostFtdcTraderApi,
                                             p_qry_option_instr_comm_rate as * mut CThostFtdcQryOptionInstrCommRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exec_order(&mut self, p_qry_exec_order: &mut CThostFtdcQryExecOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExecOrder)(self as *mut CThostFtdcTraderApi,
                                             p_qry_exec_order as * mut CThostFtdcQryExecOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_for_quote(&mut self, p_qry_for_quote: &mut CThostFtdcQryForQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryForQuote)(self as *mut CThostFtdcTraderApi,
                                             p_qry_for_quote as * mut CThostFtdcQryForQuoteField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_quote(&mut self, p_qry_quote: &mut CThostFtdcQryQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryQuote)(self as *mut CThostFtdcTraderApi,
                                             p_qry_quote as * mut CThostFtdcQryQuoteField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_option_self_close(&mut self, p_qry_option_self_close: &mut CThostFtdcQryOptionSelfCloseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOptionSelfClose)(self as *mut CThostFtdcTraderApi,
                                             p_qry_option_self_close as * mut CThostFtdcQryOptionSelfCloseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_invest_unit(&mut self, p_qry_invest_unit: &mut CThostFtdcQryInvestUnitField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestUnit)(self as *mut CThostFtdcTraderApi,
                                             p_qry_invest_unit as * mut CThostFtdcQryInvestUnitField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_instrument_guard(&mut self, p_qry_comb_instrument_guard: &mut CThostFtdcQryCombInstrumentGuardField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCombInstrumentGuard)(self as *mut CThostFtdcTraderApi,
                                             p_qry_comb_instrument_guard as * mut CThostFtdcQryCombInstrumentGuardField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_action(&mut self, p_qry_comb_action: &mut CThostFtdcQryCombActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCombAction)(self as *mut CThostFtdcTraderApi,
                                             p_qry_comb_action as * mut CThostFtdcQryCombActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_transfer_serial(&mut self, p_qry_transfer_serial: &mut CThostFtdcQryTransferSerialField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTransferSerial)(self as *mut CThostFtdcTraderApi,
                                             p_qry_transfer_serial as * mut CThostFtdcQryTransferSerialField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_accountregister(&mut self, p_qry_accountregister: &mut CThostFtdcQryAccountregisterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryAccountregister)(self as *mut CThostFtdcTraderApi,
                                             p_qry_accountregister as * mut CThostFtdcQryAccountregisterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_contract_bank(&mut self, p_qry_contract_bank: &mut CThostFtdcQryContractBankField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryContractBank)(self as *mut CThostFtdcTraderApi,
                                             p_qry_contract_bank as * mut CThostFtdcQryContractBankField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_parked_order(&mut self, p_qry_parked_order: &mut CThostFtdcQryParkedOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryParkedOrder)(self as *mut CThostFtdcTraderApi,
                                             p_qry_parked_order as * mut CThostFtdcQryParkedOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_parked_order_action(&mut self, p_qry_parked_order_action: &mut CThostFtdcQryParkedOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryParkedOrderAction)(self as *mut CThostFtdcTraderApi,
                                             p_qry_parked_order_action as * mut CThostFtdcQryParkedOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_notice(&mut self, p_qry_trading_notice: &mut CThostFtdcQryTradingNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTradingNotice)(self as *mut CThostFtdcTraderApi,
                                             p_qry_trading_notice as * mut CThostFtdcQryTradingNoticeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_broker_trading_params(&mut self, p_qry_broker_trading_params: &mut CThostFtdcQryBrokerTradingParamsField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryBrokerTradingParams)(self as *mut CThostFtdcTraderApi,
                                             p_qry_broker_trading_params as * mut CThostFtdcQryBrokerTradingParamsField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_broker_trading_algos(&mut self, p_qry_broker_trading_algos: &mut CThostFtdcQryBrokerTradingAlgosField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryBrokerTradingAlgos)(self as *mut CThostFtdcTraderApi,
                                             p_qry_broker_trading_algos as * mut CThostFtdcQryBrokerTradingAlgosField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_query_cfmmc_trading_account_token(&mut self, p_query_cfmmc_trading_account_token: &mut CThostFtdcQueryCFMMCTradingAccountTokenField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQueryCFMMCTradingAccountToken)(self as *mut CThostFtdcTraderApi,
                                             p_query_cfmmc_trading_account_token as * mut CThostFtdcQueryCFMMCTradingAccountTokenField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_from_bank_to_future_by_future(&mut self, p_req_transfer: &mut CThostFtdcReqTransferField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqFromBankToFutureByFuture)(self as *mut CThostFtdcTraderApi,
                                             p_req_transfer as * mut CThostFtdcReqTransferField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_from_future_to_bank_by_future(&mut self, p_req_transfer: &mut CThostFtdcReqTransferField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqFromFutureToBankByFuture)(self as *mut CThostFtdcTraderApi,
                                             p_req_transfer as * mut CThostFtdcReqTransferField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_query_bank_account_money_by_future(&mut self, p_req_query_account: &mut CThostFtdcReqQueryAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQueryBankAccountMoneyByFuture)(self as *mut CThostFtdcTraderApi,
                                             p_req_query_account as * mut CThostFtdcReqQueryAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_classified_instrument(&mut self, p_qry_classified_instrument: &mut CThostFtdcQryClassifiedInstrumentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryClassifiedInstrument)(self as *mut CThostFtdcTraderApi,
                                             p_qry_classified_instrument as * mut CThostFtdcQryClassifiedInstrumentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_promotion_param(&mut self, p_qry_comb_promotion_param: &mut CThostFtdcQryCombPromotionParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCombPromotionParam)(self as *mut CThostFtdcTraderApi,
                                             p_qry_comb_promotion_param as * mut CThostFtdcQryCombPromotionParamField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_risk_settle_invst_position(&mut self, p_qry_risk_settle_invst_position: &mut CThostFtdcQryRiskSettleInvstPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRiskSettleInvstPosition)(self as *mut CThostFtdcTraderApi,
                                             p_qry_risk_settle_invst_position as * mut CThostFtdcQryRiskSettleInvstPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_risk_settle_product_status(&mut self, p_qry_risk_settle_product_status: &mut CThostFtdcQryRiskSettleProductStatusField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRiskSettleProductStatus)(self as *mut CThostFtdcTraderApi,
                                             p_qry_risk_settle_product_status as * mut CThostFtdcQryRiskSettleProductStatusField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_future_parameter(&mut self, p_qry_spbm_future_parameter: &mut CThostFtdcQrySPBMFutureParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMFutureParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_future_parameter as * mut CThostFtdcQrySPBMFutureParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_option_parameter(&mut self, p_qry_spbm_option_parameter: &mut CThostFtdcQrySPBMOptionParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMOptionParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_option_parameter as * mut CThostFtdcQrySPBMOptionParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_intra_parameter(&mut self, p_qry_spbm_intra_parameter: &mut CThostFtdcQrySPBMIntraParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMIntraParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_intra_parameter as * mut CThostFtdcQrySPBMIntraParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_inter_parameter(&mut self, p_qry_spbm_inter_parameter: &mut CThostFtdcQrySPBMInterParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMInterParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_inter_parameter as * mut CThostFtdcQrySPBMInterParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_portf_definition(&mut self, p_qry_spbm_portf_definition: &mut CThostFtdcQrySPBMPortfDefinitionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMPortfDefinition)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_portf_definition as * mut CThostFtdcQrySPBMPortfDefinitionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_investor_portf_def(&mut self, p_qry_spbm_investor_portf_def: &mut CThostFtdcQrySPBMInvestorPortfDefField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMInvestorPortfDef)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_investor_portf_def as * mut CThostFtdcQrySPBMInvestorPortfDefField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_portf_margin_ratio(&mut self, p_qry_investor_portf_margin_ratio: &mut CThostFtdcQryInvestorPortfMarginRatioField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPortfMarginRatio)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_portf_margin_ratio as * mut CThostFtdcQryInvestorPortfMarginRatioField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_prod_spbm_detail(&mut self, p_qry_investor_prod_spbm_detail: &mut CThostFtdcQryInvestorProdSPBMDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorProdSPBMDetail)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_prod_spbm_detail as * mut CThostFtdcQryInvestorProdSPBMDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_commodity_spmm_margin(&mut self, p_qry_investor_commodity_spmm_margin: &mut CThostFtdcQryInvestorCommoditySPMMMarginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorCommoditySPMMMargin)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_commodity_spmm_margin as * mut CThostFtdcQryInvestorCommoditySPMMMarginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_commodity_group_spmm_margin(&mut self, p_qry_investor_commodity_group_spmm_margin: &mut CThostFtdcQryInvestorCommodityGroupSPMMMarginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorCommodityGroupSPMMMargin)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_commodity_group_spmm_margin as * mut CThostFtdcQryInvestorCommodityGroupSPMMMarginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spmm_inst_param(&mut self, p_qry_spmm_inst_param: &mut CThostFtdcQrySPMMInstParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPMMInstParam)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spmm_inst_param as * mut CThostFtdcQrySPMMInstParamField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spmm_product_param(&mut self, p_qry_spmm_product_param: &mut CThostFtdcQrySPMMProductParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPMMProductParam)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spmm_product_param as * mut CThostFtdcQrySPMMProductParamField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_spbm_add_on_inter_parameter(&mut self, p_qry_spbm_add_on_inter_parameter: &mut CThostFtdcQrySPBMAddOnInterParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMAddOnInterParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_spbm_add_on_inter_parameter as * mut CThostFtdcQrySPBMAddOnInterParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rcams_comb_product_info(&mut self, p_qry_rcams_comb_product_info: &mut CThostFtdcQryRCAMSCombProductInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRCAMSCombProductInfo)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rcams_comb_product_info as * mut CThostFtdcQryRCAMSCombProductInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rcams_instr_parameter(&mut self, p_qry_rcams_instr_parameter: &mut CThostFtdcQryRCAMSInstrParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRCAMSInstrParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rcams_instr_parameter as * mut CThostFtdcQryRCAMSInstrParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rcams_intra_parameter(&mut self, p_qry_rcams_intra_parameter: &mut CThostFtdcQryRCAMSIntraParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRCAMSIntraParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rcams_intra_parameter as * mut CThostFtdcQryRCAMSIntraParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rcams_inter_parameter(&mut self, p_qry_rcams_inter_parameter: &mut CThostFtdcQryRCAMSInterParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRCAMSInterParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rcams_inter_parameter as * mut CThostFtdcQryRCAMSInterParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rcams_short_opt_adjust_param(&mut self, p_qry_rcams_short_opt_adjust_param: &mut CThostFtdcQryRCAMSShortOptAdjustParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRCAMSShortOptAdjustParam)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rcams_short_opt_adjust_param as * mut CThostFtdcQryRCAMSShortOptAdjustParamField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rcams_investor_comb_position(&mut self, p_qry_rcams_investor_comb_position: &mut CThostFtdcQryRCAMSInvestorCombPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRCAMSInvestorCombPosition)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rcams_investor_comb_position as * mut CThostFtdcQryRCAMSInvestorCombPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_prod_rcams_margin(&mut self, p_qry_investor_prod_rcams_margin: &mut CThostFtdcQryInvestorProdRCAMSMarginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorProdRCAMSMargin)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_prod_rcams_margin as * mut CThostFtdcQryInvestorProdRCAMSMarginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rule_instr_parameter(&mut self, p_qry_rule_instr_parameter: &mut CThostFtdcQryRULEInstrParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRULEInstrParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rule_instr_parameter as * mut CThostFtdcQryRULEInstrParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rule_intra_parameter(&mut self, p_qry_rule_intra_parameter: &mut CThostFtdcQryRULEIntraParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRULEIntraParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rule_intra_parameter as * mut CThostFtdcQryRULEIntraParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rule_inter_parameter(&mut self, p_qry_rule_inter_parameter: &mut CThostFtdcQryRULEInterParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRULEInterParameter)(self as *mut CThostFtdcTraderApi,
                                             p_qry_rule_inter_parameter as * mut CThostFtdcQryRULEInterParameterField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_prod_rule_margin(&mut self, p_qry_investor_prod_rule_margin: &mut CThostFtdcQryInvestorProdRULEMarginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    unsafe {
                                           ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorProdRULEMargin)(self as *mut CThostFtdcTraderApi,
                                             p_qry_investor_prod_rule_margin as * mut CThostFtdcQryInvestorProdRULEMarginField,
                                             n_request_id)
                                        }
                            }} 
                unsafe impl Send for CThostFtdcTraderApi {}pub trait CThostFtdcTraderSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_heart_beat_warning(&mut self, n_time_lapse : std::os::raw::c_int) {}
fn on_rsp_authenticate(&mut self, p_rsp_authenticate_field : Option<&CThostFtdcRspAuthenticateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login : Option<&CThostFtdcRspUserLoginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_logout(&mut self, p_user_logout : Option<&CThostFtdcUserLogoutField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_password_update(&mut self, p_user_password_update : Option<&CThostFtdcUserPasswordUpdateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_trading_account_password_update(&mut self, p_trading_account_password_update : Option<&CThostFtdcTradingAccountPasswordUpdateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_auth_method(&mut self, p_rsp_user_auth_method : Option<&CThostFtdcRspUserAuthMethodField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_gen_user_captcha(&mut self, p_rsp_gen_user_captcha : Option<&CThostFtdcRspGenUserCaptchaField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_gen_user_text(&mut self, p_rsp_gen_user_text : Option<&CThostFtdcRspGenUserTextField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_order_insert(&mut self, p_input_order : Option<&CThostFtdcInputOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_parked_order_insert(&mut self, p_parked_order : Option<&CThostFtdcParkedOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_parked_order_action(&mut self, p_parked_order_action : Option<&CThostFtdcParkedOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_order_action(&mut self, p_input_order_action : Option<&CThostFtdcInputOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_max_order_volume(&mut self, p_qry_max_order_volume : Option<&CThostFtdcQryMaxOrderVolumeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_settlement_info_confirm(&mut self, p_settlement_info_confirm : Option<&CThostFtdcSettlementInfoConfirmField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_remove_parked_order(&mut self, p_remove_parked_order : Option<&CThostFtdcRemoveParkedOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_remove_parked_order_action(&mut self, p_remove_parked_order_action : Option<&CThostFtdcRemoveParkedOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_exec_order_insert(&mut self, p_input_exec_order : Option<&CThostFtdcInputExecOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_exec_order_action(&mut self, p_input_exec_order_action : Option<&CThostFtdcInputExecOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_for_quote_insert(&mut self, p_input_for_quote : Option<&CThostFtdcInputForQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_quote_insert(&mut self, p_input_quote : Option<&CThostFtdcInputQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_quote_action(&mut self, p_input_quote_action : Option<&CThostFtdcInputQuoteActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_batch_order_action(&mut self, p_input_batch_order_action : Option<&CThostFtdcInputBatchOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_option_self_close_insert(&mut self, p_input_option_self_close : Option<&CThostFtdcInputOptionSelfCloseField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_option_self_close_action(&mut self, p_input_option_self_close_action : Option<&CThostFtdcInputOptionSelfCloseActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_comb_action_insert(&mut self, p_input_comb_action : Option<&CThostFtdcInputCombActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order(&mut self, p_order : Option<&CThostFtdcOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trade(&mut self, p_trade : Option<&CThostFtdcTradeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_position(&mut self, p_investor_position : Option<&CThostFtdcInvestorPositionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_account(&mut self, p_trading_account : Option<&CThostFtdcTradingAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor(&mut self, p_investor : Option<&CThostFtdcInvestorField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_code(&mut self, p_trading_code : Option<&CThostFtdcTradingCodeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_instrument_margin_rate(&mut self, p_instrument_margin_rate : Option<&CThostFtdcInstrumentMarginRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_instrument_commission_rate(&mut self, p_instrument_commission_rate : Option<&CThostFtdcInstrumentCommissionRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exchange(&mut self, p_exchange : Option<&CThostFtdcExchangeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_product(&mut self, p_product : Option<&CThostFtdcProductField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_instrument(&mut self, p_instrument : Option<&CThostFtdcInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_depth_market_data(&mut self, p_depth_market_data : Option<&CThostFtdcDepthMarketDataField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trader_offer(&mut self, p_trader_offer : Option<&CThostFtdcTraderOfferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_settlement_info(&mut self, p_settlement_info : Option<&CThostFtdcSettlementInfoField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_transfer_bank(&mut self, p_transfer_bank : Option<&CThostFtdcTransferBankField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_position_detail(&mut self, p_investor_position_detail : Option<&CThostFtdcInvestorPositionDetailField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_notice(&mut self, p_notice : Option<&CThostFtdcNoticeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_settlement_info_confirm(&mut self, p_settlement_info_confirm : Option<&CThostFtdcSettlementInfoConfirmField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_position_combine_detail(&mut self, p_investor_position_combine_detail : Option<&CThostFtdcInvestorPositionCombineDetailField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cfmmc_trading_account_key(&mut self, p_cfmmc_trading_account_key : Option<&CThostFtdcCFMMCTradingAccountKeyField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_e_warrant_offset(&mut self, p_e_warrant_offset : Option<&CThostFtdcEWarrantOffsetField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_product_group_margin(&mut self, p_investor_product_group_margin : Option<&CThostFtdcInvestorProductGroupMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exchange_margin_rate(&mut self, p_exchange_margin_rate : Option<&CThostFtdcExchangeMarginRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, p_exchange_margin_rate_adjust : Option<&CThostFtdcExchangeMarginRateAdjustField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exchange_rate(&mut self, p_exchange_rate : Option<&CThostFtdcExchangeRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_sec_agent_acid_map(&mut self, p_sec_agent_acid_map : Option<&CThostFtdcSecAgentACIDMapField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_product_exch_rate(&mut self, p_product_exch_rate : Option<&CThostFtdcProductExchRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_product_group(&mut self, p_product_group : Option<&CThostFtdcProductGroupField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_mm_instrument_commission_rate(&mut self, p_mm_instrument_commission_rate : Option<&CThostFtdcMMInstrumentCommissionRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_mm_option_instr_comm_rate(&mut self, p_mm_option_instr_comm_rate : Option<&CThostFtdcMMOptionInstrCommRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_instrument_order_comm_rate(&mut self, p_instrument_order_comm_rate : Option<&CThostFtdcInstrumentOrderCommRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_sec_agent_trading_account(&mut self, p_trading_account : Option<&CThostFtdcTradingAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_sec_agent_check_mode(&mut self, p_sec_agent_check_mode : Option<&CThostFtdcSecAgentCheckModeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_sec_agent_trade_info(&mut self, p_sec_agent_trade_info : Option<&CThostFtdcSecAgentTradeInfoField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_option_instr_trade_cost(&mut self, p_option_instr_trade_cost : Option<&CThostFtdcOptionInstrTradeCostField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_option_instr_comm_rate(&mut self, p_option_instr_comm_rate : Option<&CThostFtdcOptionInstrCommRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exec_order(&mut self, p_exec_order : Option<&CThostFtdcExecOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_for_quote(&mut self, p_for_quote : Option<&CThostFtdcForQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_quote(&mut self, p_quote : Option<&CThostFtdcQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_option_self_close(&mut self, p_option_self_close : Option<&CThostFtdcOptionSelfCloseField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_invest_unit(&mut self, p_invest_unit : Option<&CThostFtdcInvestUnitField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_instrument_guard(&mut self, p_comb_instrument_guard : Option<&CThostFtdcCombInstrumentGuardField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_action(&mut self, p_comb_action : Option<&CThostFtdcCombActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_transfer_serial(&mut self, p_transfer_serial : Option<&CThostFtdcTransferSerialField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_accountregister(&mut self, p_accountregister : Option<&CThostFtdcAccountregisterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_error(&mut self, p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rtn_order(&mut self, p_order : Option<&CThostFtdcOrderField>) {}
fn on_rtn_trade(&mut self, p_trade : Option<&CThostFtdcTradeField>) {}
fn on_err_rtn_order_insert(&mut self, p_input_order : Option<&CThostFtdcInputOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_order_action(&mut self, p_order_action : Option<&CThostFtdcOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rtn_instrument_status(&mut self, p_instrument_status : Option<&CThostFtdcInstrumentStatusField>) {}
fn on_rtn_bulletin(&mut self, p_bulletin : Option<&CThostFtdcBulletinField>) {}
fn on_rtn_trading_notice(&mut self, p_trading_notice_info : Option<&CThostFtdcTradingNoticeInfoField>) {}
fn on_rtn_error_conditional_order(&mut self, p_error_conditional_order : Option<&CThostFtdcErrorConditionalOrderField>) {}
fn on_rtn_exec_order(&mut self, p_exec_order : Option<&CThostFtdcExecOrderField>) {}
fn on_err_rtn_exec_order_insert(&mut self, p_input_exec_order : Option<&CThostFtdcInputExecOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_exec_order_action(&mut self, p_exec_order_action : Option<&CThostFtdcExecOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_for_quote_insert(&mut self, p_input_for_quote : Option<&CThostFtdcInputForQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rtn_quote(&mut self, p_quote : Option<&CThostFtdcQuoteField>) {}
fn on_err_rtn_quote_insert(&mut self, p_input_quote : Option<&CThostFtdcInputQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_quote_action(&mut self, p_quote_action : Option<&CThostFtdcQuoteActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rtn_for_quote_rsp(&mut self, p_for_quote_rsp : Option<&CThostFtdcForQuoteRspField>) {}
fn on_rtn_cfmmc_trading_account_token(&mut self, p_cfmmc_trading_account_token : Option<&CThostFtdcCFMMCTradingAccountTokenField>) {}
fn on_err_rtn_batch_order_action(&mut self, p_batch_order_action : Option<&CThostFtdcBatchOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rtn_option_self_close(&mut self, p_option_self_close : Option<&CThostFtdcOptionSelfCloseField>) {}
fn on_err_rtn_option_self_close_insert(&mut self, p_input_option_self_close : Option<&CThostFtdcInputOptionSelfCloseField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_option_self_close_action(&mut self, p_option_self_close_action : Option<&CThostFtdcOptionSelfCloseActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rtn_comb_action(&mut self, p_comb_action : Option<&CThostFtdcCombActionField>) {}
fn on_err_rtn_comb_action_insert(&mut self, p_input_comb_action : Option<&CThostFtdcInputCombActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rsp_qry_contract_bank(&mut self, p_contract_bank : Option<&CThostFtdcContractBankField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_parked_order(&mut self, p_parked_order : Option<&CThostFtdcParkedOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_parked_order_action(&mut self, p_parked_order_action : Option<&CThostFtdcParkedOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_notice(&mut self, p_trading_notice : Option<&CThostFtdcTradingNoticeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_broker_trading_params(&mut self, p_broker_trading_params : Option<&CThostFtdcBrokerTradingParamsField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_broker_trading_algos(&mut self, p_broker_trading_algos : Option<&CThostFtdcBrokerTradingAlgosField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_query_cfmmc_trading_account_token(&mut self, p_query_cfmmc_trading_account_token : Option<&CThostFtdcQueryCFMMCTradingAccountTokenField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rtn_from_bank_to_future_by_bank(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) {}
fn on_rtn_from_future_to_bank_by_bank(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) {}
fn on_rtn_repeal_from_bank_to_future_by_bank(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) {}
fn on_rtn_repeal_from_future_to_bank_by_bank(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) {}
fn on_rtn_from_bank_to_future_by_future(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) {}
fn on_rtn_from_future_to_bank_by_future(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) {}
fn on_rtn_repeal_from_bank_to_future_by_future_manual(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) {}
fn on_rtn_repeal_from_future_to_bank_by_future_manual(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) {}
fn on_rtn_query_bank_balance_by_future(&mut self, p_notify_query_account : Option<&CThostFtdcNotifyQueryAccountField>) {}
fn on_err_rtn_bank_to_future_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_future_to_bank_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_repeal_bank_to_future_by_future_manual(&mut self, p_req_repeal : Option<&CThostFtdcReqRepealField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_repeal_future_to_bank_by_future_manual(&mut self, p_req_repeal : Option<&CThostFtdcReqRepealField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_err_rtn_query_bank_balance_by_future(&mut self, p_req_query_account : Option<&CThostFtdcReqQueryAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) {}
fn on_rtn_repeal_from_bank_to_future_by_future(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) {}
fn on_rtn_repeal_from_future_to_bank_by_future(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) {}
fn on_rsp_from_bank_to_future_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_from_future_to_bank_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_query_bank_account_money_by_future(&mut self, p_req_query_account : Option<&CThostFtdcReqQueryAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rtn_open_account_by_bank(&mut self, p_open_account : Option<&CThostFtdcOpenAccountField>) {}
fn on_rtn_cancel_account_by_bank(&mut self, p_cancel_account : Option<&CThostFtdcCancelAccountField>) {}
fn on_rtn_change_account_by_bank(&mut self, p_change_account : Option<&CThostFtdcChangeAccountField>) {}
fn on_rsp_qry_classified_instrument(&mut self, p_instrument : Option<&CThostFtdcInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_promotion_param(&mut self, p_comb_promotion_param : Option<&CThostFtdcCombPromotionParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_risk_settle_invst_position(&mut self, p_risk_settle_invst_position : Option<&CThostFtdcRiskSettleInvstPositionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_risk_settle_product_status(&mut self, p_risk_settle_product_status : Option<&CThostFtdcRiskSettleProductStatusField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_future_parameter(&mut self, p_spbm_future_parameter : Option<&CThostFtdcSPBMFutureParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_option_parameter(&mut self, p_spbm_option_parameter : Option<&CThostFtdcSPBMOptionParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_intra_parameter(&mut self, p_spbm_intra_parameter : Option<&CThostFtdcSPBMIntraParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_inter_parameter(&mut self, p_spbm_inter_parameter : Option<&CThostFtdcSPBMInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_portf_definition(&mut self, p_spbm_portf_definition : Option<&CThostFtdcSPBMPortfDefinitionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_investor_portf_def(&mut self, p_spbm_investor_portf_def : Option<&CThostFtdcSPBMInvestorPortfDefField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_portf_margin_ratio(&mut self, p_investor_portf_margin_ratio : Option<&CThostFtdcInvestorPortfMarginRatioField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_prod_spbm_detail(&mut self, p_investor_prod_spbm_detail : Option<&CThostFtdcInvestorProdSPBMDetailField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_commodity_spmm_margin(&mut self, p_investor_commodity_spmm_margin : Option<&CThostFtdcInvestorCommoditySPMMMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_commodity_group_spmm_margin(&mut self, p_investor_commodity_group_spmm_margin : Option<&CThostFtdcInvestorCommodityGroupSPMMMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spmm_inst_param(&mut self, p_spmm_inst_param : Option<&CThostFtdcSPMMInstParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spmm_product_param(&mut self, p_spmm_product_param : Option<&CThostFtdcSPMMProductParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_spbm_add_on_inter_parameter(&mut self, p_spbm_add_on_inter_parameter : Option<&CThostFtdcSPBMAddOnInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rcams_comb_product_info(&mut self, p_rcams_comb_product_info : Option<&CThostFtdcRCAMSCombProductInfoField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rcams_instr_parameter(&mut self, p_rcams_instr_parameter : Option<&CThostFtdcRCAMSInstrParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rcams_intra_parameter(&mut self, p_rcams_intra_parameter : Option<&CThostFtdcRCAMSIntraParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rcams_inter_parameter(&mut self, p_rcams_inter_parameter : Option<&CThostFtdcRCAMSInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rcams_short_opt_adjust_param(&mut self, p_rcams_short_opt_adjust_param : Option<&CThostFtdcRCAMSShortOptAdjustParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rcams_investor_comb_position(&mut self, p_rcams_investor_comb_position : Option<&CThostFtdcRCAMSInvestorCombPositionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_prod_rcams_margin(&mut self, p_investor_prod_rcams_margin : Option<&CThostFtdcInvestorProdRCAMSMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rule_instr_parameter(&mut self, p_rule_instr_parameter : Option<&CThostFtdcRULEInstrParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rule_intra_parameter(&mut self, p_rule_intra_parameter : Option<&CThostFtdcRULEIntraParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rule_inter_parameter(&mut self, p_rule_inter_parameter : Option<&CThostFtdcRULEInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_prod_rule_margin(&mut self, p_investor_prod_rule_margin : Option<&CThostFtdcInvestorProdRULEMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct CThostFtdcTraderSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, n_reason : std::os::raw::c_int ),
                on_heart_beat_warning: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, n_time_lapse : std::os::raw::c_int ),
                on_rsp_authenticate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_authenticate_field : * const CThostFtdcRspAuthenticateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_login: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_user_login : * const CThostFtdcRspUserLoginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_logout: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_user_logout : * const CThostFtdcUserLogoutField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_password_update: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_user_password_update : * const CThostFtdcUserPasswordUpdateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_trading_account_password_update: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trading_account_password_update : * const CThostFtdcTradingAccountPasswordUpdateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_auth_method: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_user_auth_method : * const CThostFtdcRspUserAuthMethodField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_gen_user_captcha: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_gen_user_captcha : * const CThostFtdcRspGenUserCaptchaField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_gen_user_text: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_gen_user_text : * const CThostFtdcRspGenUserTextField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_order : * const CThostFtdcInputOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_parked_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_parked_order : * const CThostFtdcParkedOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_parked_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_parked_order_action : * const CThostFtdcParkedOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_order_action : * const CThostFtdcInputOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_max_order_volume: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_qry_max_order_volume : * const CThostFtdcQryMaxOrderVolumeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_settlement_info_confirm: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_settlement_info_confirm : * const CThostFtdcSettlementInfoConfirmField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_remove_parked_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_remove_parked_order : * const CThostFtdcRemoveParkedOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_remove_parked_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_remove_parked_order_action : * const CThostFtdcRemoveParkedOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_exec_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_exec_order : * const CThostFtdcInputExecOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_exec_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_exec_order_action : * const CThostFtdcInputExecOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_for_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_for_quote : * const CThostFtdcInputForQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_quote : * const CThostFtdcInputQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_quote_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_quote_action : * const CThostFtdcInputQuoteActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_batch_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_batch_order_action : * const CThostFtdcInputBatchOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_option_self_close_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_option_self_close : * const CThostFtdcInputOptionSelfCloseField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_option_self_close_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_option_self_close_action : * const CThostFtdcInputOptionSelfCloseActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_comb_action_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_comb_action : * const CThostFtdcInputCombActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_order : * const CThostFtdcOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trade: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trade : * const CThostFtdcTradeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_position: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_position : * const CThostFtdcInvestorPositionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_account: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trading_account : * const CThostFtdcTradingAccountField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor : * const CThostFtdcInvestorField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_code: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trading_code : * const CThostFtdcTradingCodeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_instrument_margin_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_instrument_margin_rate : * const CThostFtdcInstrumentMarginRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_instrument_commission_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_instrument_commission_rate : * const CThostFtdcInstrumentCommissionRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exchange: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exchange : * const CThostFtdcExchangeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_product: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_product : * const CThostFtdcProductField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_instrument: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_instrument : * const CThostFtdcInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_depth_market_data: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_depth_market_data : * const CThostFtdcDepthMarketDataField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trader_offer: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trader_offer : * const CThostFtdcTraderOfferField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_settlement_info: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_settlement_info : * const CThostFtdcSettlementInfoField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_transfer_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_transfer_bank : * const CThostFtdcTransferBankField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_position_detail: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_position_detail : * const CThostFtdcInvestorPositionDetailField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_notice: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_notice : * const CThostFtdcNoticeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_settlement_info_confirm: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_settlement_info_confirm : * const CThostFtdcSettlementInfoConfirmField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_position_combine_detail: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_position_combine_detail : * const CThostFtdcInvestorPositionCombineDetailField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cfmmc_trading_account_key: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_cfmmc_trading_account_key : * const CThostFtdcCFMMCTradingAccountKeyField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_e_warrant_offset: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_e_warrant_offset : * const CThostFtdcEWarrantOffsetField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_product_group_margin: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_product_group_margin : * const CThostFtdcInvestorProductGroupMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exchange_margin_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exchange_margin_rate : * const CThostFtdcExchangeMarginRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exchange_margin_rate_adjust: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exchange_margin_rate_adjust : * const CThostFtdcExchangeMarginRateAdjustField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exchange_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exchange_rate : * const CThostFtdcExchangeRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_sec_agent_acid_map: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_sec_agent_acid_map : * const CThostFtdcSecAgentACIDMapField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_product_exch_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_product_exch_rate : * const CThostFtdcProductExchRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_product_group: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_product_group : * const CThostFtdcProductGroupField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_mm_instrument_commission_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_mm_instrument_commission_rate : * const CThostFtdcMMInstrumentCommissionRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_mm_option_instr_comm_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_mm_option_instr_comm_rate : * const CThostFtdcMMOptionInstrCommRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_instrument_order_comm_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_instrument_order_comm_rate : * const CThostFtdcInstrumentOrderCommRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_sec_agent_trading_account: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trading_account : * const CThostFtdcTradingAccountField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_sec_agent_check_mode: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_sec_agent_check_mode : * const CThostFtdcSecAgentCheckModeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_sec_agent_trade_info: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_sec_agent_trade_info : * const CThostFtdcSecAgentTradeInfoField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_option_instr_trade_cost: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_option_instr_trade_cost : * const CThostFtdcOptionInstrTradeCostField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_option_instr_comm_rate: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_option_instr_comm_rate : * const CThostFtdcOptionInstrCommRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exec_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exec_order : * const CThostFtdcExecOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_for_quote: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_for_quote : * const CThostFtdcForQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_quote: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_quote : * const CThostFtdcQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_option_self_close: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_option_self_close : * const CThostFtdcOptionSelfCloseField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_invest_unit: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_invest_unit : * const CThostFtdcInvestUnitField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_instrument_guard: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_comb_instrument_guard : * const CThostFtdcCombInstrumentGuardField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_comb_action : * const CThostFtdcCombActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_transfer_serial: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_transfer_serial : * const CThostFtdcTransferSerialField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_accountregister: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_accountregister : * const CThostFtdcAccountregisterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_error: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rtn_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_order : * const CThostFtdcOrderField ),
                on_rtn_trade: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trade : * const CThostFtdcTradeField ),
                on_err_rtn_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_order : * const CThostFtdcInputOrderField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_order_action : * const CThostFtdcOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rtn_instrument_status: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_instrument_status : * const CThostFtdcInstrumentStatusField ),
                on_rtn_bulletin: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_bulletin : * const CThostFtdcBulletinField ),
                on_rtn_trading_notice: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trading_notice_info : * const CThostFtdcTradingNoticeInfoField ),
                on_rtn_error_conditional_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_error_conditional_order : * const CThostFtdcErrorConditionalOrderField ),
                on_rtn_exec_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exec_order : * const CThostFtdcExecOrderField ),
                on_err_rtn_exec_order_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_exec_order : * const CThostFtdcInputExecOrderField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_exec_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_exec_order_action : * const CThostFtdcExecOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_for_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_for_quote : * const CThostFtdcInputForQuoteField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rtn_quote: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_quote : * const CThostFtdcQuoteField ),
                on_err_rtn_quote_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_quote : * const CThostFtdcInputQuoteField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_quote_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_quote_action : * const CThostFtdcQuoteActionField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rtn_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_for_quote_rsp : * const CThostFtdcForQuoteRspField ),
                on_rtn_cfmmc_trading_account_token: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_cfmmc_trading_account_token : * const CThostFtdcCFMMCTradingAccountTokenField ),
                on_err_rtn_batch_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_batch_order_action : * const CThostFtdcBatchOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rtn_option_self_close: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_option_self_close : * const CThostFtdcOptionSelfCloseField ),
                on_err_rtn_option_self_close_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_option_self_close : * const CThostFtdcInputOptionSelfCloseField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_option_self_close_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_option_self_close_action : * const CThostFtdcOptionSelfCloseActionField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rtn_comb_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_comb_action : * const CThostFtdcCombActionField ),
                on_err_rtn_comb_action_insert: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_input_comb_action : * const CThostFtdcInputCombActionField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rsp_qry_contract_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_contract_bank : * const CThostFtdcContractBankField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_parked_order: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_parked_order : * const CThostFtdcParkedOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_parked_order_action: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_parked_order_action : * const CThostFtdcParkedOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_notice: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_trading_notice : * const CThostFtdcTradingNoticeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_broker_trading_params: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_broker_trading_params : * const CThostFtdcBrokerTradingParamsField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_broker_trading_algos: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_broker_trading_algos : * const CThostFtdcBrokerTradingAlgosField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_query_cfmmc_trading_account_token: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_query_cfmmc_trading_account_token : * const CThostFtdcQueryCFMMCTradingAccountTokenField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rtn_from_bank_to_future_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField ),
                on_rtn_from_future_to_bank_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField ),
                on_rtn_repeal_from_bank_to_future_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField ),
                on_rtn_repeal_from_future_to_bank_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField ),
                on_rtn_from_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField ),
                on_rtn_from_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField ),
                on_rtn_repeal_from_bank_to_future_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField ),
                on_rtn_repeal_from_future_to_bank_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField ),
                on_rtn_query_bank_balance_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_notify_query_account : * const CThostFtdcNotifyQueryAccountField ),
                on_err_rtn_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_repeal_bank_to_future_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_repeal : * const CThostFtdcReqRepealField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_repeal_future_to_bank_by_future_manual: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_repeal : * const CThostFtdcReqRepealField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_err_rtn_query_bank_balance_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_query_account : * const CThostFtdcReqQueryAccountField,p_rsp_info : * const CThostFtdcRspInfoField ),
                on_rtn_repeal_from_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField ),
                on_rtn_repeal_from_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField ),
                on_rsp_from_bank_to_future_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_from_future_to_bank_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_query_bank_account_money_by_future: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_req_query_account : * const CThostFtdcReqQueryAccountField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rtn_open_account_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_open_account : * const CThostFtdcOpenAccountField ),
                on_rtn_cancel_account_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_cancel_account : * const CThostFtdcCancelAccountField ),
                on_rtn_change_account_by_bank: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_change_account : * const CThostFtdcChangeAccountField ),
                on_rsp_qry_classified_instrument: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_instrument : * const CThostFtdcInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_promotion_param: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_comb_promotion_param : * const CThostFtdcCombPromotionParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_risk_settle_invst_position: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_risk_settle_invst_position : * const CThostFtdcRiskSettleInvstPositionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_risk_settle_product_status: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_risk_settle_product_status : * const CThostFtdcRiskSettleProductStatusField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_future_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_future_parameter : * const CThostFtdcSPBMFutureParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_option_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_option_parameter : * const CThostFtdcSPBMOptionParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_intra_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_intra_parameter : * const CThostFtdcSPBMIntraParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_inter_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_inter_parameter : * const CThostFtdcSPBMInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_portf_definition: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_portf_definition : * const CThostFtdcSPBMPortfDefinitionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_investor_portf_def: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_investor_portf_def : * const CThostFtdcSPBMInvestorPortfDefField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_portf_margin_ratio: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_portf_margin_ratio : * const CThostFtdcInvestorPortfMarginRatioField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_prod_spbm_detail: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_prod_spbm_detail : * const CThostFtdcInvestorProdSPBMDetailField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_commodity_spmm_margin: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_commodity_spmm_margin : * const CThostFtdcInvestorCommoditySPMMMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_commodity_group_spmm_margin: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_commodity_group_spmm_margin : * const CThostFtdcInvestorCommodityGroupSPMMMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spmm_inst_param: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spmm_inst_param : * const CThostFtdcSPMMInstParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spmm_product_param: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spmm_product_param : * const CThostFtdcSPMMProductParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_spbm_add_on_inter_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_spbm_add_on_inter_parameter : * const CThostFtdcSPBMAddOnInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rcams_comb_product_info: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rcams_comb_product_info : * const CThostFtdcRCAMSCombProductInfoField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rcams_instr_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rcams_instr_parameter : * const CThostFtdcRCAMSInstrParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rcams_intra_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rcams_intra_parameter : * const CThostFtdcRCAMSIntraParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rcams_inter_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rcams_inter_parameter : * const CThostFtdcRCAMSInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rcams_short_opt_adjust_param: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rcams_short_opt_adjust_param : * const CThostFtdcRCAMSShortOptAdjustParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rcams_investor_comb_position: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rcams_investor_comb_position : * const CThostFtdcRCAMSInvestorCombPositionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_prod_rcams_margin: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_prod_rcams_margin : * const CThostFtdcInvestorProdRCAMSMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rule_instr_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rule_instr_parameter : * const CThostFtdcRULEInstrParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rule_intra_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rule_intra_parameter : * const CThostFtdcRULEIntraParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rule_inter_parameter: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_rule_inter_parameter : * const CThostFtdcRULEInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_prod_rule_margin: extern "C" fn(spi: *mut CThostFtdcTraderSpiFat, p_investor_prod_rule_margin : * const CThostFtdcInvestorProdRULEMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                 } 

        #[derive(Clone, Debug, Decode, Encode)]
        pub enum CThostFtdcTraderSpiOutput {OnFrontConnected(CThostFtdcTraderSpiOnFrontConnectedPacket),OnFrontDisconnected(CThostFtdcTraderSpiOnFrontDisconnectedPacket),OnHeartBeatWarning(CThostFtdcTraderSpiOnHeartBeatWarningPacket),OnRspAuthenticate(CThostFtdcTraderSpiOnRspAuthenticatePacket),OnRspUserLogin(CThostFtdcTraderSpiOnRspUserLoginPacket),OnRspUserLogout(CThostFtdcTraderSpiOnRspUserLogoutPacket),OnRspUserPasswordUpdate(CThostFtdcTraderSpiOnRspUserPasswordUpdatePacket),OnRspTradingAccountPasswordUpdate(CThostFtdcTraderSpiOnRspTradingAccountPasswordUpdatePacket),OnRspUserAuthMethod(CThostFtdcTraderSpiOnRspUserAuthMethodPacket),OnRspGenUserCaptcha(CThostFtdcTraderSpiOnRspGenUserCaptchaPacket),OnRspGenUserText(CThostFtdcTraderSpiOnRspGenUserTextPacket),OnRspOrderInsert(CThostFtdcTraderSpiOnRspOrderInsertPacket),OnRspParkedOrderInsert(CThostFtdcTraderSpiOnRspParkedOrderInsertPacket),OnRspParkedOrderAction(CThostFtdcTraderSpiOnRspParkedOrderActionPacket),OnRspOrderAction(CThostFtdcTraderSpiOnRspOrderActionPacket),OnRspQryMaxOrderVolume(CThostFtdcTraderSpiOnRspQryMaxOrderVolumePacket),OnRspSettlementInfoConfirm(CThostFtdcTraderSpiOnRspSettlementInfoConfirmPacket),OnRspRemoveParkedOrder(CThostFtdcTraderSpiOnRspRemoveParkedOrderPacket),OnRspRemoveParkedOrderAction(CThostFtdcTraderSpiOnRspRemoveParkedOrderActionPacket),OnRspExecOrderInsert(CThostFtdcTraderSpiOnRspExecOrderInsertPacket),OnRspExecOrderAction(CThostFtdcTraderSpiOnRspExecOrderActionPacket),OnRspForQuoteInsert(CThostFtdcTraderSpiOnRspForQuoteInsertPacket),OnRspQuoteInsert(CThostFtdcTraderSpiOnRspQuoteInsertPacket),OnRspQuoteAction(CThostFtdcTraderSpiOnRspQuoteActionPacket),OnRspBatchOrderAction(CThostFtdcTraderSpiOnRspBatchOrderActionPacket),OnRspOptionSelfCloseInsert(CThostFtdcTraderSpiOnRspOptionSelfCloseInsertPacket),OnRspOptionSelfCloseAction(CThostFtdcTraderSpiOnRspOptionSelfCloseActionPacket),OnRspCombActionInsert(CThostFtdcTraderSpiOnRspCombActionInsertPacket),OnRspQryOrder(CThostFtdcTraderSpiOnRspQryOrderPacket),OnRspQryTrade(CThostFtdcTraderSpiOnRspQryTradePacket),OnRspQryInvestorPosition(CThostFtdcTraderSpiOnRspQryInvestorPositionPacket),OnRspQryTradingAccount(CThostFtdcTraderSpiOnRspQryTradingAccountPacket),OnRspQryInvestor(CThostFtdcTraderSpiOnRspQryInvestorPacket),OnRspQryTradingCode(CThostFtdcTraderSpiOnRspQryTradingCodePacket),OnRspQryInstrumentMarginRate(CThostFtdcTraderSpiOnRspQryInstrumentMarginRatePacket),OnRspQryInstrumentCommissionRate(CThostFtdcTraderSpiOnRspQryInstrumentCommissionRatePacket),OnRspQryExchange(CThostFtdcTraderSpiOnRspQryExchangePacket),OnRspQryProduct(CThostFtdcTraderSpiOnRspQryProductPacket),OnRspQryInstrument(CThostFtdcTraderSpiOnRspQryInstrumentPacket),OnRspQryDepthMarketData(CThostFtdcTraderSpiOnRspQryDepthMarketDataPacket),OnRspQryTraderOffer(CThostFtdcTraderSpiOnRspQryTraderOfferPacket),OnRspQrySettlementInfo(CThostFtdcTraderSpiOnRspQrySettlementInfoPacket),OnRspQryTransferBank(CThostFtdcTraderSpiOnRspQryTransferBankPacket),OnRspQryInvestorPositionDetail(CThostFtdcTraderSpiOnRspQryInvestorPositionDetailPacket),OnRspQryNotice(CThostFtdcTraderSpiOnRspQryNoticePacket),OnRspQrySettlementInfoConfirm(CThostFtdcTraderSpiOnRspQrySettlementInfoConfirmPacket),OnRspQryInvestorPositionCombineDetail(CThostFtdcTraderSpiOnRspQryInvestorPositionCombineDetailPacket),OnRspQryCFMMCTradingAccountKey(CThostFtdcTraderSpiOnRspQryCFMMCTradingAccountKeyPacket),OnRspQryEWarrantOffset(CThostFtdcTraderSpiOnRspQryEWarrantOffsetPacket),OnRspQryInvestorProductGroupMargin(CThostFtdcTraderSpiOnRspQryInvestorProductGroupMarginPacket),OnRspQryExchangeMarginRate(CThostFtdcTraderSpiOnRspQryExchangeMarginRatePacket),OnRspQryExchangeMarginRateAdjust(CThostFtdcTraderSpiOnRspQryExchangeMarginRateAdjustPacket),OnRspQryExchangeRate(CThostFtdcTraderSpiOnRspQryExchangeRatePacket),OnRspQrySecAgentACIDMap(CThostFtdcTraderSpiOnRspQrySecAgentACIDMapPacket),OnRspQryProductExchRate(CThostFtdcTraderSpiOnRspQryProductExchRatePacket),OnRspQryProductGroup(CThostFtdcTraderSpiOnRspQryProductGroupPacket),OnRspQryMMInstrumentCommissionRate(CThostFtdcTraderSpiOnRspQryMMInstrumentCommissionRatePacket),OnRspQryMMOptionInstrCommRate(CThostFtdcTraderSpiOnRspQryMMOptionInstrCommRatePacket),OnRspQryInstrumentOrderCommRate(CThostFtdcTraderSpiOnRspQryInstrumentOrderCommRatePacket),OnRspQrySecAgentTradingAccount(CThostFtdcTraderSpiOnRspQrySecAgentTradingAccountPacket),OnRspQrySecAgentCheckMode(CThostFtdcTraderSpiOnRspQrySecAgentCheckModePacket),OnRspQrySecAgentTradeInfo(CThostFtdcTraderSpiOnRspQrySecAgentTradeInfoPacket),OnRspQryOptionInstrTradeCost(CThostFtdcTraderSpiOnRspQryOptionInstrTradeCostPacket),OnRspQryOptionInstrCommRate(CThostFtdcTraderSpiOnRspQryOptionInstrCommRatePacket),OnRspQryExecOrder(CThostFtdcTraderSpiOnRspQryExecOrderPacket),OnRspQryForQuote(CThostFtdcTraderSpiOnRspQryForQuotePacket),OnRspQryQuote(CThostFtdcTraderSpiOnRspQryQuotePacket),OnRspQryOptionSelfClose(CThostFtdcTraderSpiOnRspQryOptionSelfClosePacket),OnRspQryInvestUnit(CThostFtdcTraderSpiOnRspQryInvestUnitPacket),OnRspQryCombInstrumentGuard(CThostFtdcTraderSpiOnRspQryCombInstrumentGuardPacket),OnRspQryCombAction(CThostFtdcTraderSpiOnRspQryCombActionPacket),OnRspQryTransferSerial(CThostFtdcTraderSpiOnRspQryTransferSerialPacket),OnRspQryAccountregister(CThostFtdcTraderSpiOnRspQryAccountregisterPacket),OnRspError(CThostFtdcTraderSpiOnRspErrorPacket),OnRtnOrder(CThostFtdcTraderSpiOnRtnOrderPacket),OnRtnTrade(CThostFtdcTraderSpiOnRtnTradePacket),OnErrRtnOrderInsert(CThostFtdcTraderSpiOnErrRtnOrderInsertPacket),OnErrRtnOrderAction(CThostFtdcTraderSpiOnErrRtnOrderActionPacket),OnRtnInstrumentStatus(CThostFtdcTraderSpiOnRtnInstrumentStatusPacket),OnRtnBulletin(CThostFtdcTraderSpiOnRtnBulletinPacket),OnRtnTradingNotice(CThostFtdcTraderSpiOnRtnTradingNoticePacket),OnRtnErrorConditionalOrder(CThostFtdcTraderSpiOnRtnErrorConditionalOrderPacket),OnRtnExecOrder(CThostFtdcTraderSpiOnRtnExecOrderPacket),OnErrRtnExecOrderInsert(CThostFtdcTraderSpiOnErrRtnExecOrderInsertPacket),OnErrRtnExecOrderAction(CThostFtdcTraderSpiOnErrRtnExecOrderActionPacket),OnErrRtnForQuoteInsert(CThostFtdcTraderSpiOnErrRtnForQuoteInsertPacket),OnRtnQuote(CThostFtdcTraderSpiOnRtnQuotePacket),OnErrRtnQuoteInsert(CThostFtdcTraderSpiOnErrRtnQuoteInsertPacket),OnErrRtnQuoteAction(CThostFtdcTraderSpiOnErrRtnQuoteActionPacket),OnRtnForQuoteRsp(CThostFtdcTraderSpiOnRtnForQuoteRspPacket),OnRtnCFMMCTradingAccountToken(CThostFtdcTraderSpiOnRtnCFMMCTradingAccountTokenPacket),OnErrRtnBatchOrderAction(CThostFtdcTraderSpiOnErrRtnBatchOrderActionPacket),OnRtnOptionSelfClose(CThostFtdcTraderSpiOnRtnOptionSelfClosePacket),OnErrRtnOptionSelfCloseInsert(CThostFtdcTraderSpiOnErrRtnOptionSelfCloseInsertPacket),OnErrRtnOptionSelfCloseAction(CThostFtdcTraderSpiOnErrRtnOptionSelfCloseActionPacket),OnRtnCombAction(CThostFtdcTraderSpiOnRtnCombActionPacket),OnErrRtnCombActionInsert(CThostFtdcTraderSpiOnErrRtnCombActionInsertPacket),OnRspQryContractBank(CThostFtdcTraderSpiOnRspQryContractBankPacket),OnRspQryParkedOrder(CThostFtdcTraderSpiOnRspQryParkedOrderPacket),OnRspQryParkedOrderAction(CThostFtdcTraderSpiOnRspQryParkedOrderActionPacket),OnRspQryTradingNotice(CThostFtdcTraderSpiOnRspQryTradingNoticePacket),OnRspQryBrokerTradingParams(CThostFtdcTraderSpiOnRspQryBrokerTradingParamsPacket),OnRspQryBrokerTradingAlgos(CThostFtdcTraderSpiOnRspQryBrokerTradingAlgosPacket),OnRspQueryCFMMCTradingAccountToken(CThostFtdcTraderSpiOnRspQueryCFMMCTradingAccountTokenPacket),OnRtnFromBankToFutureByBank(CThostFtdcTraderSpiOnRtnFromBankToFutureByBankPacket),OnRtnFromFutureToBankByBank(CThostFtdcTraderSpiOnRtnFromFutureToBankByBankPacket),OnRtnRepealFromBankToFutureByBank(CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByBankPacket),OnRtnRepealFromFutureToBankByBank(CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByBankPacket),OnRtnFromBankToFutureByFuture(CThostFtdcTraderSpiOnRtnFromBankToFutureByFuturePacket),OnRtnFromFutureToBankByFuture(CThostFtdcTraderSpiOnRtnFromFutureToBankByFuturePacket),OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByFutureManualPacket),OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByFutureManualPacket),OnRtnQueryBankBalanceByFuture(CThostFtdcTraderSpiOnRtnQueryBankBalanceByFuturePacket),OnErrRtnBankToFutureByFuture(CThostFtdcTraderSpiOnErrRtnBankToFutureByFuturePacket),OnErrRtnFutureToBankByFuture(CThostFtdcTraderSpiOnErrRtnFutureToBankByFuturePacket),OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcTraderSpiOnErrRtnRepealBankToFutureByFutureManualPacket),OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcTraderSpiOnErrRtnRepealFutureToBankByFutureManualPacket),OnErrRtnQueryBankBalanceByFuture(CThostFtdcTraderSpiOnErrRtnQueryBankBalanceByFuturePacket),OnRtnRepealFromBankToFutureByFuture(CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByFuturePacket),OnRtnRepealFromFutureToBankByFuture(CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByFuturePacket),OnRspFromBankToFutureByFuture(CThostFtdcTraderSpiOnRspFromBankToFutureByFuturePacket),OnRspFromFutureToBankByFuture(CThostFtdcTraderSpiOnRspFromFutureToBankByFuturePacket),OnRspQueryBankAccountMoneyByFuture(CThostFtdcTraderSpiOnRspQueryBankAccountMoneyByFuturePacket),OnRtnOpenAccountByBank(CThostFtdcTraderSpiOnRtnOpenAccountByBankPacket),OnRtnCancelAccountByBank(CThostFtdcTraderSpiOnRtnCancelAccountByBankPacket),OnRtnChangeAccountByBank(CThostFtdcTraderSpiOnRtnChangeAccountByBankPacket),OnRspQryClassifiedInstrument(CThostFtdcTraderSpiOnRspQryClassifiedInstrumentPacket),OnRspQryCombPromotionParam(CThostFtdcTraderSpiOnRspQryCombPromotionParamPacket),OnRspQryRiskSettleInvstPosition(CThostFtdcTraderSpiOnRspQryRiskSettleInvstPositionPacket),OnRspQryRiskSettleProductStatus(CThostFtdcTraderSpiOnRspQryRiskSettleProductStatusPacket),OnRspQrySPBMFutureParameter(CThostFtdcTraderSpiOnRspQrySPBMFutureParameterPacket),OnRspQrySPBMOptionParameter(CThostFtdcTraderSpiOnRspQrySPBMOptionParameterPacket),OnRspQrySPBMIntraParameter(CThostFtdcTraderSpiOnRspQrySPBMIntraParameterPacket),OnRspQrySPBMInterParameter(CThostFtdcTraderSpiOnRspQrySPBMInterParameterPacket),OnRspQrySPBMPortfDefinition(CThostFtdcTraderSpiOnRspQrySPBMPortfDefinitionPacket),OnRspQrySPBMInvestorPortfDef(CThostFtdcTraderSpiOnRspQrySPBMInvestorPortfDefPacket),OnRspQryInvestorPortfMarginRatio(CThostFtdcTraderSpiOnRspQryInvestorPortfMarginRatioPacket),OnRspQryInvestorProdSPBMDetail(CThostFtdcTraderSpiOnRspQryInvestorProdSPBMDetailPacket),OnRspQryInvestorCommoditySPMMMargin(CThostFtdcTraderSpiOnRspQryInvestorCommoditySPMMMarginPacket),OnRspQryInvestorCommodityGroupSPMMMargin(CThostFtdcTraderSpiOnRspQryInvestorCommodityGroupSPMMMarginPacket),OnRspQrySPMMInstParam(CThostFtdcTraderSpiOnRspQrySPMMInstParamPacket),OnRspQrySPMMProductParam(CThostFtdcTraderSpiOnRspQrySPMMProductParamPacket),OnRspQrySPBMAddOnInterParameter(CThostFtdcTraderSpiOnRspQrySPBMAddOnInterParameterPacket),OnRspQryRCAMSCombProductInfo(CThostFtdcTraderSpiOnRspQryRCAMSCombProductInfoPacket),OnRspQryRCAMSInstrParameter(CThostFtdcTraderSpiOnRspQryRCAMSInstrParameterPacket),OnRspQryRCAMSIntraParameter(CThostFtdcTraderSpiOnRspQryRCAMSIntraParameterPacket),OnRspQryRCAMSInterParameter(CThostFtdcTraderSpiOnRspQryRCAMSInterParameterPacket),OnRspQryRCAMSShortOptAdjustParam(CThostFtdcTraderSpiOnRspQryRCAMSShortOptAdjustParamPacket),OnRspQryRCAMSInvestorCombPosition(CThostFtdcTraderSpiOnRspQryRCAMSInvestorCombPositionPacket),OnRspQryInvestorProdRCAMSMargin(CThostFtdcTraderSpiOnRspQryInvestorProdRCAMSMarginPacket),OnRspQryRULEInstrParameter(CThostFtdcTraderSpiOnRspQryRULEInstrParameterPacket),OnRspQryRULEIntraParameter(CThostFtdcTraderSpiOnRspQryRULEIntraParameterPacket),OnRspQryRULEInterParameter(CThostFtdcTraderSpiOnRspQryRULEInterParameterPacket),OnRspQryInvestorProdRULEMargin(CThostFtdcTraderSpiOnRspQryInvestorProdRULEMarginPacket), } 

            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnHeartBeatWarningPacket {
                pub n_time_lapse : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspAuthenticatePacket {
                pub p_rsp_authenticate_field : Option<CThostFtdcRspAuthenticateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspUserLoginPacket {
                pub p_rsp_user_login : Option<CThostFtdcRspUserLoginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspUserLogoutPacket {
                pub p_user_logout : Option<CThostFtdcUserLogoutField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspUserPasswordUpdatePacket {
                pub p_user_password_update : Option<CThostFtdcUserPasswordUpdateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspTradingAccountPasswordUpdatePacket {
                pub p_trading_account_password_update : Option<CThostFtdcTradingAccountPasswordUpdateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspUserAuthMethodPacket {
                pub p_rsp_user_auth_method : Option<CThostFtdcRspUserAuthMethodField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspGenUserCaptchaPacket {
                pub p_rsp_gen_user_captcha : Option<CThostFtdcRspGenUserCaptchaField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspGenUserTextPacket {
                pub p_rsp_gen_user_text : Option<CThostFtdcRspGenUserTextField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspOrderInsertPacket {
                pub p_input_order : Option<CThostFtdcInputOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspParkedOrderInsertPacket {
                pub p_parked_order : Option<CThostFtdcParkedOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspParkedOrderActionPacket {
                pub p_parked_order_action : Option<CThostFtdcParkedOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspOrderActionPacket {
                pub p_input_order_action : Option<CThostFtdcInputOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryMaxOrderVolumePacket {
                pub p_qry_max_order_volume : Option<CThostFtdcQryMaxOrderVolumeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspSettlementInfoConfirmPacket {
                pub p_settlement_info_confirm : Option<CThostFtdcSettlementInfoConfirmField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspRemoveParkedOrderPacket {
                pub p_remove_parked_order : Option<CThostFtdcRemoveParkedOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspRemoveParkedOrderActionPacket {
                pub p_remove_parked_order_action : Option<CThostFtdcRemoveParkedOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspExecOrderInsertPacket {
                pub p_input_exec_order : Option<CThostFtdcInputExecOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspExecOrderActionPacket {
                pub p_input_exec_order_action : Option<CThostFtdcInputExecOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspForQuoteInsertPacket {
                pub p_input_for_quote : Option<CThostFtdcInputForQuoteField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQuoteInsertPacket {
                pub p_input_quote : Option<CThostFtdcInputQuoteField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQuoteActionPacket {
                pub p_input_quote_action : Option<CThostFtdcInputQuoteActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspBatchOrderActionPacket {
                pub p_input_batch_order_action : Option<CThostFtdcInputBatchOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspOptionSelfCloseInsertPacket {
                pub p_input_option_self_close : Option<CThostFtdcInputOptionSelfCloseField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspOptionSelfCloseActionPacket {
                pub p_input_option_self_close_action : Option<CThostFtdcInputOptionSelfCloseActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspCombActionInsertPacket {
                pub p_input_comb_action : Option<CThostFtdcInputCombActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryOrderPacket {
                pub p_order : Option<CThostFtdcOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTradePacket {
                pub p_trade : Option<CThostFtdcTradeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorPositionPacket {
                pub p_investor_position : Option<CThostFtdcInvestorPositionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTradingAccountPacket {
                pub p_trading_account : Option<CThostFtdcTradingAccountField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorPacket {
                pub p_investor : Option<CThostFtdcInvestorField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTradingCodePacket {
                pub p_trading_code : Option<CThostFtdcTradingCodeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInstrumentMarginRatePacket {
                pub p_instrument_margin_rate : Option<CThostFtdcInstrumentMarginRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInstrumentCommissionRatePacket {
                pub p_instrument_commission_rate : Option<CThostFtdcInstrumentCommissionRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryExchangePacket {
                pub p_exchange : Option<CThostFtdcExchangeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryProductPacket {
                pub p_product : Option<CThostFtdcProductField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInstrumentPacket {
                pub p_instrument : Option<CThostFtdcInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryDepthMarketDataPacket {
                pub p_depth_market_data : Option<CThostFtdcDepthMarketDataField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTraderOfferPacket {
                pub p_trader_offer : Option<CThostFtdcTraderOfferField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySettlementInfoPacket {
                pub p_settlement_info : Option<CThostFtdcSettlementInfoField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTransferBankPacket {
                pub p_transfer_bank : Option<CThostFtdcTransferBankField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorPositionDetailPacket {
                pub p_investor_position_detail : Option<CThostFtdcInvestorPositionDetailField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryNoticePacket {
                pub p_notice : Option<CThostFtdcNoticeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySettlementInfoConfirmPacket {
                pub p_settlement_info_confirm : Option<CThostFtdcSettlementInfoConfirmField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorPositionCombineDetailPacket {
                pub p_investor_position_combine_detail : Option<CThostFtdcInvestorPositionCombineDetailField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryCFMMCTradingAccountKeyPacket {
                pub p_cfmmc_trading_account_key : Option<CThostFtdcCFMMCTradingAccountKeyField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryEWarrantOffsetPacket {
                pub p_e_warrant_offset : Option<CThostFtdcEWarrantOffsetField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorProductGroupMarginPacket {
                pub p_investor_product_group_margin : Option<CThostFtdcInvestorProductGroupMarginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryExchangeMarginRatePacket {
                pub p_exchange_margin_rate : Option<CThostFtdcExchangeMarginRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryExchangeMarginRateAdjustPacket {
                pub p_exchange_margin_rate_adjust : Option<CThostFtdcExchangeMarginRateAdjustField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryExchangeRatePacket {
                pub p_exchange_rate : Option<CThostFtdcExchangeRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySecAgentACIDMapPacket {
                pub p_sec_agent_acid_map : Option<CThostFtdcSecAgentACIDMapField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryProductExchRatePacket {
                pub p_product_exch_rate : Option<CThostFtdcProductExchRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryProductGroupPacket {
                pub p_product_group : Option<CThostFtdcProductGroupField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryMMInstrumentCommissionRatePacket {
                pub p_mm_instrument_commission_rate : Option<CThostFtdcMMInstrumentCommissionRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryMMOptionInstrCommRatePacket {
                pub p_mm_option_instr_comm_rate : Option<CThostFtdcMMOptionInstrCommRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInstrumentOrderCommRatePacket {
                pub p_instrument_order_comm_rate : Option<CThostFtdcInstrumentOrderCommRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySecAgentTradingAccountPacket {
                pub p_trading_account : Option<CThostFtdcTradingAccountField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySecAgentCheckModePacket {
                pub p_sec_agent_check_mode : Option<CThostFtdcSecAgentCheckModeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySecAgentTradeInfoPacket {
                pub p_sec_agent_trade_info : Option<CThostFtdcSecAgentTradeInfoField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryOptionInstrTradeCostPacket {
                pub p_option_instr_trade_cost : Option<CThostFtdcOptionInstrTradeCostField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryOptionInstrCommRatePacket {
                pub p_option_instr_comm_rate : Option<CThostFtdcOptionInstrCommRateField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryExecOrderPacket {
                pub p_exec_order : Option<CThostFtdcExecOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryForQuotePacket {
                pub p_for_quote : Option<CThostFtdcForQuoteField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryQuotePacket {
                pub p_quote : Option<CThostFtdcQuoteField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryOptionSelfClosePacket {
                pub p_option_self_close : Option<CThostFtdcOptionSelfCloseField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestUnitPacket {
                pub p_invest_unit : Option<CThostFtdcInvestUnitField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryCombInstrumentGuardPacket {
                pub p_comb_instrument_guard : Option<CThostFtdcCombInstrumentGuardField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryCombActionPacket {
                pub p_comb_action : Option<CThostFtdcCombActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTransferSerialPacket {
                pub p_transfer_serial : Option<CThostFtdcTransferSerialField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryAccountregisterPacket {
                pub p_accountregister : Option<CThostFtdcAccountregisterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspErrorPacket {
                pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnOrderPacket {
                pub p_order : Option<CThostFtdcOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnTradePacket {
                pub p_trade : Option<CThostFtdcTradeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnOrderInsertPacket {
                pub p_input_order : Option<CThostFtdcInputOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnOrderActionPacket {
                pub p_order_action : Option<CThostFtdcOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnInstrumentStatusPacket {
                pub p_instrument_status : Option<CThostFtdcInstrumentStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnBulletinPacket {
                pub p_bulletin : Option<CThostFtdcBulletinField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnTradingNoticePacket {
                pub p_trading_notice_info : Option<CThostFtdcTradingNoticeInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnErrorConditionalOrderPacket {
                pub p_error_conditional_order : Option<CThostFtdcErrorConditionalOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnExecOrderPacket {
                pub p_exec_order : Option<CThostFtdcExecOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnExecOrderInsertPacket {
                pub p_input_exec_order : Option<CThostFtdcInputExecOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnExecOrderActionPacket {
                pub p_exec_order_action : Option<CThostFtdcExecOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnForQuoteInsertPacket {
                pub p_input_for_quote : Option<CThostFtdcInputForQuoteField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnQuotePacket {
                pub p_quote : Option<CThostFtdcQuoteField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnQuoteInsertPacket {
                pub p_input_quote : Option<CThostFtdcInputQuoteField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnQuoteActionPacket {
                pub p_quote_action : Option<CThostFtdcQuoteActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnForQuoteRspPacket {
                pub p_for_quote_rsp : Option<CThostFtdcForQuoteRspField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnCFMMCTradingAccountTokenPacket {
                pub p_cfmmc_trading_account_token : Option<CThostFtdcCFMMCTradingAccountTokenField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnBatchOrderActionPacket {
                pub p_batch_order_action : Option<CThostFtdcBatchOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnOptionSelfClosePacket {
                pub p_option_self_close : Option<CThostFtdcOptionSelfCloseField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnOptionSelfCloseInsertPacket {
                pub p_input_option_self_close : Option<CThostFtdcInputOptionSelfCloseField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnOptionSelfCloseActionPacket {
                pub p_option_self_close_action : Option<CThostFtdcOptionSelfCloseActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnCombActionPacket {
                pub p_comb_action : Option<CThostFtdcCombActionField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnCombActionInsertPacket {
                pub p_input_comb_action : Option<CThostFtdcInputCombActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryContractBankPacket {
                pub p_contract_bank : Option<CThostFtdcContractBankField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryParkedOrderPacket {
                pub p_parked_order : Option<CThostFtdcParkedOrderField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryParkedOrderActionPacket {
                pub p_parked_order_action : Option<CThostFtdcParkedOrderActionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryTradingNoticePacket {
                pub p_trading_notice : Option<CThostFtdcTradingNoticeField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryBrokerTradingParamsPacket {
                pub p_broker_trading_params : Option<CThostFtdcBrokerTradingParamsField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryBrokerTradingAlgosPacket {
                pub p_broker_trading_algos : Option<CThostFtdcBrokerTradingAlgosField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQueryCFMMCTradingAccountTokenPacket {
                pub p_query_cfmmc_trading_account_token : Option<CThostFtdcQueryCFMMCTradingAccountTokenField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnFromBankToFutureByBankPacket {
                pub p_rsp_transfer : Option<CThostFtdcRspTransferField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnFromFutureToBankByBankPacket {
                pub p_rsp_transfer : Option<CThostFtdcRspTransferField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByBankPacket {
                pub p_rsp_repeal : Option<CThostFtdcRspRepealField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByBankPacket {
                pub p_rsp_repeal : Option<CThostFtdcRspRepealField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnFromBankToFutureByFuturePacket {
                pub p_rsp_transfer : Option<CThostFtdcRspTransferField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnFromFutureToBankByFuturePacket {
                pub p_rsp_transfer : Option<CThostFtdcRspTransferField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByFutureManualPacket {
                pub p_rsp_repeal : Option<CThostFtdcRspRepealField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByFutureManualPacket {
                pub p_rsp_repeal : Option<CThostFtdcRspRepealField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnQueryBankBalanceByFuturePacket {
                pub p_notify_query_account : Option<CThostFtdcNotifyQueryAccountField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnBankToFutureByFuturePacket {
                pub p_req_transfer : Option<CThostFtdcReqTransferField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnFutureToBankByFuturePacket {
                pub p_req_transfer : Option<CThostFtdcReqTransferField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnRepealBankToFutureByFutureManualPacket {
                pub p_req_repeal : Option<CThostFtdcReqRepealField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnRepealFutureToBankByFutureManualPacket {
                pub p_req_repeal : Option<CThostFtdcReqRepealField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnErrRtnQueryBankBalanceByFuturePacket {
                pub p_req_query_account : Option<CThostFtdcReqQueryAccountField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByFuturePacket {
                pub p_rsp_repeal : Option<CThostFtdcRspRepealField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByFuturePacket {
                pub p_rsp_repeal : Option<CThostFtdcRspRepealField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspFromBankToFutureByFuturePacket {
                pub p_req_transfer : Option<CThostFtdcReqTransferField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspFromFutureToBankByFuturePacket {
                pub p_req_transfer : Option<CThostFtdcReqTransferField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQueryBankAccountMoneyByFuturePacket {
                pub p_req_query_account : Option<CThostFtdcReqQueryAccountField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnOpenAccountByBankPacket {
                pub p_open_account : Option<CThostFtdcOpenAccountField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnCancelAccountByBankPacket {
                pub p_cancel_account : Option<CThostFtdcCancelAccountField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRtnChangeAccountByBankPacket {
                pub p_change_account : Option<CThostFtdcChangeAccountField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryClassifiedInstrumentPacket {
                pub p_instrument : Option<CThostFtdcInstrumentField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryCombPromotionParamPacket {
                pub p_comb_promotion_param : Option<CThostFtdcCombPromotionParamField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRiskSettleInvstPositionPacket {
                pub p_risk_settle_invst_position : Option<CThostFtdcRiskSettleInvstPositionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRiskSettleProductStatusPacket {
                pub p_risk_settle_product_status : Option<CThostFtdcRiskSettleProductStatusField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMFutureParameterPacket {
                pub p_spbm_future_parameter : Option<CThostFtdcSPBMFutureParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMOptionParameterPacket {
                pub p_spbm_option_parameter : Option<CThostFtdcSPBMOptionParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMIntraParameterPacket {
                pub p_spbm_intra_parameter : Option<CThostFtdcSPBMIntraParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMInterParameterPacket {
                pub p_spbm_inter_parameter : Option<CThostFtdcSPBMInterParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMPortfDefinitionPacket {
                pub p_spbm_portf_definition : Option<CThostFtdcSPBMPortfDefinitionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMInvestorPortfDefPacket {
                pub p_spbm_investor_portf_def : Option<CThostFtdcSPBMInvestorPortfDefField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorPortfMarginRatioPacket {
                pub p_investor_portf_margin_ratio : Option<CThostFtdcInvestorPortfMarginRatioField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorProdSPBMDetailPacket {
                pub p_investor_prod_spbm_detail : Option<CThostFtdcInvestorProdSPBMDetailField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorCommoditySPMMMarginPacket {
                pub p_investor_commodity_spmm_margin : Option<CThostFtdcInvestorCommoditySPMMMarginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorCommodityGroupSPMMMarginPacket {
                pub p_investor_commodity_group_spmm_margin : Option<CThostFtdcInvestorCommodityGroupSPMMMarginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPMMInstParamPacket {
                pub p_spmm_inst_param : Option<CThostFtdcSPMMInstParamField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPMMProductParamPacket {
                pub p_spmm_product_param : Option<CThostFtdcSPMMProductParamField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQrySPBMAddOnInterParameterPacket {
                pub p_spbm_add_on_inter_parameter : Option<CThostFtdcSPBMAddOnInterParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRCAMSCombProductInfoPacket {
                pub p_rcams_comb_product_info : Option<CThostFtdcRCAMSCombProductInfoField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRCAMSInstrParameterPacket {
                pub p_rcams_instr_parameter : Option<CThostFtdcRCAMSInstrParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRCAMSIntraParameterPacket {
                pub p_rcams_intra_parameter : Option<CThostFtdcRCAMSIntraParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRCAMSInterParameterPacket {
                pub p_rcams_inter_parameter : Option<CThostFtdcRCAMSInterParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRCAMSShortOptAdjustParamPacket {
                pub p_rcams_short_opt_adjust_param : Option<CThostFtdcRCAMSShortOptAdjustParamField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRCAMSInvestorCombPositionPacket {
                pub p_rcams_investor_comb_position : Option<CThostFtdcRCAMSInvestorCombPositionField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorProdRCAMSMarginPacket {
                pub p_investor_prod_rcams_margin : Option<CThostFtdcInvestorProdRCAMSMarginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRULEInstrParameterPacket {
                pub p_rule_instr_parameter : Option<CThostFtdcRULEInstrParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRULEIntraParameterPacket {
                pub p_rule_intra_parameter : Option<CThostFtdcRULEIntraParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryRULEInterParameterPacket {
                pub p_rule_inter_parameter : Option<CThostFtdcRULEInterParameterField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct CThostFtdcTraderSpiOnRspQryInvestorProdRULEMarginPacket {
                pub p_investor_prod_rule_margin : Option<CThostFtdcInvestorProdRULEMarginField>,pub p_rsp_info : Option<CThostFtdcRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }  
static C_THOST_FTDC_TRADER_SPI_VTABLE: CThostFtdcTraderSpiVTable = CThostFtdcTraderSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_heart_beat_warning: spi_on_heart_beat_warning,
            on_rsp_authenticate: spi_on_rsp_authenticate,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_user_password_update: spi_on_rsp_user_password_update,
            on_rsp_trading_account_password_update: spi_on_rsp_trading_account_password_update,
            on_rsp_user_auth_method: spi_on_rsp_user_auth_method,
            on_rsp_gen_user_captcha: spi_on_rsp_gen_user_captcha,
            on_rsp_gen_user_text: spi_on_rsp_gen_user_text,
            on_rsp_order_insert: spi_on_rsp_order_insert,
            on_rsp_parked_order_insert: spi_on_rsp_parked_order_insert,
            on_rsp_parked_order_action: spi_on_rsp_parked_order_action,
            on_rsp_order_action: spi_on_rsp_order_action,
            on_rsp_qry_max_order_volume: spi_on_rsp_qry_max_order_volume,
            on_rsp_settlement_info_confirm: spi_on_rsp_settlement_info_confirm,
            on_rsp_remove_parked_order: spi_on_rsp_remove_parked_order,
            on_rsp_remove_parked_order_action: spi_on_rsp_remove_parked_order_action,
            on_rsp_exec_order_insert: spi_on_rsp_exec_order_insert,
            on_rsp_exec_order_action: spi_on_rsp_exec_order_action,
            on_rsp_for_quote_insert: spi_on_rsp_for_quote_insert,
            on_rsp_quote_insert: spi_on_rsp_quote_insert,
            on_rsp_quote_action: spi_on_rsp_quote_action,
            on_rsp_batch_order_action: spi_on_rsp_batch_order_action,
            on_rsp_option_self_close_insert: spi_on_rsp_option_self_close_insert,
            on_rsp_option_self_close_action: spi_on_rsp_option_self_close_action,
            on_rsp_comb_action_insert: spi_on_rsp_comb_action_insert,
            on_rsp_qry_order: spi_on_rsp_qry_order,
            on_rsp_qry_trade: spi_on_rsp_qry_trade,
            on_rsp_qry_investor_position: spi_on_rsp_qry_investor_position,
            on_rsp_qry_trading_account: spi_on_rsp_qry_trading_account,
            on_rsp_qry_investor: spi_on_rsp_qry_investor,
            on_rsp_qry_trading_code: spi_on_rsp_qry_trading_code,
            on_rsp_qry_instrument_margin_rate: spi_on_rsp_qry_instrument_margin_rate,
            on_rsp_qry_instrument_commission_rate: spi_on_rsp_qry_instrument_commission_rate,
            on_rsp_qry_exchange: spi_on_rsp_qry_exchange,
            on_rsp_qry_product: spi_on_rsp_qry_product,
            on_rsp_qry_instrument: spi_on_rsp_qry_instrument,
            on_rsp_qry_depth_market_data: spi_on_rsp_qry_depth_market_data,
            on_rsp_qry_trader_offer: spi_on_rsp_qry_trader_offer,
            on_rsp_qry_settlement_info: spi_on_rsp_qry_settlement_info,
            on_rsp_qry_transfer_bank: spi_on_rsp_qry_transfer_bank,
            on_rsp_qry_investor_position_detail: spi_on_rsp_qry_investor_position_detail,
            on_rsp_qry_notice: spi_on_rsp_qry_notice,
            on_rsp_qry_settlement_info_confirm: spi_on_rsp_qry_settlement_info_confirm,
            on_rsp_qry_investor_position_combine_detail: spi_on_rsp_qry_investor_position_combine_detail,
            on_rsp_qry_cfmmc_trading_account_key: spi_on_rsp_qry_cfmmc_trading_account_key,
            on_rsp_qry_e_warrant_offset: spi_on_rsp_qry_e_warrant_offset,
            on_rsp_qry_investor_product_group_margin: spi_on_rsp_qry_investor_product_group_margin,
            on_rsp_qry_exchange_margin_rate: spi_on_rsp_qry_exchange_margin_rate,
            on_rsp_qry_exchange_margin_rate_adjust: spi_on_rsp_qry_exchange_margin_rate_adjust,
            on_rsp_qry_exchange_rate: spi_on_rsp_qry_exchange_rate,
            on_rsp_qry_sec_agent_acid_map: spi_on_rsp_qry_sec_agent_acid_map,
            on_rsp_qry_product_exch_rate: spi_on_rsp_qry_product_exch_rate,
            on_rsp_qry_product_group: spi_on_rsp_qry_product_group,
            on_rsp_qry_mm_instrument_commission_rate: spi_on_rsp_qry_mm_instrument_commission_rate,
            on_rsp_qry_mm_option_instr_comm_rate: spi_on_rsp_qry_mm_option_instr_comm_rate,
            on_rsp_qry_instrument_order_comm_rate: spi_on_rsp_qry_instrument_order_comm_rate,
            on_rsp_qry_sec_agent_trading_account: spi_on_rsp_qry_sec_agent_trading_account,
            on_rsp_qry_sec_agent_check_mode: spi_on_rsp_qry_sec_agent_check_mode,
            on_rsp_qry_sec_agent_trade_info: spi_on_rsp_qry_sec_agent_trade_info,
            on_rsp_qry_option_instr_trade_cost: spi_on_rsp_qry_option_instr_trade_cost,
            on_rsp_qry_option_instr_comm_rate: spi_on_rsp_qry_option_instr_comm_rate,
            on_rsp_qry_exec_order: spi_on_rsp_qry_exec_order,
            on_rsp_qry_for_quote: spi_on_rsp_qry_for_quote,
            on_rsp_qry_quote: spi_on_rsp_qry_quote,
            on_rsp_qry_option_self_close: spi_on_rsp_qry_option_self_close,
            on_rsp_qry_invest_unit: spi_on_rsp_qry_invest_unit,
            on_rsp_qry_comb_instrument_guard: spi_on_rsp_qry_comb_instrument_guard,
            on_rsp_qry_comb_action: spi_on_rsp_qry_comb_action,
            on_rsp_qry_transfer_serial: spi_on_rsp_qry_transfer_serial,
            on_rsp_qry_accountregister: spi_on_rsp_qry_accountregister,
            on_rsp_error: spi_on_rsp_error,
            on_rtn_order: spi_on_rtn_order,
            on_rtn_trade: spi_on_rtn_trade,
            on_err_rtn_order_insert: spi_on_err_rtn_order_insert,
            on_err_rtn_order_action: spi_on_err_rtn_order_action,
            on_rtn_instrument_status: spi_on_rtn_instrument_status,
            on_rtn_bulletin: spi_on_rtn_bulletin,
            on_rtn_trading_notice: spi_on_rtn_trading_notice,
            on_rtn_error_conditional_order: spi_on_rtn_error_conditional_order,
            on_rtn_exec_order: spi_on_rtn_exec_order,
            on_err_rtn_exec_order_insert: spi_on_err_rtn_exec_order_insert,
            on_err_rtn_exec_order_action: spi_on_err_rtn_exec_order_action,
            on_err_rtn_for_quote_insert: spi_on_err_rtn_for_quote_insert,
            on_rtn_quote: spi_on_rtn_quote,
            on_err_rtn_quote_insert: spi_on_err_rtn_quote_insert,
            on_err_rtn_quote_action: spi_on_err_rtn_quote_action,
            on_rtn_for_quote_rsp: spi_on_rtn_for_quote_rsp,
            on_rtn_cfmmc_trading_account_token: spi_on_rtn_cfmmc_trading_account_token,
            on_err_rtn_batch_order_action: spi_on_err_rtn_batch_order_action,
            on_rtn_option_self_close: spi_on_rtn_option_self_close,
            on_err_rtn_option_self_close_insert: spi_on_err_rtn_option_self_close_insert,
            on_err_rtn_option_self_close_action: spi_on_err_rtn_option_self_close_action,
            on_rtn_comb_action: spi_on_rtn_comb_action,
            on_err_rtn_comb_action_insert: spi_on_err_rtn_comb_action_insert,
            on_rsp_qry_contract_bank: spi_on_rsp_qry_contract_bank,
            on_rsp_qry_parked_order: spi_on_rsp_qry_parked_order,
            on_rsp_qry_parked_order_action: spi_on_rsp_qry_parked_order_action,
            on_rsp_qry_trading_notice: spi_on_rsp_qry_trading_notice,
            on_rsp_qry_broker_trading_params: spi_on_rsp_qry_broker_trading_params,
            on_rsp_qry_broker_trading_algos: spi_on_rsp_qry_broker_trading_algos,
            on_rsp_query_cfmmc_trading_account_token: spi_on_rsp_query_cfmmc_trading_account_token,
            on_rtn_from_bank_to_future_by_bank: spi_on_rtn_from_bank_to_future_by_bank,
            on_rtn_from_future_to_bank_by_bank: spi_on_rtn_from_future_to_bank_by_bank,
            on_rtn_repeal_from_bank_to_future_by_bank: spi_on_rtn_repeal_from_bank_to_future_by_bank,
            on_rtn_repeal_from_future_to_bank_by_bank: spi_on_rtn_repeal_from_future_to_bank_by_bank,
            on_rtn_from_bank_to_future_by_future: spi_on_rtn_from_bank_to_future_by_future,
            on_rtn_from_future_to_bank_by_future: spi_on_rtn_from_future_to_bank_by_future,
            on_rtn_repeal_from_bank_to_future_by_future_manual: spi_on_rtn_repeal_from_bank_to_future_by_future_manual,
            on_rtn_repeal_from_future_to_bank_by_future_manual: spi_on_rtn_repeal_from_future_to_bank_by_future_manual,
            on_rtn_query_bank_balance_by_future: spi_on_rtn_query_bank_balance_by_future,
            on_err_rtn_bank_to_future_by_future: spi_on_err_rtn_bank_to_future_by_future,
            on_err_rtn_future_to_bank_by_future: spi_on_err_rtn_future_to_bank_by_future,
            on_err_rtn_repeal_bank_to_future_by_future_manual: spi_on_err_rtn_repeal_bank_to_future_by_future_manual,
            on_err_rtn_repeal_future_to_bank_by_future_manual: spi_on_err_rtn_repeal_future_to_bank_by_future_manual,
            on_err_rtn_query_bank_balance_by_future: spi_on_err_rtn_query_bank_balance_by_future,
            on_rtn_repeal_from_bank_to_future_by_future: spi_on_rtn_repeal_from_bank_to_future_by_future,
            on_rtn_repeal_from_future_to_bank_by_future: spi_on_rtn_repeal_from_future_to_bank_by_future,
            on_rsp_from_bank_to_future_by_future: spi_on_rsp_from_bank_to_future_by_future,
            on_rsp_from_future_to_bank_by_future: spi_on_rsp_from_future_to_bank_by_future,
            on_rsp_query_bank_account_money_by_future: spi_on_rsp_query_bank_account_money_by_future,
            on_rtn_open_account_by_bank: spi_on_rtn_open_account_by_bank,
            on_rtn_cancel_account_by_bank: spi_on_rtn_cancel_account_by_bank,
            on_rtn_change_account_by_bank: spi_on_rtn_change_account_by_bank,
            on_rsp_qry_classified_instrument: spi_on_rsp_qry_classified_instrument,
            on_rsp_qry_comb_promotion_param: spi_on_rsp_qry_comb_promotion_param,
            on_rsp_qry_risk_settle_invst_position: spi_on_rsp_qry_risk_settle_invst_position,
            on_rsp_qry_risk_settle_product_status: spi_on_rsp_qry_risk_settle_product_status,
            on_rsp_qry_spbm_future_parameter: spi_on_rsp_qry_spbm_future_parameter,
            on_rsp_qry_spbm_option_parameter: spi_on_rsp_qry_spbm_option_parameter,
            on_rsp_qry_spbm_intra_parameter: spi_on_rsp_qry_spbm_intra_parameter,
            on_rsp_qry_spbm_inter_parameter: spi_on_rsp_qry_spbm_inter_parameter,
            on_rsp_qry_spbm_portf_definition: spi_on_rsp_qry_spbm_portf_definition,
            on_rsp_qry_spbm_investor_portf_def: spi_on_rsp_qry_spbm_investor_portf_def,
            on_rsp_qry_investor_portf_margin_ratio: spi_on_rsp_qry_investor_portf_margin_ratio,
            on_rsp_qry_investor_prod_spbm_detail: spi_on_rsp_qry_investor_prod_spbm_detail,
            on_rsp_qry_investor_commodity_spmm_margin: spi_on_rsp_qry_investor_commodity_spmm_margin,
            on_rsp_qry_investor_commodity_group_spmm_margin: spi_on_rsp_qry_investor_commodity_group_spmm_margin,
            on_rsp_qry_spmm_inst_param: spi_on_rsp_qry_spmm_inst_param,
            on_rsp_qry_spmm_product_param: spi_on_rsp_qry_spmm_product_param,
            on_rsp_qry_spbm_add_on_inter_parameter: spi_on_rsp_qry_spbm_add_on_inter_parameter,
            on_rsp_qry_rcams_comb_product_info: spi_on_rsp_qry_rcams_comb_product_info,
            on_rsp_qry_rcams_instr_parameter: spi_on_rsp_qry_rcams_instr_parameter,
            on_rsp_qry_rcams_intra_parameter: spi_on_rsp_qry_rcams_intra_parameter,
            on_rsp_qry_rcams_inter_parameter: spi_on_rsp_qry_rcams_inter_parameter,
            on_rsp_qry_rcams_short_opt_adjust_param: spi_on_rsp_qry_rcams_short_opt_adjust_param,
            on_rsp_qry_rcams_investor_comb_position: spi_on_rsp_qry_rcams_investor_comb_position,
            on_rsp_qry_investor_prod_rcams_margin: spi_on_rsp_qry_investor_prod_rcams_margin,
            on_rsp_qry_rule_instr_parameter: spi_on_rsp_qry_rule_instr_parameter,
            on_rsp_qry_rule_intra_parameter: spi_on_rsp_qry_rule_intra_parameter,
            on_rsp_qry_rule_inter_parameter: spi_on_rsp_qry_rule_inter_parameter,
            on_rsp_qry_investor_prod_rule_margin: spi_on_rsp_qry_investor_prod_rule_margin,
             };
extern "C" fn spi_on_front_connected(spi: *mut CThostFtdcTraderSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut CThostFtdcTraderSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_heart_beat_warning(spi: *mut CThostFtdcTraderSpiFat, n_time_lapse : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_heart_beat_warning(n_time_lapse)
                    }
                }extern "C" fn spi_on_rsp_authenticate(spi: *mut CThostFtdcTraderSpiFat, p_rsp_authenticate_field : * const CThostFtdcRspAuthenticateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_authenticate(p_rsp_authenticate_field.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut CThostFtdcTraderSpiFat, p_rsp_user_login : * const CThostFtdcRspUserLoginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut CThostFtdcTraderSpiFat, p_user_logout : * const CThostFtdcUserLogoutField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_password_update(spi: *mut CThostFtdcTraderSpiFat, p_user_password_update : * const CThostFtdcUserPasswordUpdateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_password_update(p_user_password_update.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_trading_account_password_update(spi: *mut CThostFtdcTraderSpiFat, p_trading_account_password_update : * const CThostFtdcTradingAccountPasswordUpdateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_trading_account_password_update(p_trading_account_password_update.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_auth_method(spi: *mut CThostFtdcTraderSpiFat, p_rsp_user_auth_method : * const CThostFtdcRspUserAuthMethodField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_auth_method(p_rsp_user_auth_method.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_gen_user_captcha(spi: *mut CThostFtdcTraderSpiFat, p_rsp_gen_user_captcha : * const CThostFtdcRspGenUserCaptchaField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_gen_user_captcha(p_rsp_gen_user_captcha.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_gen_user_text(spi: *mut CThostFtdcTraderSpiFat, p_rsp_gen_user_text : * const CThostFtdcRspGenUserTextField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_gen_user_text(p_rsp_gen_user_text.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_order_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_order : * const CThostFtdcInputOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_insert(p_input_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_parked_order_insert(spi: *mut CThostFtdcTraderSpiFat, p_parked_order : * const CThostFtdcParkedOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_parked_order_insert(p_parked_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_parked_order_action(spi: *mut CThostFtdcTraderSpiFat, p_parked_order_action : * const CThostFtdcParkedOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_parked_order_action(p_parked_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_order_action(spi: *mut CThostFtdcTraderSpiFat, p_input_order_action : * const CThostFtdcInputOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_action(p_input_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_max_order_volume(spi: *mut CThostFtdcTraderSpiFat, p_qry_max_order_volume : * const CThostFtdcQryMaxOrderVolumeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_max_order_volume(p_qry_max_order_volume.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_settlement_info_confirm(spi: *mut CThostFtdcTraderSpiFat, p_settlement_info_confirm : * const CThostFtdcSettlementInfoConfirmField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_settlement_info_confirm(p_settlement_info_confirm.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_remove_parked_order(spi: *mut CThostFtdcTraderSpiFat, p_remove_parked_order : * const CThostFtdcRemoveParkedOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_remove_parked_order(p_remove_parked_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_remove_parked_order_action(spi: *mut CThostFtdcTraderSpiFat, p_remove_parked_order_action : * const CThostFtdcRemoveParkedOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_remove_parked_order_action(p_remove_parked_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_exec_order_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_exec_order : * const CThostFtdcInputExecOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_exec_order_insert(p_input_exec_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_exec_order_action(spi: *mut CThostFtdcTraderSpiFat, p_input_exec_order_action : * const CThostFtdcInputExecOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_exec_order_action(p_input_exec_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_for_quote_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_for_quote : * const CThostFtdcInputForQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_for_quote_insert(p_input_for_quote.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_quote_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_quote : * const CThostFtdcInputQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_quote_insert(p_input_quote.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_quote_action(spi: *mut CThostFtdcTraderSpiFat, p_input_quote_action : * const CThostFtdcInputQuoteActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_quote_action(p_input_quote_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_batch_order_action(spi: *mut CThostFtdcTraderSpiFat, p_input_batch_order_action : * const CThostFtdcInputBatchOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_batch_order_action(p_input_batch_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_option_self_close_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_option_self_close : * const CThostFtdcInputOptionSelfCloseField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_option_self_close_insert(p_input_option_self_close.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_option_self_close_action(spi: *mut CThostFtdcTraderSpiFat, p_input_option_self_close_action : * const CThostFtdcInputOptionSelfCloseActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_option_self_close_action(p_input_option_self_close_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_comb_action_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_comb_action : * const CThostFtdcInputCombActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_action_insert(p_input_comb_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order(spi: *mut CThostFtdcTraderSpiFat, p_order : * const CThostFtdcOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order(p_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trade(spi: *mut CThostFtdcTraderSpiFat, p_trade : * const CThostFtdcTradeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trade(p_trade.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_position(spi: *mut CThostFtdcTraderSpiFat, p_investor_position : * const CThostFtdcInvestorPositionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_position(p_investor_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut CThostFtdcTraderSpiFat, p_trading_account : * const CThostFtdcTradingAccountField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_account(p_trading_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor(spi: *mut CThostFtdcTraderSpiFat, p_investor : * const CThostFtdcInvestorField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor(p_investor.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_code(spi: *mut CThostFtdcTraderSpiFat, p_trading_code : * const CThostFtdcTradingCodeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_code(p_trading_code.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_instrument_margin_rate(spi: *mut CThostFtdcTraderSpiFat, p_instrument_margin_rate : * const CThostFtdcInstrumentMarginRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_instrument_margin_rate(p_instrument_margin_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_instrument_commission_rate(spi: *mut CThostFtdcTraderSpiFat, p_instrument_commission_rate : * const CThostFtdcInstrumentCommissionRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_instrument_commission_rate(p_instrument_commission_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exchange(spi: *mut CThostFtdcTraderSpiFat, p_exchange : * const CThostFtdcExchangeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange(p_exchange.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_product(spi: *mut CThostFtdcTraderSpiFat, p_product : * const CThostFtdcProductField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_product(p_product.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_instrument(spi: *mut CThostFtdcTraderSpiFat, p_instrument : * const CThostFtdcInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_instrument(p_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_depth_market_data(spi: *mut CThostFtdcTraderSpiFat, p_depth_market_data : * const CThostFtdcDepthMarketDataField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_depth_market_data(p_depth_market_data.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trader_offer(spi: *mut CThostFtdcTraderSpiFat, p_trader_offer : * const CThostFtdcTraderOfferField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trader_offer(p_trader_offer.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_settlement_info(spi: *mut CThostFtdcTraderSpiFat, p_settlement_info : * const CThostFtdcSettlementInfoField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_settlement_info(p_settlement_info.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_transfer_bank(spi: *mut CThostFtdcTraderSpiFat, p_transfer_bank : * const CThostFtdcTransferBankField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_transfer_bank(p_transfer_bank.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_position_detail(spi: *mut CThostFtdcTraderSpiFat, p_investor_position_detail : * const CThostFtdcInvestorPositionDetailField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_position_detail(p_investor_position_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_notice(spi: *mut CThostFtdcTraderSpiFat, p_notice : * const CThostFtdcNoticeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_notice(p_notice.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_settlement_info_confirm(spi: *mut CThostFtdcTraderSpiFat, p_settlement_info_confirm : * const CThostFtdcSettlementInfoConfirmField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_settlement_info_confirm(p_settlement_info_confirm.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_position_combine_detail(spi: *mut CThostFtdcTraderSpiFat, p_investor_position_combine_detail : * const CThostFtdcInvestorPositionCombineDetailField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_position_combine_detail(p_investor_position_combine_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cfmmc_trading_account_key(spi: *mut CThostFtdcTraderSpiFat, p_cfmmc_trading_account_key : * const CThostFtdcCFMMCTradingAccountKeyField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cfmmc_trading_account_key(p_cfmmc_trading_account_key.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_e_warrant_offset(spi: *mut CThostFtdcTraderSpiFat, p_e_warrant_offset : * const CThostFtdcEWarrantOffsetField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_e_warrant_offset(p_e_warrant_offset.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_product_group_margin(spi: *mut CThostFtdcTraderSpiFat, p_investor_product_group_margin : * const CThostFtdcInvestorProductGroupMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_product_group_margin(p_investor_product_group_margin.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exchange_margin_rate(spi: *mut CThostFtdcTraderSpiFat, p_exchange_margin_rate : * const CThostFtdcExchangeMarginRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange_margin_rate(p_exchange_margin_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exchange_margin_rate_adjust(spi: *mut CThostFtdcTraderSpiFat, p_exchange_margin_rate_adjust : * const CThostFtdcExchangeMarginRateAdjustField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange_margin_rate_adjust(p_exchange_margin_rate_adjust.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exchange_rate(spi: *mut CThostFtdcTraderSpiFat, p_exchange_rate : * const CThostFtdcExchangeRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange_rate(p_exchange_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_sec_agent_acid_map(spi: *mut CThostFtdcTraderSpiFat, p_sec_agent_acid_map : * const CThostFtdcSecAgentACIDMapField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_sec_agent_acid_map(p_sec_agent_acid_map.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_product_exch_rate(spi: *mut CThostFtdcTraderSpiFat, p_product_exch_rate : * const CThostFtdcProductExchRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_product_exch_rate(p_product_exch_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_product_group(spi: *mut CThostFtdcTraderSpiFat, p_product_group : * const CThostFtdcProductGroupField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_product_group(p_product_group.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_mm_instrument_commission_rate(spi: *mut CThostFtdcTraderSpiFat, p_mm_instrument_commission_rate : * const CThostFtdcMMInstrumentCommissionRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_mm_instrument_commission_rate(p_mm_instrument_commission_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_mm_option_instr_comm_rate(spi: *mut CThostFtdcTraderSpiFat, p_mm_option_instr_comm_rate : * const CThostFtdcMMOptionInstrCommRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_mm_option_instr_comm_rate(p_mm_option_instr_comm_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_instrument_order_comm_rate(spi: *mut CThostFtdcTraderSpiFat, p_instrument_order_comm_rate : * const CThostFtdcInstrumentOrderCommRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_instrument_order_comm_rate(p_instrument_order_comm_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_sec_agent_trading_account(spi: *mut CThostFtdcTraderSpiFat, p_trading_account : * const CThostFtdcTradingAccountField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_sec_agent_trading_account(p_trading_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_sec_agent_check_mode(spi: *mut CThostFtdcTraderSpiFat, p_sec_agent_check_mode : * const CThostFtdcSecAgentCheckModeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_sec_agent_check_mode(p_sec_agent_check_mode.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_sec_agent_trade_info(spi: *mut CThostFtdcTraderSpiFat, p_sec_agent_trade_info : * const CThostFtdcSecAgentTradeInfoField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_sec_agent_trade_info(p_sec_agent_trade_info.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_option_instr_trade_cost(spi: *mut CThostFtdcTraderSpiFat, p_option_instr_trade_cost : * const CThostFtdcOptionInstrTradeCostField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_option_instr_trade_cost(p_option_instr_trade_cost.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_option_instr_comm_rate(spi: *mut CThostFtdcTraderSpiFat, p_option_instr_comm_rate : * const CThostFtdcOptionInstrCommRateField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_option_instr_comm_rate(p_option_instr_comm_rate.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exec_order(spi: *mut CThostFtdcTraderSpiFat, p_exec_order : * const CThostFtdcExecOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exec_order(p_exec_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_for_quote(spi: *mut CThostFtdcTraderSpiFat, p_for_quote : * const CThostFtdcForQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_for_quote(p_for_quote.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_quote(spi: *mut CThostFtdcTraderSpiFat, p_quote : * const CThostFtdcQuoteField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_quote(p_quote.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_option_self_close(spi: *mut CThostFtdcTraderSpiFat, p_option_self_close : * const CThostFtdcOptionSelfCloseField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_option_self_close(p_option_self_close.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_invest_unit(spi: *mut CThostFtdcTraderSpiFat, p_invest_unit : * const CThostFtdcInvestUnitField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_invest_unit(p_invest_unit.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_instrument_guard(spi: *mut CThostFtdcTraderSpiFat, p_comb_instrument_guard : * const CThostFtdcCombInstrumentGuardField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_instrument_guard(p_comb_instrument_guard.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_action(spi: *mut CThostFtdcTraderSpiFat, p_comb_action : * const CThostFtdcCombActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_action(p_comb_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_transfer_serial(spi: *mut CThostFtdcTraderSpiFat, p_transfer_serial : * const CThostFtdcTransferSerialField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_transfer_serial(p_transfer_serial.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_accountregister(spi: *mut CThostFtdcTraderSpiFat, p_accountregister : * const CThostFtdcAccountregisterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_accountregister(p_accountregister.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_error(spi: *mut CThostFtdcTraderSpiFat, p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_error(p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rtn_order(spi: *mut CThostFtdcTraderSpiFat, p_order : * const CThostFtdcOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_order(p_order.as_ref())
                    }
                }extern "C" fn spi_on_rtn_trade(spi: *mut CThostFtdcTraderSpiFat, p_trade : * const CThostFtdcTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trade(p_trade.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_order_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_order : * const CThostFtdcInputOrderField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_insert(p_input_order.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_order_action(spi: *mut CThostFtdcTraderSpiFat, p_order_action : * const CThostFtdcOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_action(p_order_action.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_instrument_status(spi: *mut CThostFtdcTraderSpiFat, p_instrument_status : * const CThostFtdcInstrumentStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_instrument_status(p_instrument_status.as_ref())
                    }
                }extern "C" fn spi_on_rtn_bulletin(spi: *mut CThostFtdcTraderSpiFat, p_bulletin : * const CThostFtdcBulletinField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_bulletin(p_bulletin.as_ref())
                    }
                }extern "C" fn spi_on_rtn_trading_notice(spi: *mut CThostFtdcTraderSpiFat, p_trading_notice_info : * const CThostFtdcTradingNoticeInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trading_notice(p_trading_notice_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_error_conditional_order(spi: *mut CThostFtdcTraderSpiFat, p_error_conditional_order : * const CThostFtdcErrorConditionalOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_error_conditional_order(p_error_conditional_order.as_ref())
                    }
                }extern "C" fn spi_on_rtn_exec_order(spi: *mut CThostFtdcTraderSpiFat, p_exec_order : * const CThostFtdcExecOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_exec_order(p_exec_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_exec_order_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_exec_order : * const CThostFtdcInputExecOrderField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_exec_order_insert(p_input_exec_order.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_exec_order_action(spi: *mut CThostFtdcTraderSpiFat, p_exec_order_action : * const CThostFtdcExecOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_exec_order_action(p_exec_order_action.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_for_quote_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_for_quote : * const CThostFtdcInputForQuoteField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_for_quote_insert(p_input_for_quote.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_quote(spi: *mut CThostFtdcTraderSpiFat, p_quote : * const CThostFtdcQuoteField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_quote(p_quote.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_quote_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_quote : * const CThostFtdcInputQuoteField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_quote_insert(p_input_quote.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_quote_action(spi: *mut CThostFtdcTraderSpiFat, p_quote_action : * const CThostFtdcQuoteActionField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_quote_action(p_quote_action.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut CThostFtdcTraderSpiFat, p_for_quote_rsp : * const CThostFtdcForQuoteRspField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_for_quote_rsp(p_for_quote_rsp.as_ref())
                    }
                }extern "C" fn spi_on_rtn_cfmmc_trading_account_token(spi: *mut CThostFtdcTraderSpiFat, p_cfmmc_trading_account_token : * const CThostFtdcCFMMCTradingAccountTokenField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_cfmmc_trading_account_token(p_cfmmc_trading_account_token.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_batch_order_action(spi: *mut CThostFtdcTraderSpiFat, p_batch_order_action : * const CThostFtdcBatchOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_batch_order_action(p_batch_order_action.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_option_self_close(spi: *mut CThostFtdcTraderSpiFat, p_option_self_close : * const CThostFtdcOptionSelfCloseField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_option_self_close(p_option_self_close.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_option_self_close_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_option_self_close : * const CThostFtdcInputOptionSelfCloseField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_option_self_close_insert(p_input_option_self_close.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_option_self_close_action(spi: *mut CThostFtdcTraderSpiFat, p_option_self_close_action : * const CThostFtdcOptionSelfCloseActionField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_option_self_close_action(p_option_self_close_action.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_comb_action(spi: *mut CThostFtdcTraderSpiFat, p_comb_action : * const CThostFtdcCombActionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_comb_action(p_comb_action.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_comb_action_insert(spi: *mut CThostFtdcTraderSpiFat, p_input_comb_action : * const CThostFtdcInputCombActionField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_action_insert(p_input_comb_action.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rsp_qry_contract_bank(spi: *mut CThostFtdcTraderSpiFat, p_contract_bank : * const CThostFtdcContractBankField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_contract_bank(p_contract_bank.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_parked_order(spi: *mut CThostFtdcTraderSpiFat, p_parked_order : * const CThostFtdcParkedOrderField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_parked_order(p_parked_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_parked_order_action(spi: *mut CThostFtdcTraderSpiFat, p_parked_order_action : * const CThostFtdcParkedOrderActionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_parked_order_action(p_parked_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut CThostFtdcTraderSpiFat, p_trading_notice : * const CThostFtdcTradingNoticeField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_notice(p_trading_notice.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_broker_trading_params(spi: *mut CThostFtdcTraderSpiFat, p_broker_trading_params : * const CThostFtdcBrokerTradingParamsField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_broker_trading_params(p_broker_trading_params.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_broker_trading_algos(spi: *mut CThostFtdcTraderSpiFat, p_broker_trading_algos : * const CThostFtdcBrokerTradingAlgosField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_broker_trading_algos(p_broker_trading_algos.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_query_cfmmc_trading_account_token(spi: *mut CThostFtdcTraderSpiFat, p_query_cfmmc_trading_account_token : * const CThostFtdcQueryCFMMCTradingAccountTokenField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_query_cfmmc_trading_account_token(p_query_cfmmc_trading_account_token.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rtn_from_bank_to_future_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_from_bank_to_future_by_bank(p_rsp_transfer.as_ref())
                    }
                }extern "C" fn spi_on_rtn_from_future_to_bank_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_from_future_to_bank_by_bank(p_rsp_transfer.as_ref())
                    }
                }extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_repeal_from_bank_to_future_by_bank(p_rsp_repeal.as_ref())
                    }
                }extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_repeal_from_future_to_bank_by_bank(p_rsp_repeal.as_ref())
                    }
                }extern "C" fn spi_on_rtn_from_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_from_bank_to_future_by_future(p_rsp_transfer.as_ref())
                    }
                }extern "C" fn spi_on_rtn_from_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpiFat, p_rsp_transfer : * const CThostFtdcRspTransferField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_from_future_to_bank_by_future(p_rsp_transfer.as_ref())
                    }
                }extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_future_manual(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_repeal_from_bank_to_future_by_future_manual(p_rsp_repeal.as_ref())
                    }
                }extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_future_manual(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_repeal_from_future_to_bank_by_future_manual(p_rsp_repeal.as_ref())
                    }
                }extern "C" fn spi_on_rtn_query_bank_balance_by_future(spi: *mut CThostFtdcTraderSpiFat, p_notify_query_account : * const CThostFtdcNotifyQueryAccountField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_query_bank_balance_by_future(p_notify_query_account.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_bank_to_future_by_future(p_req_transfer.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_future_to_bank_by_future(p_req_transfer.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_repeal_bank_to_future_by_future_manual(spi: *mut CThostFtdcTraderSpiFat, p_req_repeal : * const CThostFtdcReqRepealField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_repeal_bank_to_future_by_future_manual(p_req_repeal.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_repeal_future_to_bank_by_future_manual(spi: *mut CThostFtdcTraderSpiFat, p_req_repeal : * const CThostFtdcReqRepealField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_repeal_future_to_bank_by_future_manual(p_req_repeal.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_query_bank_balance_by_future(spi: *mut CThostFtdcTraderSpiFat, p_req_query_account : * const CThostFtdcReqQueryAccountField,p_rsp_info : * const CThostFtdcRspInfoField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_query_bank_balance_by_future(p_req_query_account.as_ref(),p_rsp_info.as_ref())
                    }
                }extern "C" fn spi_on_rtn_repeal_from_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_repeal_from_bank_to_future_by_future(p_rsp_repeal.as_ref())
                    }
                }extern "C" fn spi_on_rtn_repeal_from_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpiFat, p_rsp_repeal : * const CThostFtdcRspRepealField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_repeal_from_future_to_bank_by_future(p_rsp_repeal.as_ref())
                    }
                }extern "C" fn spi_on_rsp_from_bank_to_future_by_future(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_from_bank_to_future_by_future(p_req_transfer.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_from_future_to_bank_by_future(spi: *mut CThostFtdcTraderSpiFat, p_req_transfer : * const CThostFtdcReqTransferField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_from_future_to_bank_by_future(p_req_transfer.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_query_bank_account_money_by_future(spi: *mut CThostFtdcTraderSpiFat, p_req_query_account : * const CThostFtdcReqQueryAccountField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_query_bank_account_money_by_future(p_req_query_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rtn_open_account_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_open_account : * const CThostFtdcOpenAccountField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_open_account_by_bank(p_open_account.as_ref())
                    }
                }extern "C" fn spi_on_rtn_cancel_account_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_cancel_account : * const CThostFtdcCancelAccountField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_cancel_account_by_bank(p_cancel_account.as_ref())
                    }
                }extern "C" fn spi_on_rtn_change_account_by_bank(spi: *mut CThostFtdcTraderSpiFat, p_change_account : * const CThostFtdcChangeAccountField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_change_account_by_bank(p_change_account.as_ref())
                    }
                }extern "C" fn spi_on_rsp_qry_classified_instrument(spi: *mut CThostFtdcTraderSpiFat, p_instrument : * const CThostFtdcInstrumentField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_classified_instrument(p_instrument.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_promotion_param(spi: *mut CThostFtdcTraderSpiFat, p_comb_promotion_param : * const CThostFtdcCombPromotionParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_promotion_param(p_comb_promotion_param.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_risk_settle_invst_position(spi: *mut CThostFtdcTraderSpiFat, p_risk_settle_invst_position : * const CThostFtdcRiskSettleInvstPositionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_risk_settle_invst_position(p_risk_settle_invst_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_risk_settle_product_status(spi: *mut CThostFtdcTraderSpiFat, p_risk_settle_product_status : * const CThostFtdcRiskSettleProductStatusField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_risk_settle_product_status(p_risk_settle_product_status.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_future_parameter(spi: *mut CThostFtdcTraderSpiFat, p_spbm_future_parameter : * const CThostFtdcSPBMFutureParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_future_parameter(p_spbm_future_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_option_parameter(spi: *mut CThostFtdcTraderSpiFat, p_spbm_option_parameter : * const CThostFtdcSPBMOptionParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_option_parameter(p_spbm_option_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_intra_parameter(spi: *mut CThostFtdcTraderSpiFat, p_spbm_intra_parameter : * const CThostFtdcSPBMIntraParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_intra_parameter(p_spbm_intra_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_inter_parameter(spi: *mut CThostFtdcTraderSpiFat, p_spbm_inter_parameter : * const CThostFtdcSPBMInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_inter_parameter(p_spbm_inter_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_portf_definition(spi: *mut CThostFtdcTraderSpiFat, p_spbm_portf_definition : * const CThostFtdcSPBMPortfDefinitionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_portf_definition(p_spbm_portf_definition.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_investor_portf_def(spi: *mut CThostFtdcTraderSpiFat, p_spbm_investor_portf_def : * const CThostFtdcSPBMInvestorPortfDefField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_investor_portf_def(p_spbm_investor_portf_def.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_portf_margin_ratio(spi: *mut CThostFtdcTraderSpiFat, p_investor_portf_margin_ratio : * const CThostFtdcInvestorPortfMarginRatioField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_portf_margin_ratio(p_investor_portf_margin_ratio.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_prod_spbm_detail(spi: *mut CThostFtdcTraderSpiFat, p_investor_prod_spbm_detail : * const CThostFtdcInvestorProdSPBMDetailField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_prod_spbm_detail(p_investor_prod_spbm_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_commodity_spmm_margin(spi: *mut CThostFtdcTraderSpiFat, p_investor_commodity_spmm_margin : * const CThostFtdcInvestorCommoditySPMMMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_commodity_spmm_margin(p_investor_commodity_spmm_margin.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_commodity_group_spmm_margin(spi: *mut CThostFtdcTraderSpiFat, p_investor_commodity_group_spmm_margin : * const CThostFtdcInvestorCommodityGroupSPMMMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_commodity_group_spmm_margin(p_investor_commodity_group_spmm_margin.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spmm_inst_param(spi: *mut CThostFtdcTraderSpiFat, p_spmm_inst_param : * const CThostFtdcSPMMInstParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spmm_inst_param(p_spmm_inst_param.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spmm_product_param(spi: *mut CThostFtdcTraderSpiFat, p_spmm_product_param : * const CThostFtdcSPMMProductParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spmm_product_param(p_spmm_product_param.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_spbm_add_on_inter_parameter(spi: *mut CThostFtdcTraderSpiFat, p_spbm_add_on_inter_parameter : * const CThostFtdcSPBMAddOnInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_spbm_add_on_inter_parameter(p_spbm_add_on_inter_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rcams_comb_product_info(spi: *mut CThostFtdcTraderSpiFat, p_rcams_comb_product_info : * const CThostFtdcRCAMSCombProductInfoField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rcams_comb_product_info(p_rcams_comb_product_info.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rcams_instr_parameter(spi: *mut CThostFtdcTraderSpiFat, p_rcams_instr_parameter : * const CThostFtdcRCAMSInstrParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rcams_instr_parameter(p_rcams_instr_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rcams_intra_parameter(spi: *mut CThostFtdcTraderSpiFat, p_rcams_intra_parameter : * const CThostFtdcRCAMSIntraParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rcams_intra_parameter(p_rcams_intra_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rcams_inter_parameter(spi: *mut CThostFtdcTraderSpiFat, p_rcams_inter_parameter : * const CThostFtdcRCAMSInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rcams_inter_parameter(p_rcams_inter_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rcams_short_opt_adjust_param(spi: *mut CThostFtdcTraderSpiFat, p_rcams_short_opt_adjust_param : * const CThostFtdcRCAMSShortOptAdjustParamField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rcams_short_opt_adjust_param(p_rcams_short_opt_adjust_param.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rcams_investor_comb_position(spi: *mut CThostFtdcTraderSpiFat, p_rcams_investor_comb_position : * const CThostFtdcRCAMSInvestorCombPositionField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rcams_investor_comb_position(p_rcams_investor_comb_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_prod_rcams_margin(spi: *mut CThostFtdcTraderSpiFat, p_investor_prod_rcams_margin : * const CThostFtdcInvestorProdRCAMSMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_prod_rcams_margin(p_investor_prod_rcams_margin.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rule_instr_parameter(spi: *mut CThostFtdcTraderSpiFat, p_rule_instr_parameter : * const CThostFtdcRULEInstrParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rule_instr_parameter(p_rule_instr_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rule_intra_parameter(spi: *mut CThostFtdcTraderSpiFat, p_rule_intra_parameter : * const CThostFtdcRULEIntraParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rule_intra_parameter(p_rule_intra_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rule_inter_parameter(spi: *mut CThostFtdcTraderSpiFat, p_rule_inter_parameter : * const CThostFtdcRULEInterParameterField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rule_inter_parameter(p_rule_inter_parameter.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_prod_rule_margin(spi: *mut CThostFtdcTraderSpiFat, p_investor_prod_rule_margin : * const CThostFtdcInvestorProdRULEMarginField,p_rsp_info : * const CThostFtdcRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_prod_rule_margin(p_investor_prod_rule_margin.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }

        #[repr(C)]
        pub struct CThostFtdcTraderSpiFat {
            vtable: *const CThostFtdcTraderSpiVTable,
            pub md_spi_ptr: *mut dyn CThostFtdcTraderSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct CThostFtdcTraderSpiInner {
            buf: std::collections::VecDeque<CThostFtdcTraderSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl CThostFtdcTraderSpiInner {
            fn push(&mut self, msg: CThostFtdcTraderSpiOutput) {
                self.buf.push_back(msg);
                if let Some(waker) = self.waker.take() {
                    waker.wake()
                }
            }
        }
        
        pub struct CThostFtdcTraderSpiStream {
            inner: Arc<Mutex<CThostFtdcTraderSpiInner>>,
        }
        
        impl Stream for CThostFtdcTraderSpiStream {
            type Item = CThostFtdcTraderSpiOutput;
        
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
        
        pub fn create_spi() -> (Box<CThostFtdcTraderSpiStream>, *mut CThostFtdcTraderSpiStream) {
            let i = CThostFtdcTraderSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = CThostFtdcTraderSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl CThostFtdcTraderSpi_trait for CThostFtdcTraderSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnFrontConnected( CThostFtdcTraderSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnFrontDisconnected( CThostFtdcTraderSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_heart_beat_warning(&mut self, n_time_lapse : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnHeartBeatWarning( CThostFtdcTraderSpiOnHeartBeatWarningPacket { n_time_lapse:n_time_lapse } ))
                }
            fn on_rsp_authenticate(&mut self, p_rsp_authenticate_field : Option<&CThostFtdcRspAuthenticateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspAuthenticate( CThostFtdcTraderSpiOnRspAuthenticatePacket { p_rsp_authenticate_field:p_rsp_authenticate_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login : Option<&CThostFtdcRspUserLoginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspUserLogin( CThostFtdcTraderSpiOnRspUserLoginPacket { p_rsp_user_login:p_rsp_user_login.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout : Option<&CThostFtdcUserLogoutField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspUserLogout( CThostFtdcTraderSpiOnRspUserLogoutPacket { p_user_logout:p_user_logout.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_password_update(&mut self, p_user_password_update : Option<&CThostFtdcUserPasswordUpdateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspUserPasswordUpdate( CThostFtdcTraderSpiOnRspUserPasswordUpdatePacket { p_user_password_update:p_user_password_update.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_trading_account_password_update(&mut self, p_trading_account_password_update : Option<&CThostFtdcTradingAccountPasswordUpdateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspTradingAccountPasswordUpdate( CThostFtdcTraderSpiOnRspTradingAccountPasswordUpdatePacket { p_trading_account_password_update:p_trading_account_password_update.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_auth_method(&mut self, p_rsp_user_auth_method : Option<&CThostFtdcRspUserAuthMethodField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspUserAuthMethod( CThostFtdcTraderSpiOnRspUserAuthMethodPacket { p_rsp_user_auth_method:p_rsp_user_auth_method.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_gen_user_captcha(&mut self, p_rsp_gen_user_captcha : Option<&CThostFtdcRspGenUserCaptchaField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspGenUserCaptcha( CThostFtdcTraderSpiOnRspGenUserCaptchaPacket { p_rsp_gen_user_captcha:p_rsp_gen_user_captcha.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_gen_user_text(&mut self, p_rsp_gen_user_text : Option<&CThostFtdcRspGenUserTextField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspGenUserText( CThostFtdcTraderSpiOnRspGenUserTextPacket { p_rsp_gen_user_text:p_rsp_gen_user_text.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_order_insert(&mut self, p_input_order : Option<&CThostFtdcInputOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspOrderInsert( CThostFtdcTraderSpiOnRspOrderInsertPacket { p_input_order:p_input_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_parked_order_insert(&mut self, p_parked_order : Option<&CThostFtdcParkedOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspParkedOrderInsert( CThostFtdcTraderSpiOnRspParkedOrderInsertPacket { p_parked_order:p_parked_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_parked_order_action(&mut self, p_parked_order_action : Option<&CThostFtdcParkedOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspParkedOrderAction( CThostFtdcTraderSpiOnRspParkedOrderActionPacket { p_parked_order_action:p_parked_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_order_action(&mut self, p_input_order_action : Option<&CThostFtdcInputOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspOrderAction( CThostFtdcTraderSpiOnRspOrderActionPacket { p_input_order_action:p_input_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_max_order_volume(&mut self, p_qry_max_order_volume : Option<&CThostFtdcQryMaxOrderVolumeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryMaxOrderVolume( CThostFtdcTraderSpiOnRspQryMaxOrderVolumePacket { p_qry_max_order_volume:p_qry_max_order_volume.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_settlement_info_confirm(&mut self, p_settlement_info_confirm : Option<&CThostFtdcSettlementInfoConfirmField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspSettlementInfoConfirm( CThostFtdcTraderSpiOnRspSettlementInfoConfirmPacket { p_settlement_info_confirm:p_settlement_info_confirm.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_remove_parked_order(&mut self, p_remove_parked_order : Option<&CThostFtdcRemoveParkedOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspRemoveParkedOrder( CThostFtdcTraderSpiOnRspRemoveParkedOrderPacket { p_remove_parked_order:p_remove_parked_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_remove_parked_order_action(&mut self, p_remove_parked_order_action : Option<&CThostFtdcRemoveParkedOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspRemoveParkedOrderAction( CThostFtdcTraderSpiOnRspRemoveParkedOrderActionPacket { p_remove_parked_order_action:p_remove_parked_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_exec_order_insert(&mut self, p_input_exec_order : Option<&CThostFtdcInputExecOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspExecOrderInsert( CThostFtdcTraderSpiOnRspExecOrderInsertPacket { p_input_exec_order:p_input_exec_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_exec_order_action(&mut self, p_input_exec_order_action : Option<&CThostFtdcInputExecOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspExecOrderAction( CThostFtdcTraderSpiOnRspExecOrderActionPacket { p_input_exec_order_action:p_input_exec_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_for_quote_insert(&mut self, p_input_for_quote : Option<&CThostFtdcInputForQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspForQuoteInsert( CThostFtdcTraderSpiOnRspForQuoteInsertPacket { p_input_for_quote:p_input_for_quote.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_quote_insert(&mut self, p_input_quote : Option<&CThostFtdcInputQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQuoteInsert( CThostFtdcTraderSpiOnRspQuoteInsertPacket { p_input_quote:p_input_quote.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_quote_action(&mut self, p_input_quote_action : Option<&CThostFtdcInputQuoteActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQuoteAction( CThostFtdcTraderSpiOnRspQuoteActionPacket { p_input_quote_action:p_input_quote_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_batch_order_action(&mut self, p_input_batch_order_action : Option<&CThostFtdcInputBatchOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspBatchOrderAction( CThostFtdcTraderSpiOnRspBatchOrderActionPacket { p_input_batch_order_action:p_input_batch_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_option_self_close_insert(&mut self, p_input_option_self_close : Option<&CThostFtdcInputOptionSelfCloseField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspOptionSelfCloseInsert( CThostFtdcTraderSpiOnRspOptionSelfCloseInsertPacket { p_input_option_self_close:p_input_option_self_close.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_option_self_close_action(&mut self, p_input_option_self_close_action : Option<&CThostFtdcInputOptionSelfCloseActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspOptionSelfCloseAction( CThostFtdcTraderSpiOnRspOptionSelfCloseActionPacket { p_input_option_self_close_action:p_input_option_self_close_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_comb_action_insert(&mut self, p_input_comb_action : Option<&CThostFtdcInputCombActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspCombActionInsert( CThostFtdcTraderSpiOnRspCombActionInsertPacket { p_input_comb_action:p_input_comb_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order(&mut self, p_order : Option<&CThostFtdcOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryOrder( CThostFtdcTraderSpiOnRspQryOrderPacket { p_order:p_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trade(&mut self, p_trade : Option<&CThostFtdcTradeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTrade( CThostFtdcTraderSpiOnRspQryTradePacket { p_trade:p_trade.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_position(&mut self, p_investor_position : Option<&CThostFtdcInvestorPositionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorPosition( CThostFtdcTraderSpiOnRspQryInvestorPositionPacket { p_investor_position:p_investor_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_account(&mut self, p_trading_account : Option<&CThostFtdcTradingAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTradingAccount( CThostFtdcTraderSpiOnRspQryTradingAccountPacket { p_trading_account:p_trading_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor(&mut self, p_investor : Option<&CThostFtdcInvestorField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestor( CThostFtdcTraderSpiOnRspQryInvestorPacket { p_investor:p_investor.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_code(&mut self, p_trading_code : Option<&CThostFtdcTradingCodeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTradingCode( CThostFtdcTraderSpiOnRspQryTradingCodePacket { p_trading_code:p_trading_code.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_instrument_margin_rate(&mut self, p_instrument_margin_rate : Option<&CThostFtdcInstrumentMarginRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInstrumentMarginRate( CThostFtdcTraderSpiOnRspQryInstrumentMarginRatePacket { p_instrument_margin_rate:p_instrument_margin_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_instrument_commission_rate(&mut self, p_instrument_commission_rate : Option<&CThostFtdcInstrumentCommissionRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInstrumentCommissionRate( CThostFtdcTraderSpiOnRspQryInstrumentCommissionRatePacket { p_instrument_commission_rate:p_instrument_commission_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exchange(&mut self, p_exchange : Option<&CThostFtdcExchangeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryExchange( CThostFtdcTraderSpiOnRspQryExchangePacket { p_exchange:p_exchange.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_product(&mut self, p_product : Option<&CThostFtdcProductField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryProduct( CThostFtdcTraderSpiOnRspQryProductPacket { p_product:p_product.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_instrument(&mut self, p_instrument : Option<&CThostFtdcInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInstrument( CThostFtdcTraderSpiOnRspQryInstrumentPacket { p_instrument:p_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_depth_market_data(&mut self, p_depth_market_data : Option<&CThostFtdcDepthMarketDataField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryDepthMarketData( CThostFtdcTraderSpiOnRspQryDepthMarketDataPacket { p_depth_market_data:p_depth_market_data.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trader_offer(&mut self, p_trader_offer : Option<&CThostFtdcTraderOfferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTraderOffer( CThostFtdcTraderSpiOnRspQryTraderOfferPacket { p_trader_offer:p_trader_offer.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_settlement_info(&mut self, p_settlement_info : Option<&CThostFtdcSettlementInfoField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySettlementInfo( CThostFtdcTraderSpiOnRspQrySettlementInfoPacket { p_settlement_info:p_settlement_info.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_transfer_bank(&mut self, p_transfer_bank : Option<&CThostFtdcTransferBankField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTransferBank( CThostFtdcTraderSpiOnRspQryTransferBankPacket { p_transfer_bank:p_transfer_bank.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_position_detail(&mut self, p_investor_position_detail : Option<&CThostFtdcInvestorPositionDetailField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorPositionDetail( CThostFtdcTraderSpiOnRspQryInvestorPositionDetailPacket { p_investor_position_detail:p_investor_position_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_notice(&mut self, p_notice : Option<&CThostFtdcNoticeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryNotice( CThostFtdcTraderSpiOnRspQryNoticePacket { p_notice:p_notice.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_settlement_info_confirm(&mut self, p_settlement_info_confirm : Option<&CThostFtdcSettlementInfoConfirmField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySettlementInfoConfirm( CThostFtdcTraderSpiOnRspQrySettlementInfoConfirmPacket { p_settlement_info_confirm:p_settlement_info_confirm.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_position_combine_detail(&mut self, p_investor_position_combine_detail : Option<&CThostFtdcInvestorPositionCombineDetailField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorPositionCombineDetail( CThostFtdcTraderSpiOnRspQryInvestorPositionCombineDetailPacket { p_investor_position_combine_detail:p_investor_position_combine_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cfmmc_trading_account_key(&mut self, p_cfmmc_trading_account_key : Option<&CThostFtdcCFMMCTradingAccountKeyField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryCFMMCTradingAccountKey( CThostFtdcTraderSpiOnRspQryCFMMCTradingAccountKeyPacket { p_cfmmc_trading_account_key:p_cfmmc_trading_account_key.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_e_warrant_offset(&mut self, p_e_warrant_offset : Option<&CThostFtdcEWarrantOffsetField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryEWarrantOffset( CThostFtdcTraderSpiOnRspQryEWarrantOffsetPacket { p_e_warrant_offset:p_e_warrant_offset.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_product_group_margin(&mut self, p_investor_product_group_margin : Option<&CThostFtdcInvestorProductGroupMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorProductGroupMargin( CThostFtdcTraderSpiOnRspQryInvestorProductGroupMarginPacket { p_investor_product_group_margin:p_investor_product_group_margin.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exchange_margin_rate(&mut self, p_exchange_margin_rate : Option<&CThostFtdcExchangeMarginRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryExchangeMarginRate( CThostFtdcTraderSpiOnRspQryExchangeMarginRatePacket { p_exchange_margin_rate:p_exchange_margin_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, p_exchange_margin_rate_adjust : Option<&CThostFtdcExchangeMarginRateAdjustField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryExchangeMarginRateAdjust( CThostFtdcTraderSpiOnRspQryExchangeMarginRateAdjustPacket { p_exchange_margin_rate_adjust:p_exchange_margin_rate_adjust.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exchange_rate(&mut self, p_exchange_rate : Option<&CThostFtdcExchangeRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryExchangeRate( CThostFtdcTraderSpiOnRspQryExchangeRatePacket { p_exchange_rate:p_exchange_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_sec_agent_acid_map(&mut self, p_sec_agent_acid_map : Option<&CThostFtdcSecAgentACIDMapField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySecAgentACIDMap( CThostFtdcTraderSpiOnRspQrySecAgentACIDMapPacket { p_sec_agent_acid_map:p_sec_agent_acid_map.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_product_exch_rate(&mut self, p_product_exch_rate : Option<&CThostFtdcProductExchRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryProductExchRate( CThostFtdcTraderSpiOnRspQryProductExchRatePacket { p_product_exch_rate:p_product_exch_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_product_group(&mut self, p_product_group : Option<&CThostFtdcProductGroupField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryProductGroup( CThostFtdcTraderSpiOnRspQryProductGroupPacket { p_product_group:p_product_group.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_mm_instrument_commission_rate(&mut self, p_mm_instrument_commission_rate : Option<&CThostFtdcMMInstrumentCommissionRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryMMInstrumentCommissionRate( CThostFtdcTraderSpiOnRspQryMMInstrumentCommissionRatePacket { p_mm_instrument_commission_rate:p_mm_instrument_commission_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_mm_option_instr_comm_rate(&mut self, p_mm_option_instr_comm_rate : Option<&CThostFtdcMMOptionInstrCommRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryMMOptionInstrCommRate( CThostFtdcTraderSpiOnRspQryMMOptionInstrCommRatePacket { p_mm_option_instr_comm_rate:p_mm_option_instr_comm_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_instrument_order_comm_rate(&mut self, p_instrument_order_comm_rate : Option<&CThostFtdcInstrumentOrderCommRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInstrumentOrderCommRate( CThostFtdcTraderSpiOnRspQryInstrumentOrderCommRatePacket { p_instrument_order_comm_rate:p_instrument_order_comm_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_sec_agent_trading_account(&mut self, p_trading_account : Option<&CThostFtdcTradingAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySecAgentTradingAccount( CThostFtdcTraderSpiOnRspQrySecAgentTradingAccountPacket { p_trading_account:p_trading_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_sec_agent_check_mode(&mut self, p_sec_agent_check_mode : Option<&CThostFtdcSecAgentCheckModeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySecAgentCheckMode( CThostFtdcTraderSpiOnRspQrySecAgentCheckModePacket { p_sec_agent_check_mode:p_sec_agent_check_mode.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_sec_agent_trade_info(&mut self, p_sec_agent_trade_info : Option<&CThostFtdcSecAgentTradeInfoField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySecAgentTradeInfo( CThostFtdcTraderSpiOnRspQrySecAgentTradeInfoPacket { p_sec_agent_trade_info:p_sec_agent_trade_info.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_option_instr_trade_cost(&mut self, p_option_instr_trade_cost : Option<&CThostFtdcOptionInstrTradeCostField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryOptionInstrTradeCost( CThostFtdcTraderSpiOnRspQryOptionInstrTradeCostPacket { p_option_instr_trade_cost:p_option_instr_trade_cost.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_option_instr_comm_rate(&mut self, p_option_instr_comm_rate : Option<&CThostFtdcOptionInstrCommRateField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryOptionInstrCommRate( CThostFtdcTraderSpiOnRspQryOptionInstrCommRatePacket { p_option_instr_comm_rate:p_option_instr_comm_rate.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exec_order(&mut self, p_exec_order : Option<&CThostFtdcExecOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryExecOrder( CThostFtdcTraderSpiOnRspQryExecOrderPacket { p_exec_order:p_exec_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_for_quote(&mut self, p_for_quote : Option<&CThostFtdcForQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryForQuote( CThostFtdcTraderSpiOnRspQryForQuotePacket { p_for_quote:p_for_quote.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_quote(&mut self, p_quote : Option<&CThostFtdcQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryQuote( CThostFtdcTraderSpiOnRspQryQuotePacket { p_quote:p_quote.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_option_self_close(&mut self, p_option_self_close : Option<&CThostFtdcOptionSelfCloseField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryOptionSelfClose( CThostFtdcTraderSpiOnRspQryOptionSelfClosePacket { p_option_self_close:p_option_self_close.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_invest_unit(&mut self, p_invest_unit : Option<&CThostFtdcInvestUnitField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestUnit( CThostFtdcTraderSpiOnRspQryInvestUnitPacket { p_invest_unit:p_invest_unit.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_instrument_guard(&mut self, p_comb_instrument_guard : Option<&CThostFtdcCombInstrumentGuardField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryCombInstrumentGuard( CThostFtdcTraderSpiOnRspQryCombInstrumentGuardPacket { p_comb_instrument_guard:p_comb_instrument_guard.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_action(&mut self, p_comb_action : Option<&CThostFtdcCombActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryCombAction( CThostFtdcTraderSpiOnRspQryCombActionPacket { p_comb_action:p_comb_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_transfer_serial(&mut self, p_transfer_serial : Option<&CThostFtdcTransferSerialField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTransferSerial( CThostFtdcTraderSpiOnRspQryTransferSerialPacket { p_transfer_serial:p_transfer_serial.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_accountregister(&mut self, p_accountregister : Option<&CThostFtdcAccountregisterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryAccountregister( CThostFtdcTraderSpiOnRspQryAccountregisterPacket { p_accountregister:p_accountregister.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_error(&mut self, p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspError( CThostFtdcTraderSpiOnRspErrorPacket { p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rtn_order(&mut self, p_order : Option<&CThostFtdcOrderField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnOrder( CThostFtdcTraderSpiOnRtnOrderPacket { p_order:p_order.cloned() } ))
                }
            fn on_rtn_trade(&mut self, p_trade : Option<&CThostFtdcTradeField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnTrade( CThostFtdcTraderSpiOnRtnTradePacket { p_trade:p_trade.cloned() } ))
                }
            fn on_err_rtn_order_insert(&mut self, p_input_order : Option<&CThostFtdcInputOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnOrderInsert( CThostFtdcTraderSpiOnErrRtnOrderInsertPacket { p_input_order:p_input_order.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_order_action(&mut self, p_order_action : Option<&CThostFtdcOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnOrderAction( CThostFtdcTraderSpiOnErrRtnOrderActionPacket { p_order_action:p_order_action.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rtn_instrument_status(&mut self, p_instrument_status : Option<&CThostFtdcInstrumentStatusField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnInstrumentStatus( CThostFtdcTraderSpiOnRtnInstrumentStatusPacket { p_instrument_status:p_instrument_status.cloned() } ))
                }
            fn on_rtn_bulletin(&mut self, p_bulletin : Option<&CThostFtdcBulletinField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnBulletin( CThostFtdcTraderSpiOnRtnBulletinPacket { p_bulletin:p_bulletin.cloned() } ))
                }
            fn on_rtn_trading_notice(&mut self, p_trading_notice_info : Option<&CThostFtdcTradingNoticeInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnTradingNotice( CThostFtdcTraderSpiOnRtnTradingNoticePacket { p_trading_notice_info:p_trading_notice_info.cloned() } ))
                }
            fn on_rtn_error_conditional_order(&mut self, p_error_conditional_order : Option<&CThostFtdcErrorConditionalOrderField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnErrorConditionalOrder( CThostFtdcTraderSpiOnRtnErrorConditionalOrderPacket { p_error_conditional_order:p_error_conditional_order.cloned() } ))
                }
            fn on_rtn_exec_order(&mut self, p_exec_order : Option<&CThostFtdcExecOrderField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnExecOrder( CThostFtdcTraderSpiOnRtnExecOrderPacket { p_exec_order:p_exec_order.cloned() } ))
                }
            fn on_err_rtn_exec_order_insert(&mut self, p_input_exec_order : Option<&CThostFtdcInputExecOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnExecOrderInsert( CThostFtdcTraderSpiOnErrRtnExecOrderInsertPacket { p_input_exec_order:p_input_exec_order.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_exec_order_action(&mut self, p_exec_order_action : Option<&CThostFtdcExecOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnExecOrderAction( CThostFtdcTraderSpiOnErrRtnExecOrderActionPacket { p_exec_order_action:p_exec_order_action.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_for_quote_insert(&mut self, p_input_for_quote : Option<&CThostFtdcInputForQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnForQuoteInsert( CThostFtdcTraderSpiOnErrRtnForQuoteInsertPacket { p_input_for_quote:p_input_for_quote.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rtn_quote(&mut self, p_quote : Option<&CThostFtdcQuoteField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnQuote( CThostFtdcTraderSpiOnRtnQuotePacket { p_quote:p_quote.cloned() } ))
                }
            fn on_err_rtn_quote_insert(&mut self, p_input_quote : Option<&CThostFtdcInputQuoteField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnQuoteInsert( CThostFtdcTraderSpiOnErrRtnQuoteInsertPacket { p_input_quote:p_input_quote.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_quote_action(&mut self, p_quote_action : Option<&CThostFtdcQuoteActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnQuoteAction( CThostFtdcTraderSpiOnErrRtnQuoteActionPacket { p_quote_action:p_quote_action.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rtn_for_quote_rsp(&mut self, p_for_quote_rsp : Option<&CThostFtdcForQuoteRspField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnForQuoteRsp( CThostFtdcTraderSpiOnRtnForQuoteRspPacket { p_for_quote_rsp:p_for_quote_rsp.cloned() } ))
                }
            fn on_rtn_cfmmc_trading_account_token(&mut self, p_cfmmc_trading_account_token : Option<&CThostFtdcCFMMCTradingAccountTokenField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnCFMMCTradingAccountToken( CThostFtdcTraderSpiOnRtnCFMMCTradingAccountTokenPacket { p_cfmmc_trading_account_token:p_cfmmc_trading_account_token.cloned() } ))
                }
            fn on_err_rtn_batch_order_action(&mut self, p_batch_order_action : Option<&CThostFtdcBatchOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnBatchOrderAction( CThostFtdcTraderSpiOnErrRtnBatchOrderActionPacket { p_batch_order_action:p_batch_order_action.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rtn_option_self_close(&mut self, p_option_self_close : Option<&CThostFtdcOptionSelfCloseField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnOptionSelfClose( CThostFtdcTraderSpiOnRtnOptionSelfClosePacket { p_option_self_close:p_option_self_close.cloned() } ))
                }
            fn on_err_rtn_option_self_close_insert(&mut self, p_input_option_self_close : Option<&CThostFtdcInputOptionSelfCloseField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnOptionSelfCloseInsert( CThostFtdcTraderSpiOnErrRtnOptionSelfCloseInsertPacket { p_input_option_self_close:p_input_option_self_close.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_option_self_close_action(&mut self, p_option_self_close_action : Option<&CThostFtdcOptionSelfCloseActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnOptionSelfCloseAction( CThostFtdcTraderSpiOnErrRtnOptionSelfCloseActionPacket { p_option_self_close_action:p_option_self_close_action.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rtn_comb_action(&mut self, p_comb_action : Option<&CThostFtdcCombActionField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnCombAction( CThostFtdcTraderSpiOnRtnCombActionPacket { p_comb_action:p_comb_action.cloned() } ))
                }
            fn on_err_rtn_comb_action_insert(&mut self, p_input_comb_action : Option<&CThostFtdcInputCombActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnCombActionInsert( CThostFtdcTraderSpiOnErrRtnCombActionInsertPacket { p_input_comb_action:p_input_comb_action.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rsp_qry_contract_bank(&mut self, p_contract_bank : Option<&CThostFtdcContractBankField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryContractBank( CThostFtdcTraderSpiOnRspQryContractBankPacket { p_contract_bank:p_contract_bank.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_parked_order(&mut self, p_parked_order : Option<&CThostFtdcParkedOrderField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryParkedOrder( CThostFtdcTraderSpiOnRspQryParkedOrderPacket { p_parked_order:p_parked_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_parked_order_action(&mut self, p_parked_order_action : Option<&CThostFtdcParkedOrderActionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryParkedOrderAction( CThostFtdcTraderSpiOnRspQryParkedOrderActionPacket { p_parked_order_action:p_parked_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_notice(&mut self, p_trading_notice : Option<&CThostFtdcTradingNoticeField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryTradingNotice( CThostFtdcTraderSpiOnRspQryTradingNoticePacket { p_trading_notice:p_trading_notice.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_broker_trading_params(&mut self, p_broker_trading_params : Option<&CThostFtdcBrokerTradingParamsField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryBrokerTradingParams( CThostFtdcTraderSpiOnRspQryBrokerTradingParamsPacket { p_broker_trading_params:p_broker_trading_params.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_broker_trading_algos(&mut self, p_broker_trading_algos : Option<&CThostFtdcBrokerTradingAlgosField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryBrokerTradingAlgos( CThostFtdcTraderSpiOnRspQryBrokerTradingAlgosPacket { p_broker_trading_algos:p_broker_trading_algos.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_query_cfmmc_trading_account_token(&mut self, p_query_cfmmc_trading_account_token : Option<&CThostFtdcQueryCFMMCTradingAccountTokenField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQueryCFMMCTradingAccountToken( CThostFtdcTraderSpiOnRspQueryCFMMCTradingAccountTokenPacket { p_query_cfmmc_trading_account_token:p_query_cfmmc_trading_account_token.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rtn_from_bank_to_future_by_bank(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnFromBankToFutureByBank( CThostFtdcTraderSpiOnRtnFromBankToFutureByBankPacket { p_rsp_transfer:p_rsp_transfer.cloned() } ))
                }
            fn on_rtn_from_future_to_bank_by_bank(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnFromFutureToBankByBank( CThostFtdcTraderSpiOnRtnFromFutureToBankByBankPacket { p_rsp_transfer:p_rsp_transfer.cloned() } ))
                }
            fn on_rtn_repeal_from_bank_to_future_by_bank(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnRepealFromBankToFutureByBank( CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByBankPacket { p_rsp_repeal:p_rsp_repeal.cloned() } ))
                }
            fn on_rtn_repeal_from_future_to_bank_by_bank(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnRepealFromFutureToBankByBank( CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByBankPacket { p_rsp_repeal:p_rsp_repeal.cloned() } ))
                }
            fn on_rtn_from_bank_to_future_by_future(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnFromBankToFutureByFuture( CThostFtdcTraderSpiOnRtnFromBankToFutureByFuturePacket { p_rsp_transfer:p_rsp_transfer.cloned() } ))
                }
            fn on_rtn_from_future_to_bank_by_future(&mut self, p_rsp_transfer : Option<&CThostFtdcRspTransferField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnFromFutureToBankByFuture( CThostFtdcTraderSpiOnRtnFromFutureToBankByFuturePacket { p_rsp_transfer:p_rsp_transfer.cloned() } ))
                }
            fn on_rtn_repeal_from_bank_to_future_by_future_manual(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnRepealFromBankToFutureByFutureManual( CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByFutureManualPacket { p_rsp_repeal:p_rsp_repeal.cloned() } ))
                }
            fn on_rtn_repeal_from_future_to_bank_by_future_manual(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnRepealFromFutureToBankByFutureManual( CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByFutureManualPacket { p_rsp_repeal:p_rsp_repeal.cloned() } ))
                }
            fn on_rtn_query_bank_balance_by_future(&mut self, p_notify_query_account : Option<&CThostFtdcNotifyQueryAccountField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnQueryBankBalanceByFuture( CThostFtdcTraderSpiOnRtnQueryBankBalanceByFuturePacket { p_notify_query_account:p_notify_query_account.cloned() } ))
                }
            fn on_err_rtn_bank_to_future_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnBankToFutureByFuture( CThostFtdcTraderSpiOnErrRtnBankToFutureByFuturePacket { p_req_transfer:p_req_transfer.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_future_to_bank_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnFutureToBankByFuture( CThostFtdcTraderSpiOnErrRtnFutureToBankByFuturePacket { p_req_transfer:p_req_transfer.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_repeal_bank_to_future_by_future_manual(&mut self, p_req_repeal : Option<&CThostFtdcReqRepealField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnRepealBankToFutureByFutureManual( CThostFtdcTraderSpiOnErrRtnRepealBankToFutureByFutureManualPacket { p_req_repeal:p_req_repeal.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_repeal_future_to_bank_by_future_manual(&mut self, p_req_repeal : Option<&CThostFtdcReqRepealField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnRepealFutureToBankByFutureManual( CThostFtdcTraderSpiOnErrRtnRepealFutureToBankByFutureManualPacket { p_req_repeal:p_req_repeal.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_err_rtn_query_bank_balance_by_future(&mut self, p_req_query_account : Option<&CThostFtdcReqQueryAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnErrRtnQueryBankBalanceByFuture( CThostFtdcTraderSpiOnErrRtnQueryBankBalanceByFuturePacket { p_req_query_account:p_req_query_account.cloned(),p_rsp_info:p_rsp_info.cloned() } ))
                }
            fn on_rtn_repeal_from_bank_to_future_by_future(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnRepealFromBankToFutureByFuture( CThostFtdcTraderSpiOnRtnRepealFromBankToFutureByFuturePacket { p_rsp_repeal:p_rsp_repeal.cloned() } ))
                }
            fn on_rtn_repeal_from_future_to_bank_by_future(&mut self, p_rsp_repeal : Option<&CThostFtdcRspRepealField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnRepealFromFutureToBankByFuture( CThostFtdcTraderSpiOnRtnRepealFromFutureToBankByFuturePacket { p_rsp_repeal:p_rsp_repeal.cloned() } ))
                }
            fn on_rsp_from_bank_to_future_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspFromBankToFutureByFuture( CThostFtdcTraderSpiOnRspFromBankToFutureByFuturePacket { p_req_transfer:p_req_transfer.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_from_future_to_bank_by_future(&mut self, p_req_transfer : Option<&CThostFtdcReqTransferField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspFromFutureToBankByFuture( CThostFtdcTraderSpiOnRspFromFutureToBankByFuturePacket { p_req_transfer:p_req_transfer.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_query_bank_account_money_by_future(&mut self, p_req_query_account : Option<&CThostFtdcReqQueryAccountField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQueryBankAccountMoneyByFuture( CThostFtdcTraderSpiOnRspQueryBankAccountMoneyByFuturePacket { p_req_query_account:p_req_query_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rtn_open_account_by_bank(&mut self, p_open_account : Option<&CThostFtdcOpenAccountField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnOpenAccountByBank( CThostFtdcTraderSpiOnRtnOpenAccountByBankPacket { p_open_account:p_open_account.cloned() } ))
                }
            fn on_rtn_cancel_account_by_bank(&mut self, p_cancel_account : Option<&CThostFtdcCancelAccountField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnCancelAccountByBank( CThostFtdcTraderSpiOnRtnCancelAccountByBankPacket { p_cancel_account:p_cancel_account.cloned() } ))
                }
            fn on_rtn_change_account_by_bank(&mut self, p_change_account : Option<&CThostFtdcChangeAccountField>) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRtnChangeAccountByBank( CThostFtdcTraderSpiOnRtnChangeAccountByBankPacket { p_change_account:p_change_account.cloned() } ))
                }
            fn on_rsp_qry_classified_instrument(&mut self, p_instrument : Option<&CThostFtdcInstrumentField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryClassifiedInstrument( CThostFtdcTraderSpiOnRspQryClassifiedInstrumentPacket { p_instrument:p_instrument.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_promotion_param(&mut self, p_comb_promotion_param : Option<&CThostFtdcCombPromotionParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryCombPromotionParam( CThostFtdcTraderSpiOnRspQryCombPromotionParamPacket { p_comb_promotion_param:p_comb_promotion_param.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_risk_settle_invst_position(&mut self, p_risk_settle_invst_position : Option<&CThostFtdcRiskSettleInvstPositionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRiskSettleInvstPosition( CThostFtdcTraderSpiOnRspQryRiskSettleInvstPositionPacket { p_risk_settle_invst_position:p_risk_settle_invst_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_risk_settle_product_status(&mut self, p_risk_settle_product_status : Option<&CThostFtdcRiskSettleProductStatusField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRiskSettleProductStatus( CThostFtdcTraderSpiOnRspQryRiskSettleProductStatusPacket { p_risk_settle_product_status:p_risk_settle_product_status.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_future_parameter(&mut self, p_spbm_future_parameter : Option<&CThostFtdcSPBMFutureParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMFutureParameter( CThostFtdcTraderSpiOnRspQrySPBMFutureParameterPacket { p_spbm_future_parameter:p_spbm_future_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_option_parameter(&mut self, p_spbm_option_parameter : Option<&CThostFtdcSPBMOptionParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMOptionParameter( CThostFtdcTraderSpiOnRspQrySPBMOptionParameterPacket { p_spbm_option_parameter:p_spbm_option_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_intra_parameter(&mut self, p_spbm_intra_parameter : Option<&CThostFtdcSPBMIntraParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMIntraParameter( CThostFtdcTraderSpiOnRspQrySPBMIntraParameterPacket { p_spbm_intra_parameter:p_spbm_intra_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_inter_parameter(&mut self, p_spbm_inter_parameter : Option<&CThostFtdcSPBMInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMInterParameter( CThostFtdcTraderSpiOnRspQrySPBMInterParameterPacket { p_spbm_inter_parameter:p_spbm_inter_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_portf_definition(&mut self, p_spbm_portf_definition : Option<&CThostFtdcSPBMPortfDefinitionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMPortfDefinition( CThostFtdcTraderSpiOnRspQrySPBMPortfDefinitionPacket { p_spbm_portf_definition:p_spbm_portf_definition.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_investor_portf_def(&mut self, p_spbm_investor_portf_def : Option<&CThostFtdcSPBMInvestorPortfDefField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMInvestorPortfDef( CThostFtdcTraderSpiOnRspQrySPBMInvestorPortfDefPacket { p_spbm_investor_portf_def:p_spbm_investor_portf_def.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_portf_margin_ratio(&mut self, p_investor_portf_margin_ratio : Option<&CThostFtdcInvestorPortfMarginRatioField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorPortfMarginRatio( CThostFtdcTraderSpiOnRspQryInvestorPortfMarginRatioPacket { p_investor_portf_margin_ratio:p_investor_portf_margin_ratio.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_prod_spbm_detail(&mut self, p_investor_prod_spbm_detail : Option<&CThostFtdcInvestorProdSPBMDetailField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorProdSPBMDetail( CThostFtdcTraderSpiOnRspQryInvestorProdSPBMDetailPacket { p_investor_prod_spbm_detail:p_investor_prod_spbm_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_commodity_spmm_margin(&mut self, p_investor_commodity_spmm_margin : Option<&CThostFtdcInvestorCommoditySPMMMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorCommoditySPMMMargin( CThostFtdcTraderSpiOnRspQryInvestorCommoditySPMMMarginPacket { p_investor_commodity_spmm_margin:p_investor_commodity_spmm_margin.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_commodity_group_spmm_margin(&mut self, p_investor_commodity_group_spmm_margin : Option<&CThostFtdcInvestorCommodityGroupSPMMMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorCommodityGroupSPMMMargin( CThostFtdcTraderSpiOnRspQryInvestorCommodityGroupSPMMMarginPacket { p_investor_commodity_group_spmm_margin:p_investor_commodity_group_spmm_margin.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spmm_inst_param(&mut self, p_spmm_inst_param : Option<&CThostFtdcSPMMInstParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPMMInstParam( CThostFtdcTraderSpiOnRspQrySPMMInstParamPacket { p_spmm_inst_param:p_spmm_inst_param.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spmm_product_param(&mut self, p_spmm_product_param : Option<&CThostFtdcSPMMProductParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPMMProductParam( CThostFtdcTraderSpiOnRspQrySPMMProductParamPacket { p_spmm_product_param:p_spmm_product_param.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_spbm_add_on_inter_parameter(&mut self, p_spbm_add_on_inter_parameter : Option<&CThostFtdcSPBMAddOnInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQrySPBMAddOnInterParameter( CThostFtdcTraderSpiOnRspQrySPBMAddOnInterParameterPacket { p_spbm_add_on_inter_parameter:p_spbm_add_on_inter_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rcams_comb_product_info(&mut self, p_rcams_comb_product_info : Option<&CThostFtdcRCAMSCombProductInfoField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRCAMSCombProductInfo( CThostFtdcTraderSpiOnRspQryRCAMSCombProductInfoPacket { p_rcams_comb_product_info:p_rcams_comb_product_info.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rcams_instr_parameter(&mut self, p_rcams_instr_parameter : Option<&CThostFtdcRCAMSInstrParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRCAMSInstrParameter( CThostFtdcTraderSpiOnRspQryRCAMSInstrParameterPacket { p_rcams_instr_parameter:p_rcams_instr_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rcams_intra_parameter(&mut self, p_rcams_intra_parameter : Option<&CThostFtdcRCAMSIntraParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRCAMSIntraParameter( CThostFtdcTraderSpiOnRspQryRCAMSIntraParameterPacket { p_rcams_intra_parameter:p_rcams_intra_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rcams_inter_parameter(&mut self, p_rcams_inter_parameter : Option<&CThostFtdcRCAMSInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRCAMSInterParameter( CThostFtdcTraderSpiOnRspQryRCAMSInterParameterPacket { p_rcams_inter_parameter:p_rcams_inter_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rcams_short_opt_adjust_param(&mut self, p_rcams_short_opt_adjust_param : Option<&CThostFtdcRCAMSShortOptAdjustParamField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRCAMSShortOptAdjustParam( CThostFtdcTraderSpiOnRspQryRCAMSShortOptAdjustParamPacket { p_rcams_short_opt_adjust_param:p_rcams_short_opt_adjust_param.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rcams_investor_comb_position(&mut self, p_rcams_investor_comb_position : Option<&CThostFtdcRCAMSInvestorCombPositionField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRCAMSInvestorCombPosition( CThostFtdcTraderSpiOnRspQryRCAMSInvestorCombPositionPacket { p_rcams_investor_comb_position:p_rcams_investor_comb_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_prod_rcams_margin(&mut self, p_investor_prod_rcams_margin : Option<&CThostFtdcInvestorProdRCAMSMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorProdRCAMSMargin( CThostFtdcTraderSpiOnRspQryInvestorProdRCAMSMarginPacket { p_investor_prod_rcams_margin:p_investor_prod_rcams_margin.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rule_instr_parameter(&mut self, p_rule_instr_parameter : Option<&CThostFtdcRULEInstrParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRULEInstrParameter( CThostFtdcTraderSpiOnRspQryRULEInstrParameterPacket { p_rule_instr_parameter:p_rule_instr_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rule_intra_parameter(&mut self, p_rule_intra_parameter : Option<&CThostFtdcRULEIntraParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRULEIntraParameter( CThostFtdcTraderSpiOnRspQryRULEIntraParameterPacket { p_rule_intra_parameter:p_rule_intra_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rule_inter_parameter(&mut self, p_rule_inter_parameter : Option<&CThostFtdcRULEInterParameterField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryRULEInterParameter( CThostFtdcTraderSpiOnRspQryRULEInterParameterPacket { p_rule_inter_parameter:p_rule_inter_parameter.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_prod_rule_margin(&mut self, p_investor_prod_rule_margin : Option<&CThostFtdcInvestorProdRULEMarginField>,p_rsp_info : Option<&CThostFtdcRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(CThostFtdcTraderSpiOutput::OnRspQryInvestorProdRULEMargin( CThostFtdcTraderSpiOnRspQryInvestorProdRULEMarginPacket { p_investor_prod_rule_margin:p_investor_prod_rule_margin.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
             }

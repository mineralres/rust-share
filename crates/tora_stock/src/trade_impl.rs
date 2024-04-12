impl TORASTOCKAPI_CTORATstpTraderApi {
                            pub fn release(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_Release)(self as *mut TORASTOCKAPI_CTORATstpTraderApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_Init)(self as *mut TORASTOCKAPI_CTORATstpTraderApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_Join)(self as *mut TORASTOCKAPI_CTORATstpTraderApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: &str) -> () {
                                    let psz_front_address = CString::new(psz_front_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_RegisterFront)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             psz_front_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: &str) -> () {
                                    let psz_ns_address = CString::new(psz_ns_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_RegisterNameServer)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             psz_ns_address.as_ptr() as *mut c_char)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn TORASTOCKAPI_CTORATstpTraderSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &TORASTOCKAPI_CTORA_TSTP_TRADER_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_RegisterSpi)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_spi as * mut TORASTOCKAPI_CTORATstpTraderSpi)
                                            }
                                }
                            pub fn subscribe_private_topic(&mut self, n_resume_type: TORASTOCKAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_SubscribePrivateTopic)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn subscribe_public_topic(&mut self, n_resume_type: TORASTOCKAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_SubscribePublicTopic)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn req_get_connection_info(&mut self, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqGetConnectionInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut TORASTOCKAPI_CTORATstpReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqUserLogin)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_user_login_field as * mut TORASTOCKAPI_CTORATstpReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout_field: &mut TORASTOCKAPI_CTORATstpUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqUserLogout)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_user_logout_field as * mut TORASTOCKAPI_CTORATstpUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_password_update(&mut self, p_user_password_update_field: &mut TORASTOCKAPI_CTORATstpUserPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqUserPasswordUpdate)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_user_password_update_field as * mut TORASTOCKAPI_CTORATstpUserPasswordUpdateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_input_device_serial(&mut self, p_req_input_device_serial_field: &mut TORASTOCKAPI_CTORATstpReqInputDeviceSerialField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInputDeviceSerial)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_input_device_serial_field as * mut TORASTOCKAPI_CTORATstpReqInputDeviceSerialField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_insert(&mut self, p_input_order_field: &mut TORASTOCKAPI_CTORATstpInputOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqOrderInsert)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_order_field as * mut TORASTOCKAPI_CTORATstpInputOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_action(&mut self, p_input_order_action_field: &mut TORASTOCKAPI_CTORATstpInputOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqOrderAction)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_order_action_field as * mut TORASTOCKAPI_CTORATstpInputOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_insert(&mut self, p_input_cond_order_field: &mut TORASTOCKAPI_CTORATstpInputCondOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqCondOrderInsert)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_cond_order_field as * mut TORASTOCKAPI_CTORATstpInputCondOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_action(&mut self, p_input_cond_order_action_field: &mut TORASTOCKAPI_CTORATstpInputCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqCondOrderAction)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_cond_order_action_field as * mut TORASTOCKAPI_CTORATstpInputCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_nego_order_insert(&mut self, p_input_nego_order_field: &mut TORASTOCKAPI_CTORATstpInputNegoOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqNegoOrderInsert)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_nego_order_field as * mut TORASTOCKAPI_CTORATstpInputNegoOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_nego_order_action(&mut self, p_input_nego_order_action_field: &mut TORASTOCKAPI_CTORATstpInputNegoOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqNegoOrderAction)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_nego_order_action_field as * mut TORASTOCKAPI_CTORATstpInputNegoOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_fund(&mut self, p_input_transfer_fund_field: &mut TORASTOCKAPI_CTORATstpInputTransferFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqTransferFund)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_transfer_fund_field as * mut TORASTOCKAPI_CTORATstpInputTransferFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_position(&mut self, p_input_transfer_position_field: &mut TORASTOCKAPI_CTORATstpInputTransferPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqTransferPosition)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_transfer_position_field as * mut TORASTOCKAPI_CTORATstpInputTransferPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_jz_fund(&mut self, p_req_inquiry_jz_fund_field: &mut TORASTOCKAPI_CTORATstpReqInquiryJZFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryJZFund)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_jz_fund_field as * mut TORASTOCKAPI_CTORATstpReqInquiryJZFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_bank_account_fund(&mut self, p_req_inquiry_bank_account_fund_field: &mut TORASTOCKAPI_CTORATstpReqInquiryBankAccountFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryBankAccountFund)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_bank_account_fund_field as * mut TORASTOCKAPI_CTORATstpReqInquiryBankAccountFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_max_order_volume(&mut self, p_req_inquiry_max_order_volume_field: &mut TORASTOCKAPI_CTORATstpReqInquiryMaxOrderVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryMaxOrderVolume)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_max_order_volume_field as * mut TORASTOCKAPI_CTORATstpReqInquiryMaxOrderVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_trade_concentration(&mut self, p_inquiry_trade_concentration_field: &mut TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryTradeConcentration)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_inquiry_trade_concentration_field as * mut TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_modify_open_pos_cost(&mut self, p_req_modify_open_pos_cost_field: &mut TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqModifyOpenPosCost)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_modify_open_pos_cost_field as * mut TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_input_node_fund_assignment(&mut self, p_input_node_fund_assignment_field: &mut TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInputNodeFundAssignment)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_node_fund_assignment_field as * mut TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_node_fund_assignment(&mut self, p_req_inquiry_node_fund_assignment_field: &mut TORASTOCKAPI_CTORATstpReqInquiryNodeFundAssignmentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryNodeFundAssignment)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_node_fund_assignment_field as * mut TORASTOCKAPI_CTORATstpReqInquiryNodeFundAssignmentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange(&mut self, p_qry_exchange_field: &mut TORASTOCKAPI_CTORATstpQryExchangeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryExchange)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_exchange_field as * mut TORASTOCKAPI_CTORATstpQryExchangeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_security(&mut self, p_qry_security_field: &mut TORASTOCKAPI_CTORATstpQrySecurityField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQrySecurity)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_security_field as * mut TORASTOCKAPI_CTORATstpQrySecurityField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_info(&mut self, p_qry_ipo_info_field: &mut TORASTOCKAPI_CTORATstpQryIPOInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPOInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_info_field as * mut TORASTOCKAPI_CTORATstpQryIPOInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_user(&mut self, p_qry_user_field: &mut TORASTOCKAPI_CTORATstpQryUserField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryUser)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_user_field as * mut TORASTOCKAPI_CTORATstpQryUserField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor(&mut self, p_qry_investor_field: &mut TORASTOCKAPI_CTORATstpQryInvestorField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestor)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_field as * mut TORASTOCKAPI_CTORATstpQryInvestorField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_shareholder_account(&mut self, p_qry_shareholder_account_field: &mut TORASTOCKAPI_CTORATstpQryShareholderAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryShareholderAccount)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_shareholder_account_field as * mut TORASTOCKAPI_CTORATstpQryShareholderAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rational_info(&mut self, p_qry_rational_info_field: &mut TORASTOCKAPI_CTORATstpQryRationalInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryRationalInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_rational_info_field as * mut TORASTOCKAPI_CTORATstpQryRationalInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order(&mut self, p_qry_order_field: &mut TORASTOCKAPI_CTORATstpQryOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryOrder)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_order_field as * mut TORASTOCKAPI_CTORATstpQryOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_action(&mut self, p_qry_order_action_field: &mut TORASTOCKAPI_CTORATstpQryOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryOrderAction)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_order_action_field as * mut TORASTOCKAPI_CTORATstpQryOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trade(&mut self, p_qry_trade_field: &mut TORASTOCKAPI_CTORATstpQryTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryTrade)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trade_field as * mut TORASTOCKAPI_CTORATstpQryTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_account(&mut self, p_qry_trading_account_field: &mut TORASTOCKAPI_CTORATstpQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryTradingAccount)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trading_account_field as * mut TORASTOCKAPI_CTORATstpQryTradingAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position(&mut self, p_qry_position_field: &mut TORASTOCKAPI_CTORATstpQryPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPosition)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_position_field as * mut TORASTOCKAPI_CTORATstpQryPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_fee(&mut self, p_qry_trading_fee_field: &mut TORASTOCKAPI_CTORATstpQryTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryTradingFee)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trading_fee_field as * mut TORASTOCKAPI_CTORATstpQryTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_trading_fee(&mut self, p_qry_investor_trading_fee_field: &mut TORASTOCKAPI_CTORATstpQryInvestorTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestorTradingFee)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_trading_fee_field as * mut TORASTOCKAPI_CTORATstpQryInvestorTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_quota(&mut self, p_qry_ipo_quota_field: &mut TORASTOCKAPI_CTORATstpQryIPOQuotaField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPOQuota)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_quota_field as * mut TORASTOCKAPI_CTORATstpQryIPOQuotaField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_fund_detail(&mut self, p_qry_order_fund_detail_field: &mut TORASTOCKAPI_CTORATstpQryOrderFundDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryOrderFundDetail)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_order_fund_detail_field as * mut TORASTOCKAPI_CTORATstpQryOrderFundDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_fund_transfer_detail(&mut self, p_qry_fund_transfer_detail_field: &mut TORASTOCKAPI_CTORATstpQryFundTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryFundTransferDetail)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_fund_transfer_detail_field as * mut TORASTOCKAPI_CTORATstpQryFundTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position_transfer_detail(&mut self, p_qry_position_transfer_detail_field: &mut TORASTOCKAPI_CTORATstpQryPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPositionTransferDetail)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_position_transfer_detail_field as * mut TORASTOCKAPI_CTORATstpQryPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_periphery_position_transfer_detail(&mut self, p_qry_periphery_position_transfer_detail_field: &mut TORASTOCKAPI_CTORATstpQryPeripheryPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPeripheryPositionTransferDetail)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_periphery_position_transfer_detail_field as * mut TORASTOCKAPI_CTORATstpQryPeripheryPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_periphery_fund_transfer_detail(&mut self, p_qry_periphery_fund_transfer_detail_field: &mut TORASTOCKAPI_CTORATstpQryPeripheryFundTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPeripheryFundTransferDetail)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_periphery_fund_transfer_detail_field as * mut TORASTOCKAPI_CTORATstpQryPeripheryFundTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_bond_conversion_info(&mut self, p_qry_bond_conversion_info_field: &mut TORASTOCKAPI_CTORATstpQryBondConversionInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryBondConversionInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_bond_conversion_info_field as * mut TORASTOCKAPI_CTORATstpQryBondConversionInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_bond_putback_info(&mut self, p_qry_bond_putback_info_field: &mut TORASTOCKAPI_CTORATstpQryBondPutbackInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryBondPutbackInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_bond_putback_info_field as * mut TORASTOCKAPI_CTORATstpQryBondPutbackInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_cond_order_limit_param(&mut self, p_qry_investor_cond_order_limit_param_field: &mut TORASTOCKAPI_CTORATstpQryInvestorCondOrderLimitParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestorCondOrderLimitParam)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_cond_order_limit_param_field as * mut TORASTOCKAPI_CTORATstpQryInvestorCondOrderLimitParamField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_condition_order(&mut self, p_qry_condition_order_field: &mut TORASTOCKAPI_CTORATstpQryConditionOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryConditionOrder)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_condition_order_field as * mut TORASTOCKAPI_CTORATstpQryConditionOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cond_order_action(&mut self, p_qry_cond_order_action_field: &mut TORASTOCKAPI_CTORATstpQryCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryCondOrderAction)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_cond_order_action_field as * mut TORASTOCKAPI_CTORATstpQryCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_notice(&mut self, p_qry_trading_notice_field: &mut TORASTOCKAPI_CTORATstpQryTradingNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryTradingNotice)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trading_notice_field as * mut TORASTOCKAPI_CTORATstpQryTradingNoticeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_number_result(&mut self, p_qry_ipo_number_result_field: &mut TORASTOCKAPI_CTORATstpQryIPONumberResultField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPONumberResult)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_number_result_field as * mut TORASTOCKAPI_CTORATstpQryIPONumberResultField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_match_number_result(&mut self, p_qry_ipo_match_number_result_field: &mut TORASTOCKAPI_CTORATstpQryIPOMatchNumberResultField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPOMatchNumberResult)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_match_number_result_field as * mut TORASTOCKAPI_CTORATstpQryIPOMatchNumberResultField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_shareholder_spec_privilege(&mut self, p_qry_shareholder_spec_privilege_field: &mut TORASTOCKAPI_CTORATstpQryShareholderSpecPrivilegeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryShareholderSpecPrivilege)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_shareholder_spec_privilege_field as * mut TORASTOCKAPI_CTORATstpQryShareholderSpecPrivilegeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_market(&mut self, p_qry_market_field: &mut TORASTOCKAPI_CTORATstpQryMarketField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryMarket)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_market_field as * mut TORASTOCKAPI_CTORATstpQryMarketField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_etf_file(&mut self, p_qry_etf_file_field: &mut TORASTOCKAPI_CTORATstpQryETFFileField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryETFFile)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_etf_file_field as * mut TORASTOCKAPI_CTORATstpQryETFFileField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_etf_basket(&mut self, p_qry_etf_basket_field: &mut TORASTOCKAPI_CTORATstpQryETFBasketField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryETFBasket)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_etf_basket_field as * mut TORASTOCKAPI_CTORATstpQryETFBasketField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_position_limit(&mut self, p_qry_investor_position_limit_field: &mut TORASTOCKAPI_CTORATstpQryInvestorPositionLimitField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestorPositionLimit)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_position_limit_field as * mut TORASTOCKAPI_CTORATstpQryInvestorPositionLimitField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_szse_imc_params(&mut self, p_qry_szse_imc_params_field: &mut TORASTOCKAPI_CTORATstpQrySZSEImcParamsField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQrySZSEImcParams)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_szse_imc_params_field as * mut TORASTOCKAPI_CTORATstpQrySZSEImcParamsField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_szse_imc_exchange_rate(&mut self, p_qry_szse_imc_exchange_rate_field: &mut TORASTOCKAPI_CTORATstpQrySZSEImcExchangeRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQrySZSEImcExchangeRate)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_szse_imc_exchange_rate_field as * mut TORASTOCKAPI_CTORATstpQrySZSEImcExchangeRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_szsehk_price_tick_info(&mut self, p_qry_szsehk_price_tick_info_field: &mut TORASTOCKAPI_CTORATstpQrySZSEHKPriceTickInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQrySZSEHKPriceTickInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_szsehk_price_tick_info_field as * mut TORASTOCKAPI_CTORATstpQrySZSEHKPriceTickInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lof_fund_info(&mut self, p_qry_lof_fund_info_field: &mut TORASTOCKAPI_CTORATstpQryLofFundInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryLofFundInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_lof_fund_info_field as * mut TORASTOCKAPI_CTORATstpQryLofFundInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_pledge_position(&mut self, p_qry_pledge_position_field: &mut TORASTOCKAPI_CTORATstpQryPledgePositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPledgePosition)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_pledge_position_field as * mut TORASTOCKAPI_CTORATstpQryPledgePositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_pledge_info(&mut self, p_qry_pledge_info_field: &mut TORASTOCKAPI_CTORATstpQryPledgeInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPledgeInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_pledge_info_field as * mut TORASTOCKAPI_CTORATstpQryPledgeInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_system_node_info(&mut self, p_qry_system_node_info_field: &mut TORASTOCKAPI_CTORATstpQrySystemNodeInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQrySystemNodeInfo)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_system_node_info_field as * mut TORASTOCKAPI_CTORATstpQrySystemNodeInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_standard_bond_position(&mut self, p_qry_standard_bond_position_field: &mut TORASTOCKAPI_CTORATstpQryStandardBondPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryStandardBondPosition)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_standard_bond_position_field as * mut TORASTOCKAPI_CTORATstpQryStandardBondPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_prematurity_repo_order(&mut self, p_qry_prematurity_repo_order_field: &mut TORASTOCKAPI_CTORATstpQryPrematurityRepoOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryPrematurityRepoOrder)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_prematurity_repo_order_field as * mut TORASTOCKAPI_CTORATstpQryPrematurityRepoOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_nego_order(&mut self, p_qry_nego_order_field: &mut TORASTOCKAPI_CTORATstpQryNegoOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegoOrder)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_nego_order_field as * mut TORASTOCKAPI_CTORATstpQryNegoOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_nego_order_action(&mut self, p_qry_nego_order_action_field: &mut TORASTOCKAPI_CTORATstpQryNegoOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegoOrderAction)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_nego_order_action_field as * mut TORASTOCKAPI_CTORATstpQryNegoOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_nego_trade(&mut self, p_qry_nego_trade_field: &mut TORASTOCKAPI_CTORATstpQryNegoTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegoTrade)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_nego_trade_field as * mut TORASTOCKAPI_CTORATstpQryNegoTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_negotiation_param(&mut self, p_qry_negotiation_param_field: &mut TORASTOCKAPI_CTORATstpQryNegotiationParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegotiationParam)(self as *mut TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_negotiation_param_field as * mut TORASTOCKAPI_CTORATstpQryNegotiationParamField,
                                             n_request_id)
                                        }
                            }} 
                unsafe impl Send for TORASTOCKAPI_CTORATstpTraderApi {}pub trait TORASTOCKAPI_CTORATstpTraderSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_rsp_error(&mut self, p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_get_connection_info(&mut self, p_connection_info_field : Option<&TORASTOCKAPI_CTORATstpConnectionInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&TORASTOCKAPI_CTORATstpRspUserLoginField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&TORASTOCKAPI_CTORATstpUserLogoutField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&TORASTOCKAPI_CTORATstpUserPasswordUpdateField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&TORASTOCKAPI_CTORATstpRspInputDeviceSerialField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_order(&mut self, p_order_field : Option<&TORASTOCKAPI_CTORATstpOrderField>) {}
fn on_err_rtn_order_insert(&mut self, p_input_order_field : Option<&TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_trade(&mut self, p_trade_field : Option<&TORASTOCKAPI_CTORATstpTradeField>) {}
fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_order_action(&mut self, p_input_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_cond_order(&mut self, p_condition_order_field : Option<&TORASTOCKAPI_CTORATstpConditionOrderField>) {}
fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_nego_order_insert(&mut self, p_input_nego_order_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_nego_order(&mut self, p_nego_order_field : Option<&TORASTOCKAPI_CTORATstpNegoOrderField>) {}
fn on_err_rtn_nego_order_insert(&mut self, p_input_nego_order_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_nego_trade(&mut self, p_nego_trade_field : Option<&TORASTOCKAPI_CTORATstpNegoTradeField>) {}
fn on_rsp_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_market_status(&mut self, p_market_status_field : Option<&TORASTOCKAPI_CTORATstpMarketStatusField>) {}
fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_fund(&mut self, p_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpTransferFundField>) {}
fn on_rsp_transfer_position(&mut self, p_input_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_position(&mut self, p_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpTransferPositionField>) {}
fn on_rtn_periphery_transfer_position(&mut self, p_periphery_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpPeripheryTransferPositionField>) {}
fn on_rtn_periphery_transfer_fund(&mut self, p_periphery_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpPeripheryTransferFundField>) {}
fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryJZFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_trading_notice(&mut self, p_trading_notice_field : Option<&TORASTOCKAPI_CTORATstpTradingNoticeField>) {}
fn on_rsp_inquiry_max_order_volume(&mut self, p_rsp_inquiry_max_order_volume_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_trade_concentration(&mut self, p_inquiry_trade_concentration_field : Option<&TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_modify_open_pos_cost(&mut self, p_req_modify_open_pos_cost_field : Option<&TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_input_node_fund_assignment(&mut self, p_input_node_fund_assignment_field : Option<&TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_node_fund_assignment(&mut self, p_rsp_inquiry_node_fund_assignment_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_qry_exchange(&mut self, p_exchange_field : Option<&TORASTOCKAPI_CTORATstpExchangeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_security(&mut self, p_security_field : Option<&TORASTOCKAPI_CTORATstpSecurityField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_info(&mut self, p_ipo_info_field : Option<&TORASTOCKAPI_CTORATstpIPOInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_user(&mut self, p_user_field : Option<&TORASTOCKAPI_CTORATstpUserField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor(&mut self, p_investor_field : Option<&TORASTOCKAPI_CTORATstpInvestorField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account_field : Option<&TORASTOCKAPI_CTORATstpShareholderAccountField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rational_info(&mut self, p_rational_info_field : Option<&TORASTOCKAPI_CTORATstpRationalInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order(&mut self, p_order_field : Option<&TORASTOCKAPI_CTORATstpOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_action(&mut self, p_order_action_field : Option<&TORASTOCKAPI_CTORATstpOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trade(&mut self, p_trade_field : Option<&TORASTOCKAPI_CTORATstpTradeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_account(&mut self, p_trading_account_field : Option<&TORASTOCKAPI_CTORATstpTradingAccountField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position(&mut self, p_position_field : Option<&TORASTOCKAPI_CTORATstpPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_fee(&mut self, p_trading_fee_field : Option<&TORASTOCKAPI_CTORATstpTradingFeeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee_field : Option<&TORASTOCKAPI_CTORATstpInvestorTradingFeeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_quota(&mut self, p_ipo_quota_field : Option<&TORASTOCKAPI_CTORATstpIPOQuotaField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail_field : Option<&TORASTOCKAPI_CTORATstpOrderFundDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpFundTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpPositionTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_periphery_position_transfer_detail(&mut self, p_periphery_position_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_periphery_fund_transfer_detail(&mut self, p_periphery_fund_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_bond_conversion_info(&mut self, p_bond_conversion_info_field : Option<&TORASTOCKAPI_CTORATstpBondConversionInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_bond_putback_info(&mut self, p_bond_putback_info_field : Option<&TORASTOCKAPI_CTORATstpBondPutbackInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_cond_order_limit_param(&mut self, p_investor_cond_order_limit_param_field : Option<&TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_condition_order(&mut self, p_condition_order_field : Option<&TORASTOCKAPI_CTORATstpConditionOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action_field : Option<&TORASTOCKAPI_CTORATstpCondOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_notice(&mut self, p_trading_notice_field : Option<&TORASTOCKAPI_CTORATstpTradingNoticeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_number_result(&mut self, p_ipo_number_result_field : Option<&TORASTOCKAPI_CTORATstpIPONumberResultField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_match_number_result(&mut self, p_ipo_match_number_result_field : Option<&TORASTOCKAPI_CTORATstpIPOMatchNumberResultField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_shareholder_spec_privilege(&mut self, p_shareholder_spec_privilege_field : Option<&TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_market(&mut self, p_market_field : Option<&TORASTOCKAPI_CTORATstpMarketField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_etf_file(&mut self, p_etf_file_field : Option<&TORASTOCKAPI_CTORATstpETFFileField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_etf_basket(&mut self, p_etf_basket_field : Option<&TORASTOCKAPI_CTORATstpETFBasketField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_position_limit(&mut self, p_investor_position_limit_field : Option<&TORASTOCKAPI_CTORATstpInvestorPositionLimitField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_szse_imc_params(&mut self, p_szse_imc_params_field : Option<&TORASTOCKAPI_CTORATstpSZSEImcParamsField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_szse_imc_exchange_rate(&mut self, p_szse_imc_exchange_rate_field : Option<&TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_szsehk_price_tick_info(&mut self, p_szsehk_price_tick_info_field : Option<&TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lof_fund_info(&mut self, p_lof_fund_info_field : Option<&TORASTOCKAPI_CTORATstpLofFundInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_pledge_position(&mut self, p_pledge_position_field : Option<&TORASTOCKAPI_CTORATstpPledgePositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_pledge_info(&mut self, p_pledge_info_field : Option<&TORASTOCKAPI_CTORATstpPledgeInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_system_node_info(&mut self, p_system_node_info_field : Option<&TORASTOCKAPI_CTORATstpSystemNodeInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_standard_bond_position(&mut self, p_standard_bond_position_field : Option<&TORASTOCKAPI_CTORATstpStandardBondPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_prematurity_repo_order(&mut self, p_prematurity_repo_order_field : Option<&TORASTOCKAPI_CTORATstpPrematurityRepoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_nego_order(&mut self, p_nego_order_field : Option<&TORASTOCKAPI_CTORATstpNegoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_nego_order_action(&mut self, p_nego_order_action_field : Option<&TORASTOCKAPI_CTORATstpNegoOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_nego_trade(&mut self, p_nego_trade_field : Option<&TORASTOCKAPI_CTORATstpNegoTradeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_negotiation_param(&mut self, p_negotiation_param_field : Option<&TORASTOCKAPI_CTORATstpNegotiationParamField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct TORASTOCKAPI_CTORATstpTraderSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, n_reason : std::os::raw::c_int ),
                on_rsp_error: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_get_connection_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_connection_info_field : * const TORASTOCKAPI_CTORATstpConnectionInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_login: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_user_login_field : * const TORASTOCKAPI_CTORATstpRspUserLoginField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_logout: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_logout_field : * const TORASTOCKAPI_CTORATstpUserLogoutField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_password_update: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_password_update_field : * const TORASTOCKAPI_CTORATstpUserPasswordUpdateField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_input_device_serial: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_input_device_serial_field : * const TORASTOCKAPI_CTORATstpRspInputDeviceSerialField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_order_insert: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const TORASTOCKAPI_CTORATstpOrderField ),
                on_err_rtn_order_insert: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_trade: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const TORASTOCKAPI_CTORATstpTradeField ),
                on_rsp_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_insert: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_cond_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const TORASTOCKAPI_CTORATstpConditionOrderField ),
                on_err_rtn_cond_order_insert: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_cond_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_nego_order_insert: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_nego_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const TORASTOCKAPI_CTORATstpNegoOrderField ),
                on_err_rtn_nego_order_insert: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_nego_trade: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const TORASTOCKAPI_CTORATstpNegoTradeField ),
                on_rsp_nego_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_nego_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_market_status: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_status_field : * const TORASTOCKAPI_CTORATstpMarketStatusField ),
                on_rsp_transfer_fund: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_transfer_fund: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_fund: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_fund_field : * const TORASTOCKAPI_CTORATstpTransferFundField ),
                on_rsp_transfer_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_transfer_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_position_field : * const TORASTOCKAPI_CTORATstpTransferPositionField ),
                on_rtn_periphery_transfer_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_position_field : * const TORASTOCKAPI_CTORATstpPeripheryTransferPositionField ),
                on_rtn_periphery_transfer_fund: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_fund_field : * const TORASTOCKAPI_CTORATstpPeripheryTransferFundField ),
                on_rsp_inquiry_jz_fund: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const TORASTOCKAPI_CTORATstpRspInquiryJZFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_bank_account_fund: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_trading_notice: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const TORASTOCKAPI_CTORATstpTradingNoticeField ),
                on_rsp_inquiry_max_order_volume: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_max_order_volume_field : * const TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_trade_concentration: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_inquiry_trade_concentration_field : * const TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_modify_open_pos_cost: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_req_modify_open_pos_cost_field : * const TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_input_node_fund_assignment: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_node_fund_assignment_field : * const TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_node_fund_assignment: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_node_fund_assignment_field : * const TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_qry_exchange: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_exchange_field : * const TORASTOCKAPI_CTORATstpExchangeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_security: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_security_field : * const TORASTOCKAPI_CTORATstpSecurityField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_info_field : * const TORASTOCKAPI_CTORATstpIPOInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_user: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_field : * const TORASTOCKAPI_CTORATstpUserField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_field : * const TORASTOCKAPI_CTORATstpInvestorField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_shareholder_account: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_account_field : * const TORASTOCKAPI_CTORATstpShareholderAccountField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rational_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rational_info_field : * const TORASTOCKAPI_CTORATstpRationalInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const TORASTOCKAPI_CTORATstpOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_action_field : * const TORASTOCKAPI_CTORATstpOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trade: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const TORASTOCKAPI_CTORATstpTradeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_account: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_account_field : * const TORASTOCKAPI_CTORATstpTradingAccountField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_field : * const TORASTOCKAPI_CTORATstpPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_fee: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_fee_field : * const TORASTOCKAPI_CTORATstpTradingFeeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_trading_fee: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_trading_fee_field : * const TORASTOCKAPI_CTORATstpInvestorTradingFeeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_quota: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_quota_field : * const TORASTOCKAPI_CTORATstpIPOQuotaField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_fund_detail: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_fund_detail_field : * const TORASTOCKAPI_CTORATstpOrderFundDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_fund_transfer_detail: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_fund_transfer_detail_field : * const TORASTOCKAPI_CTORATstpFundTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position_transfer_detail: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_transfer_detail_field : * const TORASTOCKAPI_CTORATstpPositionTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_periphery_position_transfer_detail: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_position_transfer_detail_field : * const TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_periphery_fund_transfer_detail: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_fund_transfer_detail_field : * const TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_bond_conversion_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_conversion_info_field : * const TORASTOCKAPI_CTORATstpBondConversionInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_bond_putback_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_putback_info_field : * const TORASTOCKAPI_CTORATstpBondPutbackInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_cond_order_limit_param: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_cond_order_limit_param_field : * const TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_condition_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const TORASTOCKAPI_CTORATstpConditionOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cond_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_cond_order_action_field : * const TORASTOCKAPI_CTORATstpCondOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_notice: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const TORASTOCKAPI_CTORATstpTradingNoticeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_number_result: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_number_result_field : * const TORASTOCKAPI_CTORATstpIPONumberResultField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_match_number_result: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_match_number_result_field : * const TORASTOCKAPI_CTORATstpIPOMatchNumberResultField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_shareholder_spec_privilege: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_spec_privilege_field : * const TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_market: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_field : * const TORASTOCKAPI_CTORATstpMarketField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_etf_file: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_file_field : * const TORASTOCKAPI_CTORATstpETFFileField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_etf_basket: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_basket_field : * const TORASTOCKAPI_CTORATstpETFBasketField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_position_limit: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_position_limit_field : * const TORASTOCKAPI_CTORATstpInvestorPositionLimitField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_szse_imc_params: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_params_field : * const TORASTOCKAPI_CTORATstpSZSEImcParamsField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_szse_imc_exchange_rate: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_exchange_rate_field : * const TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_szsehk_price_tick_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_szsehk_price_tick_info_field : * const TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lof_fund_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_lof_fund_info_field : * const TORASTOCKAPI_CTORATstpLofFundInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_pledge_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_position_field : * const TORASTOCKAPI_CTORATstpPledgePositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_pledge_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_info_field : * const TORASTOCKAPI_CTORATstpPledgeInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_system_node_info: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_system_node_info_field : * const TORASTOCKAPI_CTORATstpSystemNodeInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_standard_bond_position: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_standard_bond_position_field : * const TORASTOCKAPI_CTORATstpStandardBondPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_prematurity_repo_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_prematurity_repo_order_field : * const TORASTOCKAPI_CTORATstpPrematurityRepoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_nego_order: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const TORASTOCKAPI_CTORATstpNegoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_nego_order_action: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_action_field : * const TORASTOCKAPI_CTORATstpNegoOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_nego_trade: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const TORASTOCKAPI_CTORATstpNegoTradeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_negotiation_param: extern "C" fn(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_negotiation_param_field : * const TORASTOCKAPI_CTORATstpNegotiationParamField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                 } 

        #[derive(Clone, Debug, Decode, Encode)]
        pub enum TORASTOCKAPI_CTORATstpTraderSpiOutput {OnFrontConnected(TORASTOCKAPI_CTORATstpTraderSpiOnFrontConnectedPacket),OnFrontDisconnected(TORASTOCKAPI_CTORATstpTraderSpiOnFrontDisconnectedPacket),OnRspError(TORASTOCKAPI_CTORATstpTraderSpiOnRspErrorPacket),OnRspGetConnectionInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspGetConnectionInfoPacket),OnRspUserLogin(TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLoginPacket),OnRspUserLogout(TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLogoutPacket),OnRspUserPasswordUpdate(TORASTOCKAPI_CTORATstpTraderSpiOnRspUserPasswordUpdatePacket),OnRspInputDeviceSerial(TORASTOCKAPI_CTORATstpTraderSpiOnRspInputDeviceSerialPacket),OnRspOrderInsert(TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderInsertPacket),OnRtnOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRtnOrderPacket),OnErrRtnOrderInsert(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderInsertPacket),OnRtnTrade(TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradePacket),OnRspOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderActionPacket),OnErrRtnOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderActionPacket),OnRspCondOrderInsert(TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderInsertPacket),OnRtnCondOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRtnCondOrderPacket),OnErrRtnCondOrderInsert(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderInsertPacket),OnRspCondOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderActionPacket),OnErrRtnCondOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderActionPacket),OnRspNegoOrderInsert(TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderInsertPacket),OnRtnNegoOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoOrderPacket),OnErrRtnNegoOrderInsert(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderInsertPacket),OnRtnNegoTrade(TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoTradePacket),OnRspNegoOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderActionPacket),OnErrRtnNegoOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderActionPacket),OnRtnMarketStatus(TORASTOCKAPI_CTORATstpTraderSpiOnRtnMarketStatusPacket),OnRspTransferFund(TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferFundPacket),OnErrRtnTransferFund(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferFundPacket),OnRtnTransferFund(TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferFundPacket),OnRspTransferPosition(TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferPositionPacket),OnErrRtnTransferPosition(TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferPositionPacket),OnRtnTransferPosition(TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferPositionPacket),OnRtnPeripheryTransferPosition(TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferPositionPacket),OnRtnPeripheryTransferFund(TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferFundPacket),OnRspInquiryJZFund(TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryJZFundPacket),OnRspInquiryBankAccountFund(TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryBankAccountFundPacket),OnRtnTradingNotice(TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradingNoticePacket),OnRspInquiryMaxOrderVolume(TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryMaxOrderVolumePacket),OnRspInquiryTradeConcentration(TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryTradeConcentrationPacket),OnRspModifyOpenPosCost(TORASTOCKAPI_CTORATstpTraderSpiOnRspModifyOpenPosCostPacket),OnRspInputNodeFundAssignment(TORASTOCKAPI_CTORATstpTraderSpiOnRspInputNodeFundAssignmentPacket),OnRspInquiryNodeFundAssignment(TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryNodeFundAssignmentPacket),OnRspQryExchange(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryExchangePacket),OnRspQrySecurity(TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySecurityPacket),OnRspQryIPOInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOInfoPacket),OnRspQryUser(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryUserPacket),OnRspQryInvestor(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPacket),OnRspQryShareholderAccount(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderAccountPacket),OnRspQryRationalInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryRationalInfoPacket),OnRspQryOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderPacket),OnRspQryOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderActionPacket),OnRspQryTrade(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradePacket),OnRspQryTradingAccount(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingAccountPacket),OnRspQryPosition(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionPacket),OnRspQryTradingFee(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingFeePacket),OnRspQryInvestorTradingFee(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorTradingFeePacket),OnRspQryIPOQuota(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOQuotaPacket),OnRspQryOrderFundDetail(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderFundDetailPacket),OnRspQryFundTransferDetail(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryFundTransferDetailPacket),OnRspQryPositionTransferDetail(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionTransferDetailPacket),OnRspQryPeripheryPositionTransferDetail(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryPositionTransferDetailPacket),OnRspQryPeripheryFundTransferDetail(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryFundTransferDetailPacket),OnRspQryBondConversionInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondConversionInfoPacket),OnRspQryBondPutbackInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondPutbackInfoPacket),OnRspQryInvestorCondOrderLimitParam(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorCondOrderLimitParamPacket),OnRspQryConditionOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryConditionOrderPacket),OnRspQryCondOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryCondOrderActionPacket),OnRspQryTradingNotice(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingNoticePacket),OnRspQryIPONumberResult(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPONumberResultPacket),OnRspQryIPOMatchNumberResult(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOMatchNumberResultPacket),OnRspQryShareholderSpecPrivilege(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderSpecPrivilegePacket),OnRspQryMarket(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryMarketPacket),OnRspQryETFFile(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFFilePacket),OnRspQryETFBasket(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFBasketPacket),OnRspQryInvestorPositionLimit(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPositionLimitPacket),OnRspQrySZSEImcParams(TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcParamsPacket),OnRspQrySZSEImcExchangeRate(TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcExchangeRatePacket),OnRspQrySZSEHKPriceTickInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEHKPriceTickInfoPacket),OnRspQryLofFundInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryLofFundInfoPacket),OnRspQryPledgePosition(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgePositionPacket),OnRspQryPledgeInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgeInfoPacket),OnRspQrySystemNodeInfo(TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySystemNodeInfoPacket),OnRspQryStandardBondPosition(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryStandardBondPositionPacket),OnRspQryPrematurityRepoOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPrematurityRepoOrderPacket),OnRspQryNegoOrder(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderPacket),OnRspQryNegoOrderAction(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderActionPacket),OnRspQryNegoTrade(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoTradePacket),OnRspQryNegotiationParam(TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegotiationParamPacket), } 

            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspErrorPacket {
                pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspGetConnectionInfoPacket {
                pub p_connection_info_field : Option<TORASTOCKAPI_CTORATstpConnectionInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLoginPacket {
                pub p_rsp_user_login_field : Option<TORASTOCKAPI_CTORATstpRspUserLoginField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLogoutPacket {
                pub p_user_logout_field : Option<TORASTOCKAPI_CTORATstpUserLogoutField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspUserPasswordUpdatePacket {
                pub p_user_password_update_field : Option<TORASTOCKAPI_CTORATstpUserPasswordUpdateField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInputDeviceSerialPacket {
                pub p_rsp_input_device_serial_field : Option<TORASTOCKAPI_CTORATstpRspInputDeviceSerialField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderInsertPacket {
                pub p_input_order_field : Option<TORASTOCKAPI_CTORATstpInputOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnOrderPacket {
                pub p_order_field : Option<TORASTOCKAPI_CTORATstpOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderInsertPacket {
                pub p_input_order_field : Option<TORASTOCKAPI_CTORATstpInputOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradePacket {
                pub p_trade_field : Option<TORASTOCKAPI_CTORATstpTradeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderActionPacket {
                pub p_input_order_action_field : Option<TORASTOCKAPI_CTORATstpInputOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderActionPacket {
                pub p_input_order_action_field : Option<TORASTOCKAPI_CTORATstpInputOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderInsertPacket {
                pub p_input_cond_order_field : Option<TORASTOCKAPI_CTORATstpInputCondOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnCondOrderPacket {
                pub p_condition_order_field : Option<TORASTOCKAPI_CTORATstpConditionOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderInsertPacket {
                pub p_input_cond_order_field : Option<TORASTOCKAPI_CTORATstpInputCondOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderActionPacket {
                pub p_input_cond_order_action_field : Option<TORASTOCKAPI_CTORATstpInputCondOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderActionPacket {
                pub p_input_cond_order_action_field : Option<TORASTOCKAPI_CTORATstpInputCondOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderInsertPacket {
                pub p_input_nego_order_field : Option<TORASTOCKAPI_CTORATstpInputNegoOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoOrderPacket {
                pub p_nego_order_field : Option<TORASTOCKAPI_CTORATstpNegoOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderInsertPacket {
                pub p_input_nego_order_field : Option<TORASTOCKAPI_CTORATstpInputNegoOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoTradePacket {
                pub p_nego_trade_field : Option<TORASTOCKAPI_CTORATstpNegoTradeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderActionPacket {
                pub p_input_nego_order_action_field : Option<TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderActionPacket {
                pub p_input_nego_order_action_field : Option<TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnMarketStatusPacket {
                pub p_market_status_field : Option<TORASTOCKAPI_CTORATstpMarketStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferFundPacket {
                pub p_input_transfer_fund_field : Option<TORASTOCKAPI_CTORATstpInputTransferFundField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferFundPacket {
                pub p_input_transfer_fund_field : Option<TORASTOCKAPI_CTORATstpInputTransferFundField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferFundPacket {
                pub p_transfer_fund_field : Option<TORASTOCKAPI_CTORATstpTransferFundField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferPositionPacket {
                pub p_input_transfer_position_field : Option<TORASTOCKAPI_CTORATstpInputTransferPositionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferPositionPacket {
                pub p_input_transfer_position_field : Option<TORASTOCKAPI_CTORATstpInputTransferPositionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferPositionPacket {
                pub p_transfer_position_field : Option<TORASTOCKAPI_CTORATstpTransferPositionField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferPositionPacket {
                pub p_periphery_transfer_position_field : Option<TORASTOCKAPI_CTORATstpPeripheryTransferPositionField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferFundPacket {
                pub p_periphery_transfer_fund_field : Option<TORASTOCKAPI_CTORATstpPeripheryTransferFundField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryJZFundPacket {
                pub p_rsp_inquiry_jz_fund_field : Option<TORASTOCKAPI_CTORATstpRspInquiryJZFundField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryBankAccountFundPacket {
                pub p_rsp_inquiry_bank_account_fund_field : Option<TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradingNoticePacket {
                pub p_trading_notice_field : Option<TORASTOCKAPI_CTORATstpTradingNoticeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryMaxOrderVolumePacket {
                pub p_rsp_inquiry_max_order_volume_field : Option<TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryTradeConcentrationPacket {
                pub p_inquiry_trade_concentration_field : Option<TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspModifyOpenPosCostPacket {
                pub p_req_modify_open_pos_cost_field : Option<TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInputNodeFundAssignmentPacket {
                pub p_input_node_fund_assignment_field : Option<TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryNodeFundAssignmentPacket {
                pub p_rsp_inquiry_node_fund_assignment_field : Option<TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryExchangePacket {
                pub p_exchange_field : Option<TORASTOCKAPI_CTORATstpExchangeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySecurityPacket {
                pub p_security_field : Option<TORASTOCKAPI_CTORATstpSecurityField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOInfoPacket {
                pub p_ipo_info_field : Option<TORASTOCKAPI_CTORATstpIPOInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryUserPacket {
                pub p_user_field : Option<TORASTOCKAPI_CTORATstpUserField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPacket {
                pub p_investor_field : Option<TORASTOCKAPI_CTORATstpInvestorField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderAccountPacket {
                pub p_shareholder_account_field : Option<TORASTOCKAPI_CTORATstpShareholderAccountField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryRationalInfoPacket {
                pub p_rational_info_field : Option<TORASTOCKAPI_CTORATstpRationalInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderPacket {
                pub p_order_field : Option<TORASTOCKAPI_CTORATstpOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderActionPacket {
                pub p_order_action_field : Option<TORASTOCKAPI_CTORATstpOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradePacket {
                pub p_trade_field : Option<TORASTOCKAPI_CTORATstpTradeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingAccountPacket {
                pub p_trading_account_field : Option<TORASTOCKAPI_CTORATstpTradingAccountField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionPacket {
                pub p_position_field : Option<TORASTOCKAPI_CTORATstpPositionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingFeePacket {
                pub p_trading_fee_field : Option<TORASTOCKAPI_CTORATstpTradingFeeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorTradingFeePacket {
                pub p_investor_trading_fee_field : Option<TORASTOCKAPI_CTORATstpInvestorTradingFeeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOQuotaPacket {
                pub p_ipo_quota_field : Option<TORASTOCKAPI_CTORATstpIPOQuotaField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderFundDetailPacket {
                pub p_order_fund_detail_field : Option<TORASTOCKAPI_CTORATstpOrderFundDetailField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryFundTransferDetailPacket {
                pub p_fund_transfer_detail_field : Option<TORASTOCKAPI_CTORATstpFundTransferDetailField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionTransferDetailPacket {
                pub p_position_transfer_detail_field : Option<TORASTOCKAPI_CTORATstpPositionTransferDetailField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryPositionTransferDetailPacket {
                pub p_periphery_position_transfer_detail_field : Option<TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryFundTransferDetailPacket {
                pub p_periphery_fund_transfer_detail_field : Option<TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondConversionInfoPacket {
                pub p_bond_conversion_info_field : Option<TORASTOCKAPI_CTORATstpBondConversionInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondPutbackInfoPacket {
                pub p_bond_putback_info_field : Option<TORASTOCKAPI_CTORATstpBondPutbackInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorCondOrderLimitParamPacket {
                pub p_investor_cond_order_limit_param_field : Option<TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryConditionOrderPacket {
                pub p_condition_order_field : Option<TORASTOCKAPI_CTORATstpConditionOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryCondOrderActionPacket {
                pub p_cond_order_action_field : Option<TORASTOCKAPI_CTORATstpCondOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingNoticePacket {
                pub p_trading_notice_field : Option<TORASTOCKAPI_CTORATstpTradingNoticeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPONumberResultPacket {
                pub p_ipo_number_result_field : Option<TORASTOCKAPI_CTORATstpIPONumberResultField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOMatchNumberResultPacket {
                pub p_ipo_match_number_result_field : Option<TORASTOCKAPI_CTORATstpIPOMatchNumberResultField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderSpecPrivilegePacket {
                pub p_shareholder_spec_privilege_field : Option<TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryMarketPacket {
                pub p_market_field : Option<TORASTOCKAPI_CTORATstpMarketField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFFilePacket {
                pub p_etf_file_field : Option<TORASTOCKAPI_CTORATstpETFFileField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFBasketPacket {
                pub p_etf_basket_field : Option<TORASTOCKAPI_CTORATstpETFBasketField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPositionLimitPacket {
                pub p_investor_position_limit_field : Option<TORASTOCKAPI_CTORATstpInvestorPositionLimitField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcParamsPacket {
                pub p_szse_imc_params_field : Option<TORASTOCKAPI_CTORATstpSZSEImcParamsField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcExchangeRatePacket {
                pub p_szse_imc_exchange_rate_field : Option<TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEHKPriceTickInfoPacket {
                pub p_szsehk_price_tick_info_field : Option<TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryLofFundInfoPacket {
                pub p_lof_fund_info_field : Option<TORASTOCKAPI_CTORATstpLofFundInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgePositionPacket {
                pub p_pledge_position_field : Option<TORASTOCKAPI_CTORATstpPledgePositionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgeInfoPacket {
                pub p_pledge_info_field : Option<TORASTOCKAPI_CTORATstpPledgeInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySystemNodeInfoPacket {
                pub p_system_node_info_field : Option<TORASTOCKAPI_CTORATstpSystemNodeInfoField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryStandardBondPositionPacket {
                pub p_standard_bond_position_field : Option<TORASTOCKAPI_CTORATstpStandardBondPositionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPrematurityRepoOrderPacket {
                pub p_prematurity_repo_order_field : Option<TORASTOCKAPI_CTORATstpPrematurityRepoOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderPacket {
                pub p_nego_order_field : Option<TORASTOCKAPI_CTORATstpNegoOrderField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderActionPacket {
                pub p_nego_order_action_field : Option<TORASTOCKAPI_CTORATstpNegoOrderActionField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoTradePacket {
                pub p_nego_trade_field : Option<TORASTOCKAPI_CTORATstpNegoTradeField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegotiationParamPacket {
                pub p_negotiation_param_field : Option<TORASTOCKAPI_CTORATstpNegotiationParamField>,pub p_rsp_info_field : Option<TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }  
static TORASTOCKAPI_CTORA_TSTP_TRADER_SPI_VTABLE: TORASTOCKAPI_CTORATstpTraderSpiVTable = TORASTOCKAPI_CTORATstpTraderSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_rsp_error: spi_on_rsp_error,
            on_rsp_get_connection_info: spi_on_rsp_get_connection_info,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_user_password_update: spi_on_rsp_user_password_update,
            on_rsp_input_device_serial: spi_on_rsp_input_device_serial,
            on_rsp_order_insert: spi_on_rsp_order_insert,
            on_rtn_order: spi_on_rtn_order,
            on_err_rtn_order_insert: spi_on_err_rtn_order_insert,
            on_rtn_trade: spi_on_rtn_trade,
            on_rsp_order_action: spi_on_rsp_order_action,
            on_err_rtn_order_action: spi_on_err_rtn_order_action,
            on_rsp_cond_order_insert: spi_on_rsp_cond_order_insert,
            on_rtn_cond_order: spi_on_rtn_cond_order,
            on_err_rtn_cond_order_insert: spi_on_err_rtn_cond_order_insert,
            on_rsp_cond_order_action: spi_on_rsp_cond_order_action,
            on_err_rtn_cond_order_action: spi_on_err_rtn_cond_order_action,
            on_rsp_nego_order_insert: spi_on_rsp_nego_order_insert,
            on_rtn_nego_order: spi_on_rtn_nego_order,
            on_err_rtn_nego_order_insert: spi_on_err_rtn_nego_order_insert,
            on_rtn_nego_trade: spi_on_rtn_nego_trade,
            on_rsp_nego_order_action: spi_on_rsp_nego_order_action,
            on_err_rtn_nego_order_action: spi_on_err_rtn_nego_order_action,
            on_rtn_market_status: spi_on_rtn_market_status,
            on_rsp_transfer_fund: spi_on_rsp_transfer_fund,
            on_err_rtn_transfer_fund: spi_on_err_rtn_transfer_fund,
            on_rtn_transfer_fund: spi_on_rtn_transfer_fund,
            on_rsp_transfer_position: spi_on_rsp_transfer_position,
            on_err_rtn_transfer_position: spi_on_err_rtn_transfer_position,
            on_rtn_transfer_position: spi_on_rtn_transfer_position,
            on_rtn_periphery_transfer_position: spi_on_rtn_periphery_transfer_position,
            on_rtn_periphery_transfer_fund: spi_on_rtn_periphery_transfer_fund,
            on_rsp_inquiry_jz_fund: spi_on_rsp_inquiry_jz_fund,
            on_rsp_inquiry_bank_account_fund: spi_on_rsp_inquiry_bank_account_fund,
            on_rtn_trading_notice: spi_on_rtn_trading_notice,
            on_rsp_inquiry_max_order_volume: spi_on_rsp_inquiry_max_order_volume,
            on_rsp_inquiry_trade_concentration: spi_on_rsp_inquiry_trade_concentration,
            on_rsp_modify_open_pos_cost: spi_on_rsp_modify_open_pos_cost,
            on_rsp_input_node_fund_assignment: spi_on_rsp_input_node_fund_assignment,
            on_rsp_inquiry_node_fund_assignment: spi_on_rsp_inquiry_node_fund_assignment,
            on_rsp_qry_exchange: spi_on_rsp_qry_exchange,
            on_rsp_qry_security: spi_on_rsp_qry_security,
            on_rsp_qry_ipo_info: spi_on_rsp_qry_ipo_info,
            on_rsp_qry_user: spi_on_rsp_qry_user,
            on_rsp_qry_investor: spi_on_rsp_qry_investor,
            on_rsp_qry_shareholder_account: spi_on_rsp_qry_shareholder_account,
            on_rsp_qry_rational_info: spi_on_rsp_qry_rational_info,
            on_rsp_qry_order: spi_on_rsp_qry_order,
            on_rsp_qry_order_action: spi_on_rsp_qry_order_action,
            on_rsp_qry_trade: spi_on_rsp_qry_trade,
            on_rsp_qry_trading_account: spi_on_rsp_qry_trading_account,
            on_rsp_qry_position: spi_on_rsp_qry_position,
            on_rsp_qry_trading_fee: spi_on_rsp_qry_trading_fee,
            on_rsp_qry_investor_trading_fee: spi_on_rsp_qry_investor_trading_fee,
            on_rsp_qry_ipo_quota: spi_on_rsp_qry_ipo_quota,
            on_rsp_qry_order_fund_detail: spi_on_rsp_qry_order_fund_detail,
            on_rsp_qry_fund_transfer_detail: spi_on_rsp_qry_fund_transfer_detail,
            on_rsp_qry_position_transfer_detail: spi_on_rsp_qry_position_transfer_detail,
            on_rsp_qry_periphery_position_transfer_detail: spi_on_rsp_qry_periphery_position_transfer_detail,
            on_rsp_qry_periphery_fund_transfer_detail: spi_on_rsp_qry_periphery_fund_transfer_detail,
            on_rsp_qry_bond_conversion_info: spi_on_rsp_qry_bond_conversion_info,
            on_rsp_qry_bond_putback_info: spi_on_rsp_qry_bond_putback_info,
            on_rsp_qry_investor_cond_order_limit_param: spi_on_rsp_qry_investor_cond_order_limit_param,
            on_rsp_qry_condition_order: spi_on_rsp_qry_condition_order,
            on_rsp_qry_cond_order_action: spi_on_rsp_qry_cond_order_action,
            on_rsp_qry_trading_notice: spi_on_rsp_qry_trading_notice,
            on_rsp_qry_ipo_number_result: spi_on_rsp_qry_ipo_number_result,
            on_rsp_qry_ipo_match_number_result: spi_on_rsp_qry_ipo_match_number_result,
            on_rsp_qry_shareholder_spec_privilege: spi_on_rsp_qry_shareholder_spec_privilege,
            on_rsp_qry_market: spi_on_rsp_qry_market,
            on_rsp_qry_etf_file: spi_on_rsp_qry_etf_file,
            on_rsp_qry_etf_basket: spi_on_rsp_qry_etf_basket,
            on_rsp_qry_investor_position_limit: spi_on_rsp_qry_investor_position_limit,
            on_rsp_qry_szse_imc_params: spi_on_rsp_qry_szse_imc_params,
            on_rsp_qry_szse_imc_exchange_rate: spi_on_rsp_qry_szse_imc_exchange_rate,
            on_rsp_qry_szsehk_price_tick_info: spi_on_rsp_qry_szsehk_price_tick_info,
            on_rsp_qry_lof_fund_info: spi_on_rsp_qry_lof_fund_info,
            on_rsp_qry_pledge_position: spi_on_rsp_qry_pledge_position,
            on_rsp_qry_pledge_info: spi_on_rsp_qry_pledge_info,
            on_rsp_qry_system_node_info: spi_on_rsp_qry_system_node_info,
            on_rsp_qry_standard_bond_position: spi_on_rsp_qry_standard_bond_position,
            on_rsp_qry_prematurity_repo_order: spi_on_rsp_qry_prematurity_repo_order,
            on_rsp_qry_nego_order: spi_on_rsp_qry_nego_order,
            on_rsp_qry_nego_order_action: spi_on_rsp_qry_nego_order_action,
            on_rsp_qry_nego_trade: spi_on_rsp_qry_nego_trade,
            on_rsp_qry_negotiation_param: spi_on_rsp_qry_negotiation_param,
             };
extern "C" fn spi_on_front_connected(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_rsp_error(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_error(p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_get_connection_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_connection_info_field : * const TORASTOCKAPI_CTORATstpConnectionInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_get_connection_info(p_connection_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_user_login_field : * const TORASTOCKAPI_CTORATstpRspUserLoginField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_logout_field : * const TORASTOCKAPI_CTORATstpUserLogoutField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_password_update(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_password_update_field : * const TORASTOCKAPI_CTORATstpUserPasswordUpdateField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_password_update(p_user_password_update_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_input_device_serial(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_input_device_serial_field : * const TORASTOCKAPI_CTORATstpRspInputDeviceSerialField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_input_device_serial(p_rsp_input_device_serial_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_order_insert(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_insert(p_input_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const TORASTOCKAPI_CTORATstpOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_order(p_order_field.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_order_insert(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_insert(p_input_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_trade(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const TORASTOCKAPI_CTORATstpTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trade(p_trade_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_action(p_input_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_action(p_input_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_insert(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_insert(p_input_cond_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_cond_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const TORASTOCKAPI_CTORATstpConditionOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_cond_order(p_condition_order_field.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_insert(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_insert(p_input_cond_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_action(p_input_cond_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_action(p_input_cond_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_nego_order_insert(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_nego_order_insert(p_input_nego_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_nego_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const TORASTOCKAPI_CTORATstpNegoOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_nego_order(p_nego_order_field.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_nego_order_insert(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_nego_order_insert(p_input_nego_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_nego_trade(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const TORASTOCKAPI_CTORATstpNegoTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_nego_trade(p_nego_trade_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_nego_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_nego_order_action(p_input_nego_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_nego_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_nego_order_action(p_input_nego_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_market_status(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_status_field : * const TORASTOCKAPI_CTORATstpMarketStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_market_status(p_market_status_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_transfer_fund(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_fund(p_input_transfer_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_transfer_fund(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_fund(p_input_transfer_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_fund(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_fund_field : * const TORASTOCKAPI_CTORATstpTransferFundField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_fund(p_transfer_fund_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_transfer_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_position(p_input_transfer_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_transfer_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_position(p_input_transfer_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_position_field : * const TORASTOCKAPI_CTORATstpTransferPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_position(p_transfer_position_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_periphery_transfer_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_position_field : * const TORASTOCKAPI_CTORATstpPeripheryTransferPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_periphery_transfer_position(p_periphery_transfer_position_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_periphery_transfer_fund(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_fund_field : * const TORASTOCKAPI_CTORATstpPeripheryTransferFundField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_periphery_transfer_fund(p_periphery_transfer_fund_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_inquiry_jz_fund(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const TORASTOCKAPI_CTORATstpRspInquiryJZFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_jz_fund(p_rsp_inquiry_jz_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_bank_account_fund(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_bank_account_fund(p_rsp_inquiry_bank_account_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_trading_notice(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const TORASTOCKAPI_CTORATstpTradingNoticeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trading_notice(p_trading_notice_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_inquiry_max_order_volume(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_max_order_volume_field : * const TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_max_order_volume(p_rsp_inquiry_max_order_volume_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_trade_concentration(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_inquiry_trade_concentration_field : * const TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_trade_concentration(p_inquiry_trade_concentration_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_modify_open_pos_cost(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_req_modify_open_pos_cost_field : * const TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_modify_open_pos_cost(p_req_modify_open_pos_cost_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_input_node_fund_assignment(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_node_fund_assignment_field : * const TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_input_node_fund_assignment(p_input_node_fund_assignment_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_node_fund_assignment(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_node_fund_assignment_field : * const TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_node_fund_assignment(p_rsp_inquiry_node_fund_assignment_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_qry_exchange(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_exchange_field : * const TORASTOCKAPI_CTORATstpExchangeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange(p_exchange_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_security(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_security_field : * const TORASTOCKAPI_CTORATstpSecurityField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_security(p_security_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_info_field : * const TORASTOCKAPI_CTORATstpIPOInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_info(p_ipo_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_user(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_field : * const TORASTOCKAPI_CTORATstpUserField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_user(p_user_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_field : * const TORASTOCKAPI_CTORATstpInvestorField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor(p_investor_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_shareholder_account(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_account_field : * const TORASTOCKAPI_CTORATstpShareholderAccountField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_shareholder_account(p_shareholder_account_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rational_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_rational_info_field : * const TORASTOCKAPI_CTORATstpRationalInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rational_info(p_rational_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const TORASTOCKAPI_CTORATstpOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order(p_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_action_field : * const TORASTOCKAPI_CTORATstpOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_action(p_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trade(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const TORASTOCKAPI_CTORATstpTradeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trade(p_trade_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_account_field : * const TORASTOCKAPI_CTORATstpTradingAccountField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_account(p_trading_account_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_field : * const TORASTOCKAPI_CTORATstpPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position(p_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_fee(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_fee_field : * const TORASTOCKAPI_CTORATstpTradingFeeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_fee(p_trading_fee_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_trading_fee(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_trading_fee_field : * const TORASTOCKAPI_CTORATstpInvestorTradingFeeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_trading_fee(p_investor_trading_fee_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_quota(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_quota_field : * const TORASTOCKAPI_CTORATstpIPOQuotaField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_quota(p_ipo_quota_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_fund_detail(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_fund_detail_field : * const TORASTOCKAPI_CTORATstpOrderFundDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_fund_detail(p_order_fund_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_fund_transfer_detail(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_fund_transfer_detail_field : * const TORASTOCKAPI_CTORATstpFundTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_fund_transfer_detail(p_fund_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position_transfer_detail(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_transfer_detail_field : * const TORASTOCKAPI_CTORATstpPositionTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position_transfer_detail(p_position_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_periphery_position_transfer_detail(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_position_transfer_detail_field : * const TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_periphery_position_transfer_detail(p_periphery_position_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_periphery_fund_transfer_detail(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_fund_transfer_detail_field : * const TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_periphery_fund_transfer_detail(p_periphery_fund_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_bond_conversion_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_conversion_info_field : * const TORASTOCKAPI_CTORATstpBondConversionInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_bond_conversion_info(p_bond_conversion_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_bond_putback_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_putback_info_field : * const TORASTOCKAPI_CTORATstpBondPutbackInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_bond_putback_info(p_bond_putback_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_cond_order_limit_param(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_cond_order_limit_param_field : * const TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_cond_order_limit_param(p_investor_cond_order_limit_param_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_condition_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const TORASTOCKAPI_CTORATstpConditionOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_condition_order(p_condition_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cond_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_cond_order_action_field : * const TORASTOCKAPI_CTORATstpCondOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cond_order_action(p_cond_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const TORASTOCKAPI_CTORATstpTradingNoticeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_notice(p_trading_notice_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_number_result(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_number_result_field : * const TORASTOCKAPI_CTORATstpIPONumberResultField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_number_result(p_ipo_number_result_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_match_number_result(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_match_number_result_field : * const TORASTOCKAPI_CTORATstpIPOMatchNumberResultField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_match_number_result(p_ipo_match_number_result_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_shareholder_spec_privilege(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_spec_privilege_field : * const TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_shareholder_spec_privilege(p_shareholder_spec_privilege_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_market(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_field : * const TORASTOCKAPI_CTORATstpMarketField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_market(p_market_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_etf_file(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_file_field : * const TORASTOCKAPI_CTORATstpETFFileField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_etf_file(p_etf_file_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_etf_basket(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_basket_field : * const TORASTOCKAPI_CTORATstpETFBasketField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_etf_basket(p_etf_basket_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_position_limit(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_position_limit_field : * const TORASTOCKAPI_CTORATstpInvestorPositionLimitField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_position_limit(p_investor_position_limit_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_szse_imc_params(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_params_field : * const TORASTOCKAPI_CTORATstpSZSEImcParamsField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_szse_imc_params(p_szse_imc_params_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_szse_imc_exchange_rate(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_exchange_rate_field : * const TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_szse_imc_exchange_rate(p_szse_imc_exchange_rate_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_szsehk_price_tick_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_szsehk_price_tick_info_field : * const TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_szsehk_price_tick_info(p_szsehk_price_tick_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lof_fund_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_lof_fund_info_field : * const TORASTOCKAPI_CTORATstpLofFundInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lof_fund_info(p_lof_fund_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_pledge_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_position_field : * const TORASTOCKAPI_CTORATstpPledgePositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_pledge_position(p_pledge_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_pledge_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_info_field : * const TORASTOCKAPI_CTORATstpPledgeInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_pledge_info(p_pledge_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_system_node_info(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_system_node_info_field : * const TORASTOCKAPI_CTORATstpSystemNodeInfoField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_system_node_info(p_system_node_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_standard_bond_position(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_standard_bond_position_field : * const TORASTOCKAPI_CTORATstpStandardBondPositionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_standard_bond_position(p_standard_bond_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_prematurity_repo_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_prematurity_repo_order_field : * const TORASTOCKAPI_CTORATstpPrematurityRepoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_prematurity_repo_order(p_prematurity_repo_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_nego_order(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const TORASTOCKAPI_CTORATstpNegoOrderField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_nego_order(p_nego_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_nego_order_action(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_action_field : * const TORASTOCKAPI_CTORATstpNegoOrderActionField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_nego_order_action(p_nego_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_nego_trade(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const TORASTOCKAPI_CTORATstpNegoTradeField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_nego_trade(p_nego_trade_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_negotiation_param(spi: *mut TORASTOCKAPI_CTORATstpTraderSpiFat, p_negotiation_param_field : * const TORASTOCKAPI_CTORATstpNegotiationParamField,p_rsp_info_field : * const TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_negotiation_param(p_negotiation_param_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }

        #[repr(C)]
        pub struct TORASTOCKAPI_CTORATstpTraderSpiFat {
            vtable: *const TORASTOCKAPI_CTORATstpTraderSpiVTable,
            pub md_spi_ptr: *mut dyn TORASTOCKAPI_CTORATstpTraderSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct TORASTOCKAPI_CTORATstpTraderSpiInner {
            buf: std::collections::VecDeque<TORASTOCKAPI_CTORATstpTraderSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl TORASTOCKAPI_CTORATstpTraderSpiInner {
            fn push(&mut self, msg: TORASTOCKAPI_CTORATstpTraderSpiOutput) {
                self.buf.push_back(msg);
                if let Some(ref waker) = &self.waker {
                    waker.clone().wake()
                }
            }
        }
        
        pub struct TORASTOCKAPI_CTORATstpTraderSpiStream {
            inner: Arc<Mutex<TORASTOCKAPI_CTORATstpTraderSpiInner>>,
        }
        
        impl Stream for TORASTOCKAPI_CTORATstpTraderSpiStream {
            type Item = TORASTOCKAPI_CTORATstpTraderSpiOutput;
        
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
        
        pub fn create_spi() -> (Box<TORASTOCKAPI_CTORATstpTraderSpiStream>, *mut TORASTOCKAPI_CTORATstpTraderSpiStream) {
            let i = TORASTOCKAPI_CTORATstpTraderSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = TORASTOCKAPI_CTORATstpTraderSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl TORASTOCKAPI_CTORATstpTraderSpi_trait for TORASTOCKAPI_CTORATstpTraderSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnFrontConnected( TORASTOCKAPI_CTORATstpTraderSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnFrontDisconnected( TORASTOCKAPI_CTORATstpTraderSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_rsp_error(&mut self, p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspError( TORASTOCKAPI_CTORATstpTraderSpiOnRspErrorPacket { p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_get_connection_info(&mut self, p_connection_info_field : Option<&TORASTOCKAPI_CTORATstpConnectionInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspGetConnectionInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspGetConnectionInfoPacket { p_connection_info_field:p_connection_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&TORASTOCKAPI_CTORATstpRspUserLoginField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspUserLogin( TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLoginPacket { p_rsp_user_login_field:p_rsp_user_login_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&TORASTOCKAPI_CTORATstpUserLogoutField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspUserLogout( TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLogoutPacket { p_user_logout_field:p_user_logout_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&TORASTOCKAPI_CTORATstpUserPasswordUpdateField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspUserPasswordUpdate( TORASTOCKAPI_CTORATstpTraderSpiOnRspUserPasswordUpdatePacket { p_user_password_update_field:p_user_password_update_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&TORASTOCKAPI_CTORATstpRspInputDeviceSerialField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInputDeviceSerial( TORASTOCKAPI_CTORATstpTraderSpiOnRspInputDeviceSerialPacket { p_rsp_input_device_serial_field:p_rsp_input_device_serial_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspOrderInsert( TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderInsertPacket { p_input_order_field:p_input_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_order(&mut self, p_order_field : Option<&TORASTOCKAPI_CTORATstpOrderField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRtnOrderPacket { p_order_field:p_order_field.cloned() } ))
                }
            fn on_err_rtn_order_insert(&mut self, p_input_order_field : Option<&TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnOrderInsert( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderInsertPacket { p_input_order_field:p_input_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_trade(&mut self, p_trade_field : Option<&TORASTOCKAPI_CTORATstpTradeField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTrade( TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradePacket { p_trade_field:p_trade_field.cloned() } ))
                }
            fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderActionPacket { p_input_order_action_field:p_input_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_order_action(&mut self, p_input_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderActionPacket { p_input_order_action_field:p_input_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspCondOrderInsert( TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderInsertPacket { p_input_cond_order_field:p_input_cond_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_cond_order(&mut self, p_condition_order_field : Option<&TORASTOCKAPI_CTORATstpConditionOrderField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnCondOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRtnCondOrderPacket { p_condition_order_field:p_condition_order_field.cloned() } ))
                }
            fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnCondOrderInsert( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderInsertPacket { p_input_cond_order_field:p_input_cond_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspCondOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderActionPacket { p_input_cond_order_action_field:p_input_cond_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnCondOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderActionPacket { p_input_cond_order_action_field:p_input_cond_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_nego_order_insert(&mut self, p_input_nego_order_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspNegoOrderInsert( TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderInsertPacket { p_input_nego_order_field:p_input_nego_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_nego_order(&mut self, p_nego_order_field : Option<&TORASTOCKAPI_CTORATstpNegoOrderField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnNegoOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoOrderPacket { p_nego_order_field:p_nego_order_field.cloned() } ))
                }
            fn on_err_rtn_nego_order_insert(&mut self, p_input_nego_order_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnNegoOrderInsert( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderInsertPacket { p_input_nego_order_field:p_input_nego_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_nego_trade(&mut self, p_nego_trade_field : Option<&TORASTOCKAPI_CTORATstpNegoTradeField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnNegoTrade( TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoTradePacket { p_nego_trade_field:p_nego_trade_field.cloned() } ))
                }
            fn on_rsp_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspNegoOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderActionPacket { p_input_nego_order_action_field:p_input_nego_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnNegoOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderActionPacket { p_input_nego_order_action_field:p_input_nego_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_market_status(&mut self, p_market_status_field : Option<&TORASTOCKAPI_CTORATstpMarketStatusField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnMarketStatus( TORASTOCKAPI_CTORATstpTraderSpiOnRtnMarketStatusPacket { p_market_status_field:p_market_status_field.cloned() } ))
                }
            fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspTransferFund( TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferFundPacket { p_input_transfer_fund_field:p_input_transfer_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnTransferFund( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferFundPacket { p_input_transfer_fund_field:p_input_transfer_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_fund(&mut self, p_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpTransferFundField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTransferFund( TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferFundPacket { p_transfer_fund_field:p_transfer_fund_field.cloned() } ))
                }
            fn on_rsp_transfer_position(&mut self, p_input_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspTransferPosition( TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferPositionPacket { p_input_transfer_position_field:p_input_transfer_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnTransferPosition( TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferPositionPacket { p_input_transfer_position_field:p_input_transfer_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_position(&mut self, p_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpTransferPositionField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTransferPosition( TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferPositionPacket { p_transfer_position_field:p_transfer_position_field.cloned() } ))
                }
            fn on_rtn_periphery_transfer_position(&mut self, p_periphery_transfer_position_field : Option<&TORASTOCKAPI_CTORATstpPeripheryTransferPositionField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnPeripheryTransferPosition( TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferPositionPacket { p_periphery_transfer_position_field:p_periphery_transfer_position_field.cloned() } ))
                }
            fn on_rtn_periphery_transfer_fund(&mut self, p_periphery_transfer_fund_field : Option<&TORASTOCKAPI_CTORATstpPeripheryTransferFundField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnPeripheryTransferFund( TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferFundPacket { p_periphery_transfer_fund_field:p_periphery_transfer_fund_field.cloned() } ))
                }
            fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryJZFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryJZFund( TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryJZFundPacket { p_rsp_inquiry_jz_fund_field:p_rsp_inquiry_jz_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryBankAccountFund( TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryBankAccountFundPacket { p_rsp_inquiry_bank_account_fund_field:p_rsp_inquiry_bank_account_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_trading_notice(&mut self, p_trading_notice_field : Option<&TORASTOCKAPI_CTORATstpTradingNoticeField>) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTradingNotice( TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradingNoticePacket { p_trading_notice_field:p_trading_notice_field.cloned() } ))
                }
            fn on_rsp_inquiry_max_order_volume(&mut self, p_rsp_inquiry_max_order_volume_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryMaxOrderVolume( TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryMaxOrderVolumePacket { p_rsp_inquiry_max_order_volume_field:p_rsp_inquiry_max_order_volume_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_trade_concentration(&mut self, p_inquiry_trade_concentration_field : Option<&TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryTradeConcentration( TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryTradeConcentrationPacket { p_inquiry_trade_concentration_field:p_inquiry_trade_concentration_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_modify_open_pos_cost(&mut self, p_req_modify_open_pos_cost_field : Option<&TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspModifyOpenPosCost( TORASTOCKAPI_CTORATstpTraderSpiOnRspModifyOpenPosCostPacket { p_req_modify_open_pos_cost_field:p_req_modify_open_pos_cost_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_input_node_fund_assignment(&mut self, p_input_node_fund_assignment_field : Option<&TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInputNodeFundAssignment( TORASTOCKAPI_CTORATstpTraderSpiOnRspInputNodeFundAssignmentPacket { p_input_node_fund_assignment_field:p_input_node_fund_assignment_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_node_fund_assignment(&mut self, p_rsp_inquiry_node_fund_assignment_field : Option<&TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryNodeFundAssignment( TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryNodeFundAssignmentPacket { p_rsp_inquiry_node_fund_assignment_field:p_rsp_inquiry_node_fund_assignment_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_qry_exchange(&mut self, p_exchange_field : Option<&TORASTOCKAPI_CTORATstpExchangeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryExchange( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryExchangePacket { p_exchange_field:p_exchange_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_security(&mut self, p_security_field : Option<&TORASTOCKAPI_CTORATstpSecurityField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySecurity( TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySecurityPacket { p_security_field:p_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_info(&mut self, p_ipo_info_field : Option<&TORASTOCKAPI_CTORATstpIPOInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPOInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOInfoPacket { p_ipo_info_field:p_ipo_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_user(&mut self, p_user_field : Option<&TORASTOCKAPI_CTORATstpUserField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryUser( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryUserPacket { p_user_field:p_user_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor(&mut self, p_investor_field : Option<&TORASTOCKAPI_CTORATstpInvestorField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestor( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPacket { p_investor_field:p_investor_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account_field : Option<&TORASTOCKAPI_CTORATstpShareholderAccountField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryShareholderAccount( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderAccountPacket { p_shareholder_account_field:p_shareholder_account_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rational_info(&mut self, p_rational_info_field : Option<&TORASTOCKAPI_CTORATstpRationalInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryRationalInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryRationalInfoPacket { p_rational_info_field:p_rational_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order(&mut self, p_order_field : Option<&TORASTOCKAPI_CTORATstpOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderPacket { p_order_field:p_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_action(&mut self, p_order_action_field : Option<&TORASTOCKAPI_CTORATstpOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderActionPacket { p_order_action_field:p_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trade(&mut self, p_trade_field : Option<&TORASTOCKAPI_CTORATstpTradeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTrade( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradePacket { p_trade_field:p_trade_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_account(&mut self, p_trading_account_field : Option<&TORASTOCKAPI_CTORATstpTradingAccountField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTradingAccount( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingAccountPacket { p_trading_account_field:p_trading_account_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position(&mut self, p_position_field : Option<&TORASTOCKAPI_CTORATstpPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPosition( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionPacket { p_position_field:p_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_fee(&mut self, p_trading_fee_field : Option<&TORASTOCKAPI_CTORATstpTradingFeeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTradingFee( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingFeePacket { p_trading_fee_field:p_trading_fee_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee_field : Option<&TORASTOCKAPI_CTORATstpInvestorTradingFeeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestorTradingFee( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorTradingFeePacket { p_investor_trading_fee_field:p_investor_trading_fee_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_quota(&mut self, p_ipo_quota_field : Option<&TORASTOCKAPI_CTORATstpIPOQuotaField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPOQuota( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOQuotaPacket { p_ipo_quota_field:p_ipo_quota_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail_field : Option<&TORASTOCKAPI_CTORATstpOrderFundDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryOrderFundDetail( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderFundDetailPacket { p_order_fund_detail_field:p_order_fund_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpFundTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryFundTransferDetail( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryFundTransferDetailPacket { p_fund_transfer_detail_field:p_fund_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpPositionTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPositionTransferDetail( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionTransferDetailPacket { p_position_transfer_detail_field:p_position_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_periphery_position_transfer_detail(&mut self, p_periphery_position_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPeripheryPositionTransferDetail( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryPositionTransferDetailPacket { p_periphery_position_transfer_detail_field:p_periphery_position_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_periphery_fund_transfer_detail(&mut self, p_periphery_fund_transfer_detail_field : Option<&TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPeripheryFundTransferDetail( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryFundTransferDetailPacket { p_periphery_fund_transfer_detail_field:p_periphery_fund_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_bond_conversion_info(&mut self, p_bond_conversion_info_field : Option<&TORASTOCKAPI_CTORATstpBondConversionInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryBondConversionInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondConversionInfoPacket { p_bond_conversion_info_field:p_bond_conversion_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_bond_putback_info(&mut self, p_bond_putback_info_field : Option<&TORASTOCKAPI_CTORATstpBondPutbackInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryBondPutbackInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondPutbackInfoPacket { p_bond_putback_info_field:p_bond_putback_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_cond_order_limit_param(&mut self, p_investor_cond_order_limit_param_field : Option<&TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestorCondOrderLimitParam( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorCondOrderLimitParamPacket { p_investor_cond_order_limit_param_field:p_investor_cond_order_limit_param_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_condition_order(&mut self, p_condition_order_field : Option<&TORASTOCKAPI_CTORATstpConditionOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryConditionOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryConditionOrderPacket { p_condition_order_field:p_condition_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action_field : Option<&TORASTOCKAPI_CTORATstpCondOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryCondOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryCondOrderActionPacket { p_cond_order_action_field:p_cond_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_notice(&mut self, p_trading_notice_field : Option<&TORASTOCKAPI_CTORATstpTradingNoticeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTradingNotice( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingNoticePacket { p_trading_notice_field:p_trading_notice_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_number_result(&mut self, p_ipo_number_result_field : Option<&TORASTOCKAPI_CTORATstpIPONumberResultField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPONumberResult( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPONumberResultPacket { p_ipo_number_result_field:p_ipo_number_result_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_match_number_result(&mut self, p_ipo_match_number_result_field : Option<&TORASTOCKAPI_CTORATstpIPOMatchNumberResultField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPOMatchNumberResult( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOMatchNumberResultPacket { p_ipo_match_number_result_field:p_ipo_match_number_result_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_shareholder_spec_privilege(&mut self, p_shareholder_spec_privilege_field : Option<&TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryShareholderSpecPrivilege( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderSpecPrivilegePacket { p_shareholder_spec_privilege_field:p_shareholder_spec_privilege_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_market(&mut self, p_market_field : Option<&TORASTOCKAPI_CTORATstpMarketField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryMarket( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryMarketPacket { p_market_field:p_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_etf_file(&mut self, p_etf_file_field : Option<&TORASTOCKAPI_CTORATstpETFFileField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryETFFile( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFFilePacket { p_etf_file_field:p_etf_file_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_etf_basket(&mut self, p_etf_basket_field : Option<&TORASTOCKAPI_CTORATstpETFBasketField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryETFBasket( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFBasketPacket { p_etf_basket_field:p_etf_basket_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_position_limit(&mut self, p_investor_position_limit_field : Option<&TORASTOCKAPI_CTORATstpInvestorPositionLimitField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestorPositionLimit( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPositionLimitPacket { p_investor_position_limit_field:p_investor_position_limit_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_szse_imc_params(&mut self, p_szse_imc_params_field : Option<&TORASTOCKAPI_CTORATstpSZSEImcParamsField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySZSEImcParams( TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcParamsPacket { p_szse_imc_params_field:p_szse_imc_params_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_szse_imc_exchange_rate(&mut self, p_szse_imc_exchange_rate_field : Option<&TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySZSEImcExchangeRate( TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcExchangeRatePacket { p_szse_imc_exchange_rate_field:p_szse_imc_exchange_rate_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_szsehk_price_tick_info(&mut self, p_szsehk_price_tick_info_field : Option<&TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySZSEHKPriceTickInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEHKPriceTickInfoPacket { p_szsehk_price_tick_info_field:p_szsehk_price_tick_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lof_fund_info(&mut self, p_lof_fund_info_field : Option<&TORASTOCKAPI_CTORATstpLofFundInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryLofFundInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryLofFundInfoPacket { p_lof_fund_info_field:p_lof_fund_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_pledge_position(&mut self, p_pledge_position_field : Option<&TORASTOCKAPI_CTORATstpPledgePositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPledgePosition( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgePositionPacket { p_pledge_position_field:p_pledge_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_pledge_info(&mut self, p_pledge_info_field : Option<&TORASTOCKAPI_CTORATstpPledgeInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPledgeInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgeInfoPacket { p_pledge_info_field:p_pledge_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_system_node_info(&mut self, p_system_node_info_field : Option<&TORASTOCKAPI_CTORATstpSystemNodeInfoField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySystemNodeInfo( TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySystemNodeInfoPacket { p_system_node_info_field:p_system_node_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_standard_bond_position(&mut self, p_standard_bond_position_field : Option<&TORASTOCKAPI_CTORATstpStandardBondPositionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryStandardBondPosition( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryStandardBondPositionPacket { p_standard_bond_position_field:p_standard_bond_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_prematurity_repo_order(&mut self, p_prematurity_repo_order_field : Option<&TORASTOCKAPI_CTORATstpPrematurityRepoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPrematurityRepoOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPrematurityRepoOrderPacket { p_prematurity_repo_order_field:p_prematurity_repo_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_nego_order(&mut self, p_nego_order_field : Option<&TORASTOCKAPI_CTORATstpNegoOrderField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegoOrder( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderPacket { p_nego_order_field:p_nego_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_nego_order_action(&mut self, p_nego_order_action_field : Option<&TORASTOCKAPI_CTORATstpNegoOrderActionField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegoOrderAction( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderActionPacket { p_nego_order_action_field:p_nego_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_nego_trade(&mut self, p_nego_trade_field : Option<&TORASTOCKAPI_CTORATstpNegoTradeField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegoTrade( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoTradePacket { p_nego_trade_field:p_nego_trade_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_negotiation_param(&mut self, p_negotiation_param_field : Option<&TORASTOCKAPI_CTORATstpNegotiationParamField>,p_rsp_info_field : Option<&TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegotiationParam( TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegotiationParamPacket { p_negotiation_param_field:p_negotiation_param_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
             }

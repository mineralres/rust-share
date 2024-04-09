#![allow(non_upper_case_globals)]

use crate::error::Error;
use chrono::Local;
use clap::builder::NonEmptyStringValueParser;
use futures::StreamExt;
use itertools::Itertools;
use log::{error, info};
use rust_share::{ctp_futures, ctp_futures::*, rust_share_util::*};
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::atomic;
use std::sync::mpsc::channel;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

#[derive(Debug, PartialEq)]
pub enum DirectionType {
    Long = 0,
    Short = 1,
}

impl From<TThostFtdcDirectionType> for DirectionType {
    fn from(d: TThostFtdcDirectionType) -> Self {
        if d as u8 == THOST_FTDC_D_Buy {
            DirectionType::Long
        } else if d as u8 == THOST_FTDC_D_Sell {
            DirectionType::Short
        } else {
            panic!("unkown ctp direction={}", d);
        }
    }
}

impl DirectionType {
    pub fn opposite(&self) -> DirectionType {
        match *self {
            Self::Long => Self::Short,
            Self::Short => Self::Long,
        }
    }

    pub fn to_ctp_direction(&self) -> u8 {
        match *self {
            DirectionType::Long => THOST_FTDC_D_Buy,
            DirectionType::Short => THOST_FTDC_D_Sell,
        }
    }
}

pub enum PositionDateType {
    Today = 0,
    Yesterday = 1,
    Total = 2,
}

impl PositionDateType {
    pub fn to_ctp_offset(&self) -> u8 {
        match *self {
            PositionDateType::Total => THOST_FTDC_OF_Close,
            PositionDateType::Today => THOST_FTDC_OF_CloseToday,
            PositionDateType::Yesterday => THOST_FTDC_OF_CloseYesterday,
        }
    }
}

#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ClosePriorityType {
    #[default]
    AnyFirst,
    YesterdayFirst,
    TodayFirst,
}

#[derive(Debug, Clone)]
pub struct MarketDataSnapshot {
    pub ask1: f64,
    pub ask1_volume1: i32,
    pub bid1: f64,
    pub bid1_volume: i32,
    pub timestamp: i64,
}

impl From<&CThostFtdcDepthMarketDataField> for MarketDataSnapshot {
    fn from(src: &CThostFtdcDepthMarketDataField) -> Self {
        Self {
            ask1: src.AskPrice1,
            ask1_volume1: src.AskVolume1,
            bid1: src.BidPrice1,
            bid1_volume: src.BidVolume1,
            timestamp: 0,
        }
    }
}

/// 合约明细数据
#[derive(Clone)]
pub struct ContractDetail {
    pub exchange: String,
    pub symbol: String,
    pub pl: Vec<CThostFtdcInvestorPositionDetailField>, // 持仓明细
    pub ol: Vec<CThostFtdcOrderField>,                  // 委托明细
    pub tl: Vec<CThostFtdcTradeField>,                  // 成交明细
    pub target: Option<ContractPositionTarget>,         // 目标仓位
    pub price_tick: f64,                                // 最小变动价位
}

/// 操作类型
pub enum Operation {
    NOP,                                     // 无操作
    Input(CThostFtdcInputOrderField),        // 需要下单
    Cancel(CThostFtdcInputOrderActionField), // 需要撤单
}

impl Operation {
    pub fn or(self, opb: Operation) -> Operation {
        match self {
            Operation::NOP => opb,
            _ => self,
        }
    }
}

/// 合约目标仓位
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ContractPositionTarget {
    pub exchange: String,
    pub symbol: String,
    pub position: i32,                     // 目标数量
    pub shift: i32,                        // 撤单策略
    pub close_priority: ClosePriorityType, // 平仓优先顺序
}

impl ContractPositionTarget {
    pub fn direction(&self) -> DirectionType {
        if self.position > 0 {
            DirectionType::Long
        } else {
            DirectionType::Short
        }
    }
    pub fn opposite_direction(&self) -> DirectionType {
        if self.position > 0 {
            DirectionType::Long
        } else {
            DirectionType::Short
        }
    }
}

pub fn get_symbol(instrument_id: &[i8]) -> Result<&str, Error> {
    let instrument_id = unsafe {
        std::slice::from_raw_parts(instrument_id.as_ptr() as *mut u8, instrument_id.len())
    };
    for i in 0..instrument_id.len() {
        if instrument_id[i] == 0 {
            return unsafe { Ok(std::str::from_utf8_unchecked(&instrument_id[0..i])) };
        }
    }
    Err(Error::InvalidCtpInstrumentId)
}

/// 内存中维护一个交易账户的最新镜像
/// 可用于更新委托，成交等
/// 有些品种需要区分平今平昨
/// update_by_order的时候，如果order.status = Done先调用，但trade还没回来，那有较小概率导致多平一次仓
pub struct AccountState {
    pub broker_id: String,
    pub account: String,
    pub front_id: i32,
    pub session_id: i32,
    pub trading_day_i32: i32,
    pub trading_day_ctp: TThostFtdcDateType,
    pub sorted_cds: Vec<ContractDetail>,
    pub hm_inst: std::collections::HashMap<String, CThostFtdcInstrumentField>,
    pub request_id: atomic::AtomicI32,
    pub order_ref: atomic::AtomicI32,
}

impl AccountState {
    pub fn new(broker_id: &str, account: &str) -> Self {
        AccountState {
            broker_id: broker_id.to_owned(),
            account: account.to_owned(),
            sorted_cds: Vec::new(),
            front_id: 0,
            session_id: 0,
            order_ref: atomic::AtomicI32::new(1),
            trading_day_i32: 0,
            trading_day_ctp: [0; 9],
            request_id: atomic::AtomicI32::new(0),
            hm_inst: std::collections::HashMap::new(),
        }
    }

    /// 登陆数量更新
    pub fn on_login(&mut self, lf: &CThostFtdcRspUserLoginField) {
        self.front_id = lf.FrontID;
        self.session_id = lf.SessionID;
        // self.broker_id = ascii_cstr_to_str(lf.BrokerID).expect("").into();
        // self.account = ascii_cstr_to_str(lf.UserID).expect("").into();
        let mut max_order_ref = ascii_cstr_to_str_i8(&lf.MaxOrderRef)
            .expect("")
            .parse::<i32>()
            .unwrap();
        if max_order_ref < 1 {
            max_order_ref = 1;
        }
        self.order_ref
            .store(max_order_ref, atomic::Ordering::SeqCst);
        self.trading_day_ctp = lf.TradingDay.clone();
        self.trading_day_i32 = ascii_cstr_to_str_i8(&lf.TradingDay)
            .expect("")
            .parse::<i32>()
            .unwrap();
    }

    // get request_id
    pub fn get_request_id(&self) -> i32 {
        self.request_id.fetch_add(1, atomic::Ordering::SeqCst)
    }

    // get order_ref
    pub fn get_order_ref(&self) -> String {
        let order_ref = self.order_ref.fetch_add(1, atomic::Ordering::SeqCst);
        format!("{order_ref}")
    }

    // get contract detail
    fn get_contract_detail(&self, symbol: &str) -> Result<usize, usize> {
        self.sorted_cds
            .binary_search_by(|probe| probe.symbol.as_str().cmp(symbol))
    }

    // get instrument
    fn get_instrument(&self, instrument_id: &String) -> Option<&CThostFtdcInstrumentField> {
        self.hm_inst.get(instrument_id)
    }

    // 初始化持仓明细
    pub fn insert_position_detail(
        &mut self,
        pd: &CThostFtdcInvestorPositionDetailField,
    ) -> Result<(), Error> {
        if pd.TradingDay[0] == 0 || pd.OpenDate[0] == 0 {
            info!("insert trading_day==0 pd={:?}", pd);
            panic!("without trading_day account state cann't get the summation of position by date type.");
        }
        match self.get_contract_detail(get_symbol(&pd.InstrumentID).unwrap()) {
            Ok(i) => {
                self.sorted_cds[i].pl.push(pd.clone());
            }
            Err(i) => {
                let mut cd = ContractDetail::new_from_instrument_id(
                    &pd.ExchangeID,
                    &pd.InstrumentID,
                    self.hm_inst
                        .get(get_symbol(&pd.InstrumentID).unwrap())
                        .ok_or(Error::InstrumentNotFound)?
                        .PriceTick,
                );
                cd.pl.push(pd.clone());
                self.sorted_cds.insert(i, cd);
            }
        }
        Ok(())
    }

    // 通过OrderSysID查找order，并在成交返回时触发
    pub fn find_order_by_order_sys_id(
        &self,
        instrument_id: &TThostFtdcInstrumentIDType,
        order_sys_id: &[i8; 21],
    ) -> Option<&CThostFtdcOrderField> {
        match self.get_contract_detail(get_symbol(instrument_id).unwrap()) {
            Ok(i) => {
                let cd = &self.sorted_cds[i];
                cd.ol.iter().find(|x| x.OrderSysID == *order_sys_id)
            }
            Err(_i) => None,
        }
    }

    // 委托更新
    pub fn update_by_order(&mut self, o: &CThostFtdcOrderField) -> Result<(), Error> {
        match self.get_contract_detail(get_symbol(&o.InstrumentID).unwrap()) {
            Ok(i) => {
                let cd = &mut self.sorted_cds[i];
                let od = cd.ol.iter_mut().find(|x| {
                    x.FrontID == o.FrontID && x.SessionID == o.SessionID && x.OrderRef == o.OrderRef
                });
                match od {
                    Some(old) => {
                        *old = o.clone();
                    }
                    None => cd.ol.push(o.clone()),
                }
            }
            Err(_i) => {
                let mut cd = ContractDetail::new_from_instrument_id(
                    &o.ExchangeID,
                    &o.InstrumentID,
                    self.hm_inst
                        .get(get_symbol(&o.InstrumentID).unwrap())
                        .ok_or(Error::InstrumentNotFound)?
                        .PriceTick,
                );
                cd.ol.push(o.clone());
                self.sorted_cds.push(cd);
                self.sorted_cds.sort_by(|a, b| a.symbol.cmp(&b.symbol));
            }
        }
        Ok(())
    }

    // set order status
    pub fn set_order_status(
        &mut self,
        symbol: &str,
        front_id: i32,
        session_id: i32,
        order_ref: &[i8; 13],
        new_status: i8,
    ) {
        match self.get_contract_detail(symbol) {
            Ok(i) => {
                let cd = &mut self.sorted_cds[i];
                let od = cd.ol.iter_mut().find(|x| {
                    x.FrontID == front_id && x.SessionID == session_id && x.OrderRef == *order_ref
                });
                match od {
                    Some(old) => {
                        old.OrderStatus = new_status;
                    }
                    None => (),
                }
            }
            Err(_i) => (),
        }
    }

    // 更新成交列表
    // 同时更新本地持仓报表
    pub fn update_by_trade(&mut self, t: &CThostFtdcTradeField) -> Result<(), Error> {
        let offset = t.OffsetFlag as u8;
        let i = match self.get_contract_detail(get_symbol(&t.InstrumentID).unwrap()) {
            Ok(i) => {
                let cd = &mut self.sorted_cds[i];
                if cd.tl.iter().any(|x| {
                    x.ExchangeID == t.ExchangeID
                        && x.InstrumentID == t.InstrumentID
                        && x.TradeID == t.TradeID
                }) {
                    // 防止重复update
                    return Err(Error::DumplicateTrade);
                }
                cd.tl.push(t.clone());
                if offset == THOST_FTDC_OF_Open {
                    cd.pl.push(from_trade_to_position_detail(t));
                    return Ok(());
                }
                i
            }
            Err(i) => {
                let mut cd = ContractDetail::new_from_instrument_id(
                    &t.ExchangeID,
                    &t.InstrumentID,
                    self.hm_inst
                        .get(get_symbol(&t.InstrumentID).unwrap())
                        .ok_or(Error::InstrumentNotFound)?
                        .PriceTick,
                );
                cd.tl.push(t.clone());
                if offset == THOST_FTDC_OF_Open {
                    cd.pl.push(from_trade_to_position_detail(t));
                    return Ok(());
                }
                i
            }
        };
        let cd = &mut self.sorted_cds[i];
        let mut left_volume = t.Volume;
        for pd in cd.pl.iter_mut().filter(|pd| {
            if pd.Volume > 0
                && t.ExchangeID == pd.ExchangeID
                && t.InstrumentID == pd.InstrumentID
                && t.Direction != pd.Direction
            {
                // 平今 SHFE INE区分
                if ascii_cstr_i8_eq(&t.ExchangeID, "SHFE") || ascii_cstr_i8_eq(&t.ExchangeID, "INE")
                {
                    match offset {
                        THOST_FTDC_OF_Open => false,
                        THOST_FTDC_OF_CloseToday => {
                            t.TradeDate.cmp(&pd.OpenDate) == std::cmp::Ordering::Equal
                        }
                        THOST_FTDC_OF_CloseYesterday | THOST_FTDC_OF_Close => {
                            pd.OpenDate.cmp(&t.TradeDate) == std::cmp::Ordering::Less
                        }
                        _ => true,
                    }
                } else {
                    true
                }
            } else {
                false
            }
        }) {
            let v = std::cmp::min(left_volume, pd.Volume);
            pd.Volume -= v;
            left_volume -= v;
            if left_volume < 0 {
                panic!("left_volume < 0");
            }
            if left_volume == 0 {
                break;
            }
        }
        if left_volume > 0 {
            info!("trade={:?}", t);
            panic!("left_volume>0 无仓可平");
        }
        Ok(())
    }

    /// 查询合约最小变动价位
    pub fn get_price_tick(&self, instrument_id: &String) -> Result<f64, Error> {
        Ok(self
            .hm_inst
            .get(instrument_id)
            .ok_or(Error::InstrumentNotFound)?
            .PriceTick)
    }

    /// 初始化Operation的order_ref, request_id等
    pub fn fill_operation_parameter(&mut self, op: &mut Operation) {
        match op {
            Operation::Input(input) => {
                set_cstr_from_str_truncate_i8(&mut input.BrokerID, self.broker_id.as_str());
                set_cstr_from_str_truncate_i8(&mut input.UserID, self.account.as_str());
                set_cstr_from_str_truncate_i8(&mut input.InvestorID, self.account.as_str());
                let order_ref = self.get_order_ref();
                set_cstr_from_str_truncate_i8(&mut input.OrderRef, order_ref.as_str());
            }
            _ => (),
        }
    }

    /// 设置某合约的target 并返回对应的操作
    pub fn set_check_target(
        &mut self,
        target: ContractPositionTarget,
        md: &MarketDataSnapshot,
    ) -> Result<Operation, Error> {
        let instrument_id = &target.symbol;
        let price_tick = self.get_price_tick(instrument_id)?;
        let mut op = match self.get_contract_detail(instrument_id) {
            Ok(i) => {
                let cd = &mut self.sorted_cds[i];
                cd.target = Some(target);
                cd.check_target(md, price_tick, &self.trading_day_ctp)
            }
            Err(i) => {
                self.sorted_cds
                    .push(ContractDetail::new_from_target(target, price_tick));
                self.sorted_cds[i].check_target(md, price_tick, &self.trading_day_ctp)
            }
        };
        self.fill_operation_parameter(&mut op);
        Ok(op)
    }

    /// 检查合约的target,并返回相应操作
    pub fn check_target(
        &mut self,
        instrument_id: &String,
        md: &MarketDataSnapshot,
    ) -> Result<Operation, Error> {
        let price_tick = self.get_price_tick(instrument_id)?;
        let mut op = match self.get_contract_detail(instrument_id) {
            Ok(i) => {
                let cd = &mut self.sorted_cds[i];
                match cd.target.as_ref() {
                    Some(target) => cd.check_target(md, price_tick, &self.trading_day_ctp),
                    None => Operation::NOP,
                }
            }
            Err(_i) => Operation::NOP,
        };
        self.fill_operation_parameter(&mut op);
        Ok(op)
    }
}

impl ContractDetail {
    pub fn _new(exchange: &str, symbol: &str, price_tick: f64) -> Self {
        let cd = ContractDetail {
            exchange: exchange.into(),
            symbol: symbol.into(),
            price_tick,
            pl: vec![],
            ol: vec![],
            tl: vec![],
            target: None,
        };
        cd
    }

    pub fn new_from_instrument_id(
        exchange_id: &TThostFtdcExchangeIDType,
        instrument_id: &TThostFtdcInstrumentIDType,
        price_tick: f64,
    ) -> Self {
        let cd = ContractDetail {
            exchange: ascii_cstr_to_str_i8(exchange_id)
                .expect("exchange_id should be ascii_str")
                .into(),
            symbol: ascii_cstr_to_str_i8(instrument_id)
                .expect("instrument_id should be ascii_str")
                .into(),
            price_tick,
            pl: vec![],
            ol: vec![],
            tl: vec![],
            target: None,
        };
        cd
    }

    pub fn new_from_target(target: ContractPositionTarget, price_tick: f64) -> Self {
        let cd = ContractDetail {
            exchange: target.exchange.clone(),
            symbol: target.symbol.clone(),
            price_tick,
            pl: vec![],
            ol: vec![],
            tl: vec![],
            target: Some(target),
        };
        cd
    }

    pub fn summation(
        &self,
        d: &DirectionType,
        pos_date_type: &PositionDateType,
        trading_day: &TThostFtdcDateType,
    ) -> i32 {
        self.pl
            .iter()
            .filter(|detail| {
                if trading_day < &detail.TradingDay {
                    info!(
                        "trading_day={} detail.trading_day={} exchange={} symbol={}",
                        ascii_cstr_to_str_i8(trading_day).unwrap(),
                        ascii_cstr_to_str_i8(&detail.TradingDay).unwrap(),
                        &self.exchange,
                        &self.symbol
                    );
                    panic!("trading_day < detail.trading_day");
                }
                if detail.OpenDate[0] == 0i8 {
                    info!("开仓交易日为0 d={:?}", detail);
                }
                if DirectionType::from(detail.Direction) == *d {
                    match pos_date_type {
                        PositionDateType::Today => {
                            detail.OpenDate.cmp(trading_day) == std::cmp::Ordering::Equal
                        }
                        PositionDateType::Yesterday => {
                            detail.OpenDate.cmp(trading_day) == std::cmp::Ordering::Less
                        }
                        PositionDateType::Total => true,
                    }
                } else {
                    false
                }
            })
            .map(|x| x.Volume)
            .sum()
    }

    fn close_opposite(
        &self,
        opposite_direction: &DirectionType,
        pos_date_type: &PositionDateType,
        trading_day: &TThostFtdcDateType,
    ) -> Option<(i32, TThostFtdcDirectionType, TThostFtdcOffsetFlagType)> {
        let sum = self.summation(opposite_direction, pos_date_type, trading_day);
        if sum > 0 {
            Some((
                sum,
                opposite_direction.opposite().to_ctp_direction() as i8,
                pos_date_type.to_ctp_offset() as i8,
            ))
        } else {
            None
        }
    }

    fn close_same_direction(
        &self,
        direction: &DirectionType,
        pos_date_type: &PositionDateType,
        diff: i32,
        trading_day: &TThostFtdcDateType,
    ) -> Option<(i32, TThostFtdcDirectionType, TThostFtdcOffsetFlagType)> {
        let sum = self.summation(direction, pos_date_type, trading_day);
        if sum <= diff {
            Some((
                sum,
                direction.opposite().to_ctp_direction() as i8,
                pos_date_type.to_ctp_offset() as i8,
            ))
        } else {
            Some((
                diff,
                direction.opposite().to_ctp_direction() as i8,
                pos_date_type.to_ctp_offset() as i8,
            ))
        }
    }

    /// 平反向昨仓
    fn close_position(
        &self,
        target: &ContractPositionTarget,
        md: &MarketDataSnapshot,
        price_tick: f64,
        trading_day: &TThostFtdcDateType,
    ) -> Operation {
        let d = target.direction();
        let opd = target.opposite_direction();

        // 1. 先平反向仓
        let op = match target.close_priority {
            ClosePriorityType::AnyFirst => {
                self.close_opposite(&opd, &PositionDateType::Total, trading_day)
            }
            ClosePriorityType::YesterdayFirst => self
                .close_opposite(&opd, &PositionDateType::Yesterday, trading_day)
                .or(self.close_opposite(&opd, &PositionDateType::Today, trading_day)),
            ClosePriorityType::TodayFirst => self
                .close_opposite(&opd, &PositionDateType::Today, trading_day)
                .or(self.close_opposite(&opd, &PositionDateType::Yesterday, trading_day)),
        };
        // 2. 再平同向仓
        let op = op.or({
            let target_position = target.position.abs();
            let current_total = self.summation(&d, &PositionDateType::Total, trading_day);
            let diff = target_position - current_total;
            if diff > 0 {
                match target.close_priority {
                    ClosePriorityType::AnyFirst => {
                        self.close_same_direction(&d, &PositionDateType::Total, diff, trading_day)
                    }
                    ClosePriorityType::YesterdayFirst => self
                        .close_same_direction(&d, &PositionDateType::Yesterday, diff, trading_day)
                        .or(self.close_same_direction(
                            &d,
                            &PositionDateType::Today,
                            diff,
                            trading_day,
                        )),
                    ClosePriorityType::TodayFirst => self
                        .close_same_direction(&d, &PositionDateType::Today, diff, trading_day)
                        .or(self.close_same_direction(
                            &d,
                            &PositionDateType::Yesterday,
                            diff,
                            trading_day,
                        )),
                }
            } else {
                None
            }
        });
        match op {
            Some((volume, direction, offset)) => {
                info!(
                    "{}:{} close potition volume={volume} direction={direction} offset={offset}",
                    &self.exchange, &self.symbol
                );
                let mut input = CThostFtdcInputOrderField::default();
                set_cstr_from_str_truncate_i8(&mut input.ExchangeID, self.exchange.as_str());
                set_cstr_from_str_truncate_i8(&mut input.InstrumentID, self.symbol.as_str());
                input.Direction = direction;
                input.CombOffsetFlag[0] = offset;
                input.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation as i8;
                input.LimitPrice = get_counterparty_price(
                    md,
                    DirectionType::from(direction),
                    price_tick,
                    target.shift,
                );
                input.VolumeTotalOriginal = volume;
                input.OrderPriceType = THOST_FTDC_OPT_LimitPrice as i8;
                if input.LimitPrice == 0.0 {
                    input.OrderPriceType = THOST_FTDC_OPT_AnyPrice as i8;
                }
                input.TimeCondition = THOST_FTDC_TC_GFD as i8;
                input.VolumeCondition = THOST_FTDC_VC_AV as i8;
                input.MinVolume = 1;
                input.ContingentCondition = THOST_FTDC_CC_Immediately as i8;
                input.ForceCloseReason = THOST_FTDC_FCC_NotForceClose as i8;
                input.IsAutoSuspend = 0;
                input.UserForceClose = 0;
                Operation::Input(input)
            }
            None => Operation::NOP,
        }
    }

    /// 开同向仓
    fn open_position(
        &self,
        target: &ContractPositionTarget,
        md: &MarketDataSnapshot,
        price_tick: f64,
        trading_day: &TThostFtdcDateType,
    ) -> Operation {
        let d = target.direction();
        let current = self.summation(&d, &PositionDateType::Total, trading_day);
        if current < target.position.abs() {
            info!(
                "{}:{} current_position={} od={:?} &md.TradingDay={} target={}",
                self.exchange,
                self.symbol,
                current,
                target.direction(),
                ascii_cstr_to_str_i8(trading_day).unwrap(),
                target.position
            );
            // 开仓
            let mut input = CThostFtdcInputOrderField::default();
            set_cstr_from_str_truncate_i8(&mut input.ExchangeID, self.exchange.as_str());
            set_cstr_from_str_truncate_i8(&mut input.InstrumentID, self.symbol.as_str());
            input.Direction = d.to_ctp_direction() as i8;
            input.CombOffsetFlag[0] = THOST_FTDC_OF_Open as i8;
            input.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation as i8;
            input.LimitPrice = get_counterparty_price(md, d, price_tick, target.shift);
            input.VolumeTotalOriginal = target.position.abs() - current;
            input.OrderPriceType = THOST_FTDC_OPT_LimitPrice as i8;
            if input.LimitPrice == 0.0 {
                input.OrderPriceType = THOST_FTDC_OPT_AnyPrice as i8;
            }
            input.TimeCondition = THOST_FTDC_TC_GFD as i8;
            input.VolumeCondition = THOST_FTDC_VC_AV as i8;
            input.MinVolume = 1;
            input.ContingentCondition = THOST_FTDC_CC_Immediately as i8;
            input.ForceCloseReason = THOST_FTDC_FCC_NotForceClose as i8;
            input.IsAutoSuspend = 0;
            input.UserForceClose = 0;
            return Operation::Input(input);
        }
        Operation::NOP
    }

    // 检查持仓是否与目标一致，如果不一致，则返回相应的操作.
    fn check_target(
        &self,
        md: &MarketDataSnapshot,
        price_tick: f64,
        trading_day: &TThostFtdcDateType,
    ) -> Operation {
        match &self.target {
            Some(target) => {
                // 1 先撤反向平单(开或平)
                for o in self.ol.iter().filter(|o| {
                    if o.OrderStatus as u8 != THOST_FTDC_OST_Canceled
                        && o.OrderStatus as u8 != THOST_FTDC_OST_AllTraded
                        && o.OrderStatus as u8 != THOST_FTDC_OST_NoTradeNotQueueing
                    {
                        return true;
                    }
                    false
                }) {
                    let mut r = CThostFtdcInputOrderActionField::default();
                    r.ExchangeID = o.ExchangeID.clone();
                    r.InstrumentID = o.InstrumentID.clone();
                    r.FrontID = o.FrontID;
                    r.SessionID = o.SessionID;
                    r.OrderRef = o.OrderRef.clone();
                    r.OrderSysID = o.OrderSysID;
                    r.LimitPrice = o.LimitPrice;
                    return Operation::Cancel(r);
                }
                // 先平仓，再开仓
                self.close_position(target, md, price_tick, trading_day)
                    .or(self.open_position(target, md, price_tick, trading_day))
                // 考虑到股票的情况，既不分平今平昨，同时还有最小交易量的限制，导致平仓的时候可能出现如昨仓还剩下2股，今仓8股，但最小交易量为10股/手，
                // 这时平不出去，如果先平今的话，8股也不够一手
            }
            None => Operation::NOP,
        }
    }
}

// 把trade转position detail,方便本地计算持仓
fn from_trade_to_position_detail(
    src: &CThostFtdcTradeField,
) -> CThostFtdcInvestorPositionDetailField {
    let mut output = CThostFtdcInvestorPositionDetailField::default();
    output.InstrumentID = src.InstrumentID;
    output.ExchangeID = src.ExchangeID;
    output.Direction = src.Direction;
    output.OpenPrice = src.Price;
    output.Volume = src.Volume;
    output.TradeID = src.TradeID;
    output.OpenDate = src.TradingDay;
    output
}

// 对手价
// 1 注意 bid , ask 可能因极端行情而特别偏离当前价.
fn get_counterparty_price(
    md: &MarketDataSnapshot,
    d: DirectionType,
    price_tick: f64,
    shift: i32,
) -> f64 {
    match d {
        DirectionType::Long => md.ask1 - price_tick * shift as f64,
        DirectionType::Short => md.bid1 + price_tick * shift as f64,
    }
}

pub struct CtpMdCache {
    pub tx: mpsc::Sender<String>,
    pub hm_md: HashMap<String, MarketDataSnapshot>,
    pub subscribed: Vec<String>,
}

pub async fn run_ctp_md_cache(
    ca: CtpAccountConfig,
    mut rx: mpsc::Receiver<String>,
    cmc: Arc<Mutex<CtpMdCache>>,
) {
    info!("[{}] market data receiver start", ca.account);
    if ca.md_fronts.len() == 0 {
        panic!("md front is not valid");
    }
    let flow_path = format!(".cache/ctp_md_flow_{}_{}//", ca.broker_id, ca.account);
    check_make_dir(&flow_path);
    let mut mdapi = unsafe {
        Box::from_raw(rust_share::ctp_futures::CThostFtdcMdApi_CreateFtdcMdApi(
            flow_path.as_ptr() as *const i8,
            false,
            false,
        ))
    };
    for front in ca.md_fronts.iter() {
        info!("Md RegisterFront {}", front);
        mdapi.register_front(CString::new(front.as_str()).unwrap());
    }

    let mut stream = {
        let (stream, pp) = ctp_futures::md_api::create_spi();
        mdapi.register_spi(pp);
        stream
    };
    mdapi.init();
    let mut trading_day: [i8; 9] = [0; 9];
    let mut initialized = false;
    while let Some(msg) = stream.next().await {
        use rust_share::ctp_futures::md_api::CThostFtdcMdSpiOutput::*;
        match msg {
            OnFrontConnected(ref _p) => {
                info!("[{}] 行情服务器FrontConnected", ca.account);
                let mut req_login: CThostFtdcReqUserLoginField = Default::default();
                let ret = mdapi.req_user_login(&mut req_login, 3);
                info!("行情ReqUserLogin={}", ret);
            }
            OnFrontDisconnected(ref p) => {
                info!(
                    "Front disconnected {:?} 直接Exit trading_day={}",
                    p,
                    ascii_cstr_to_str_i8(&trading_day).unwrap()
                );
                break;
            }
            OnRspUserLogin(ref p) => {
                if p.p_rsp_info.is_some() {
                    let u = p.p_rsp_user_login.unwrap();
                    trading_day = u.TradingDay.clone();
                    info!(
                        "行情服务器登陆成功, trading_day=[{}]",
                        ascii_cstr_to_str_i8(&trading_day).unwrap()
                    );
                    initialized = true;
                } else {
                    info!(
                        "Quote Md RspUserLogin = {:?}",
                        print_rsp_info!(&p.p_rsp_info)
                    );
                }
                break;
            }
            OnRspError(ref p) => {
                info!("Quote daemon OnRspError!");
                if let Some(info) = p.p_rsp_info {
                    info!(
                        "ErrorID={}, ErrorMsg={}",
                        info.ErrorID,
                        ascii_cstr_to_str_i8(&info.ErrorMsg).unwrap().to_string()
                    );
                }
            }
            _ => {
                info!("msg={:?}", msg)
            }
        }
    }
    if !initialized {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        error!("[{}] ctp md failed to initialize", ca.account);
        info!("Restart md cache in 10 seconds");
        test_spawn(ca, rx, cmc);
        return;
    }
    let subscribed = { cmc.lock().await.subscribed.clone() };
    let result = mdapi.subscribe_market_data(
        subscribed
            .iter()
            .map(|e| CString::new(e.clone()).unwrap())
            .collect_vec(),
        subscribed.len() as i32,
    );
    if result != 0 {
        error!("Subscribe result = {}", result);
    }
    loop {
        tokio::select! {
            Some(msg) = stream.next() => {
                use rust_share::ctp_futures::md_api::CThostFtdcMdSpiOutput::*;
                match msg {
                    OnFrontDisconnected(ref p) => {
                        info!(
                            "on front disconnected {:?} 直接Exit trading_day={}",
                            p,
                            ascii_cstr_to_str_i8(&trading_day).unwrap()
                        );
                        break;
                    }
                    OnRtnDepthMarketData(ref ctp_md) => {
                        if let Some(ctp_md) = ctp_md.p_depth_market_data.as_ref() {
                            let symbol = get_symbol(&ctp_md.InstrumentID).unwrap().to_string();
                            let symbol1 = symbol.clone();
                            let mut cmc = cmc.lock().await;
                                cmc.hm_md
                                .entry(symbol)
                                .and_modify(|e| {
                                    e.ask1 = ctp_md.AskPrice1;
                                    e.bid1 = ctp_md.BidPrice1;
                                    e.ask1_volume1 = ctp_md.AskVolume1;
                                    e.bid1_volume = ctp_md.BidVolume1;
                                    e.timestamp = Local::now().timestamp();
                                    info!("[{symbol1}] update md={:?}", e);
                                })
                                .or_insert_with(|| {
                                    let mut md = MarketDataSnapshot::from(ctp_md);
                                    md.timestamp = Local::now().timestamp();
                                    info!("[{symbol1}] insert md={:?}", md);
                                    md
                                });
                        }
                    }
                    _ => (),
                }
            }
            Some(symbol) = rx.recv() => {
                let result = mdapi.subscribe_market_data(vec![CString::new(symbol).unwrap()], 1);
                if result != 0 {
                    error!("Subscribe result = {}", result);
                }
            }
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // tokio::spawn(Box::pin(run_ctp_md_cache(ca, rx, cmc)));
    test_spawn(ca, rx, cmc);
}

fn test_spawn(ca: CtpAccountConfig, rx: mpsc::Receiver<String>, cmc: Arc<Mutex<CtpMdCache>>) {
    tokio::spawn(async move { Box::pin(run_ctp_md_cache(ca, rx, cmc)).await });
}

impl CtpMdCache {
    pub async fn subscribe(&mut self, exchange: &str, symbol: &str) {
        let symbol = symbol.to_string();
        if let None = self.subscribed.iter().find(|&e| *e == symbol) {
            self.subscribed.push(symbol.clone());
            if let Err(e) = self.tx.send(symbol).await {
                error!(" send subscribe {e}");
            }
        }
    }

    pub fn get_md(&self, k: &String) -> Option<&MarketDataSnapshot> {
        self.hm_md.get(k)
    }

    pub fn new(ca: &CtpAccountConfig) -> Arc<Mutex<Self>> {
        let (tx, rx) = mpsc::channel::<String>(1000);
        let cmc = Arc::new(Mutex::new(Self {
            tx,
            hm_md: HashMap::new(),
            subscribed: vec![],
        }));
        tokio::spawn(run_ctp_md_cache(ca.clone(), rx, Arc::clone(&cmc)));
        cmc
    }
}

#[derive(Debug)]
pub enum ReqMessage {
    SetContractTarget(ContractPositionTarget),
    QueryPositionDetail,
    QueryTradingAccount,
}

pub type RspMessage = Result<Vec<ctp_futures::trader_api::CThostFtdcTraderSpiOutput>, Error>;

/// executor for all account
/// 行情存放在hm_md中, 按需取
/// trader运行后,接收set_symbol_target指令
pub struct Executor {
    pub sorted_accounts: Vec<TraderHandle>,
}

pub struct TraderHandle {
    pub account: String,
    pub tx: mpsc::Sender<(ReqMessage, oneshot::Sender<RspMessage>)>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            sorted_accounts: vec![],
        }
    }

    pub async fn query(&self, account: &str, msg: ReqMessage) -> Result<RspMessage, Error> {
        match self.sorted_accounts.iter().find(|t| t.account == account) {
            Some(th) => {
                let (tx, rx) = oneshot::channel::<RspMessage>();
                th.tx
                    .send((msg, tx))
                    .await
                    .map_err(|_e| Error::MpscSendErr)?;
                match tokio::time::timeout(tokio::time::Duration::from_secs(5), rx).await {
                    Ok(res) => match res {
                        Ok(v) => Ok(v),
                        Err(_) => Err(Error::CtpQueryTimeout),
                    },
                    Err(_) => {
                        error!("did not receive value within 5 seconds");
                        Err(Error::CtpQueryTimeout)
                    }
                }
            }
            None => Err(Error::AccountNotFound),
        }
    }

    pub async fn handle_spi_msg(
        spi_msg: &ctp_futures::trader_api::CThostFtdcTraderSpiOutput,
        state: &mut AccountState,
        ca: &CtpAccountConfig,
        cmc: &Arc<Mutex<CtpMdCache>>,
    ) -> Result<(), Error> {
        use rust_share::ctp_futures::trader_api::CThostFtdcTraderSpiOutput::*;
        match spi_msg {
            OnFrontDisconnected(p) => {
                info!(
                    "{}:{} on front disconnected {:?} 直接Exit ",
                    ca.broker_id, ca.account, p
                );
                return Err(Error::FrontDisconnected);
            }
            OnRtnTradingNotice(ref p) => {
                info!("{}:{} RtnTradingNotice = {:?}", ca.broker_id, ca.account, p);
            }
            OnRspError(ref p) => {
                info!(
                    "{}:{} RspError={:?}",
                    ca.broker_id,
                    ca.account,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnRspOrderInsert(ref p) => {
                // 需要构建order通知撤单
                info!(
                    "{}:{} RspOrderInsert={:?}",
                    ca.broker_id,
                    ca.account,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnRspOrderAction(ref p) => {
                info!(
                    "{}:{} OnRspOrderAction={:?}",
                    ca.broker_id,
                    ca.account,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnErrRtnOrderInsert(ref p) => {
                // 需要构建order通知撤单
                if let Some(input) = p.p_input_order.as_ref() {
                    state.set_order_status(
                        get_symbol(&input.InstrumentID).unwrap(),
                        state.front_id,
                        state.session_id,
                        &input.OrderRef,
                        THOST_FTDC_OST_Canceled as i8,
                    );
                    info!(
                        "{}:{} 删除发送失败的委托 OrderRef={} result={:?}",
                        ca.broker_id,
                        ca.account,
                        ascii_cstr_to_str_i8(&input.OrderRef).unwrap(),
                        print_rsp_info!(&p.p_rsp_info)
                    );
                }
            }
            OnErrRtnOrderAction(ref p) => {
                // info!("ErrRtnOrderAction={:?}", print_rsp_info!(&p.p_rsp_info));
                info!(
                    "{}:{} ErrRtnOrderAction={:?} {:?}",
                    ca.broker_id,
                    ca.account,
                    p,
                    print_rsp_info!(&p.p_rsp_info)
                );
            }
            OnRtnOrder(ref rtn) => {
                match rtn.p_order.as_ref() {
                    Some(o) => {
                        let submit_status = o.OrderSubmitStatus;
                        state.update_by_order(o).unwrap();
                        if is_order_canceled(o) {
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
                            // 完成成交的委托要放到成交回报里面处理，以避免成交未计算持仓导致重复开仓
                            let status_msg = ascii_cstr_to_str_i8(&o.StatusMsg).unwrap();
                            if status_msg.contains("当前状态禁止")
                                || status_msg.contains("废单")
                                || submit_status == ctp_futures::THOST_FTDC_OSS_CancelRejected as i8
                                || submit_status == ctp_futures::THOST_FTDC_OSS_InsertRejected as i8
                            {
                                info!("非交易时间不发单, OrderSubmitStatus={}", submit_status);
                            } else {
                                // 价格变化导致的撤单，要及时重新发单
                                // state.set_check_target(instrument_id, target, md)
                            }
                        }
                        cmc.lock()
                            .await
                            .subscribe("", ascii_cstr_to_str_i8(&o.InstrumentID).unwrap())
                            .await;
                    }
                    None => error!("RtnOrder rtn=nil"),
                }
            }
            OnRtnTrade(ref rtn) => {
                match rtn.p_trade.as_ref() {
                    Some(trade) => {
                        // rtn.TradingDay = state.trading_day_ctp.clone(); // 上海夜盘成交的交易日没有更新到第二天
                        state.update_by_trade(trade).unwrap();
                        cmc.lock()
                            .await
                            .subscribe("", ascii_cstr_to_str_i8(&trade.InstrumentID).unwrap())
                            .await;
                        // 只要有成交都重新触发
                        // info!(
                        //     "重新触发sync symbol target {}:{} position={}",
                        //     target.exchange, target.symbol, target.position
                        // );
                    }
                    None => error!("RtnTrade rtn=nil"),
                }
            }
            OnRspQryTradingAccount(ref p) => {}
            OnRspQryInvestorPosition(ref p) => {
                let _pos = match p.p_investor_position {
                    Some(ref _ip) => (),
                    None => (),
                };
            }
            OnRspQryInvestorPositionDetail(ref p) => {
                let _detail = match p.p_investor_position_detail {
                    Some(ref _pd) => (),
                    None => (),
                };
            }
            other => {
                info!("Un handled spi msg = {:?}", other);
            }
        };
        Ok(())
    }

    pub async fn handle_set_contract_target(
        target: ContractPositionTarget,
        state: &mut AccountState,
        ca: &CtpAccountConfig,
        cmc: &Arc<Mutex<CtpMdCache>>,
        api: &mut Box<CThostFtdcTraderApi>,
    ) -> Result<(), Error> {
        let md = { cmc.lock().await.get_md(&target.symbol).cloned() };
        match md {
            Some(md) => {
                let op = state.set_check_target(target, &md);
                Ok(())
            }
            None => Err(Error::MdNotFound),
        }
    }

    pub async fn handle_request_msg(
        req_msg: &ReqMessage,
        state: &mut AccountState,
        ca: &CtpAccountConfig,
        cmc: &Arc<Mutex<CtpMdCache>>,
        api: &mut Box<CThostFtdcTraderApi>,
    ) -> Result<(), Error> {
        use ReqMessage::*;
        match req_msg {
            SetContractTarget(target) => {}
            QueryPositionDetail => {
                info!("req_msg={:?}", req_msg);
                let mut req = CThostFtdcQryInvestorPositionDetailField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                let result = api.req_qry_investor_position_detail(&mut req, state.get_request_id());
                if result != 0 {
                    info!("ReqQueryInvestorPositionDetail={}", result);
                }
            }
            QueryTradingAccount => {
                let mut req = CThostFtdcQryTradingAccountField::default();
                set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                let result = api.req_qry_trading_account(&mut req, state.get_request_id());
                if result != 0 {
                    info!("ReqQueryTradingAccount={}", result);
                }
            }
        }
        info!("handle_request_msg = {:?}", req_msg);
        Ok(())
    }

    pub async fn run_trader_daemon(
        ca: CtpAccountConfig,
        cmc: Arc<Mutex<CtpMdCache>>,
        mut rx: mpsc::Receiver<(ReqMessage, oneshot::Sender<RspMessage>)>,
    ) -> Result<(), Error> {
        info!("run trader [{}]", ca.account);
        let mut state = AccountState::new(&ca.broker_id, &ca.account);
        let flow_path = format!(
            ".cache/ctp_trade_daemon_flow_{}_{}//",
            ca.broker_id, ca.account
        );
        check_make_dir(&flow_path);
        let mut api = rust_share::ctp_futures::trader_api::create_api(&flow_path, false);
        let mut stream = {
            let (stream, pp) = rust_share::ctp_futures::trader_api::create_spi();
            api.register_spi(pp);
            stream
        };
        if ca.name_servers.len() > 0 {
            api.register_name_server(CString::new(ca.name_servers[0].as_str()).unwrap());
        } else if ca.trade_fronts.len() > 0 {
            for front in ca.trade_fronts.iter() {
                api.register_front(CString::new(front.as_str()).unwrap());
            }
        }
        api.subscribe_public_topic(rust_share::ctp_futures::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.subscribe_private_topic(rust_share::ctp_futures::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
        api.init();
        // 处理登陆初始化查询
        while let Some(spi_msg) = stream.next().await {
            use rust_share::ctp_futures::trader_api::CThostFtdcTraderSpiOutput::*;
            match spi_msg {
                OnFrontConnected(_p) => {
                    let mut req = CThostFtdcReqAuthenticateField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.UserID, &ca.account);
                    set_cstr_from_str_truncate_i8(&mut req.AuthCode, &ca.auth_code);
                    set_cstr_from_str_truncate_i8(&mut req.UserProductInfo, &ca.user_product_info);
                    set_cstr_from_str_truncate_i8(&mut req.AppID, &ca.app_id);
                    api.req_authenticate(&mut req, state.get_request_id());
                }
                OnFrontDisconnected(p) => {
                    info!("on front disconnected {:?} 直接Exit ", p);
                    return Err(Error::FrontDisconnected);
                }
                OnRspAuthenticate(ref p) => {
                    if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                        let mut req = CThostFtdcReqUserLoginField::default();
                        set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                        set_cstr_from_str_truncate_i8(&mut req.UserID, &ca.account);
                        set_cstr_from_str_truncate_i8(&mut req.Password, &ca.password);
                        api.req_user_login(&mut req, state.get_request_id());
                    } else {
                        info!("RspAuthenticate={:?}", p);
                        return Err(Error::CtpAuthFailed);
                    }
                }
                OnRspUserLogin(ref p) => {
                    if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                        let u = p.p_rsp_user_login.as_ref().unwrap();
                        state.on_login(u);
                        info!(
                            "登陆成功 trading_day={} front_id={} session_id={}",
                            state.trading_day_i32, state.front_id, state.session_id
                        );
                    } else {
                        info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                    }
                    let mut req = CThostFtdcSettlementInfoConfirmField::default();
                    set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                    set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                    let result = api.req_settlement_info_confirm(&mut req, state.get_request_id());
                    if result != 0 {
                        info!("ReqSettlementInfoConfirm={}", result);
                    }
                }
                OnRspSettlementInfoConfirm(ref _p) => {
                    let mut req = CThostFtdcQryInstrumentField::default();
                    let result = api.req_qry_instrument(&mut req, state.get_request_id());
                    if result != 0 {
                        info!("ReqQryInstrument = {:?}", result);
                    }
                }
                OnRspQryInvestorPositionDetail(ref detail) => {
                    if let Some(pd) = &detail.p_investor_position_detail {
                        info!("pd={:?}", get_symbol(&pd.InstrumentID).unwrap());
                        state.insert_position_detail(pd).unwrap();
                    }
                    if detail.b_is_last {
                        let mut req = CThostFtdcQryOrderField::default();
                        set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                        set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        let result = api.req_qry_order(&mut req, state.get_request_id());
                        if result != 0 {
                            info!("ReqQryOrder = {:?}", result);
                        }
                    }
                }
                OnRspQryOrder(ref p) => {
                    if let Some(o) = &p.p_order {
                        state.update_by_order(&o).unwrap();
                    }
                    if p.b_is_last {
                        break;
                    }
                }
                OnRspQryInstrument(ref p) => {
                    if let Some(i) = &p.p_instrument {
                        state
                            .hm_inst
                            .insert(get_symbol(&i.InstrumentID).unwrap().into(), i.clone());
                    }
                    if p.b_is_last {
                        // let trading_day = &state.trading_day_ctp;
                        // 登陆成功之后从redis开始拉取消息
                        let mut req = CThostFtdcQryInvestorPositionDetailField::default();
                        set_cstr_from_str_truncate_i8(&mut req.BrokerID, &ca.broker_id);
                        set_cstr_from_str_truncate_i8(&mut req.InvestorID, &ca.account);
                        let result =
                            api.req_qry_investor_position_detail(&mut req, state.get_request_id());
                        if result != 0 {
                            info!("ReqQryInvestorPositionDetail = {:?}", result);
                        }
                    }
                }
                OnRtnInstrumentStatus(ref p) => match p.p_instrument_status {
                    Some(status) => {
                        let s = match status.InstrumentStatus as u8 as char {
                            '0' => "THOST_FTDC_IS_BeforeTrading",
                            '1' => "THOST_FTDC_IS_NoTrading",
                            '2' => "THOST_FTDC_IS_Continous",
                            '3' => "THOST_FTDC_IS_AuctionOrdering",
                            '4' => "THOST_FTDC_IS_AuctionBalance",
                            '5' => "THOST_FTDC_IS_AuctionMatch",
                            '6' => "THOST_FTDC_IS_Closed",
                            _ => "UnkownInstrumentStatus",
                        };
                        info!(
                            "RtnInstrumentStatus = {}:{} {}",
                            ascii_cstr_to_str_i8(&status.ExchangeID)
                                .unwrap()
                                .to_string(),
                            ascii_cstr_to_str_i8(&status.InstrumentID)
                                .unwrap()
                                .to_string(),
                            s
                        );
                    }
                    None => (),
                },
                OnRtnTradingNotice(ref p) => {
                    info!("RtnTradingNotice = {:?}", p);
                }
                _ => {}
            }
        }
        info!("{} 初始化查询完成.", ca.account);
        let (api, _api2) = rust_share::ctp_futures::trader_api::unsafe_clone_api(api);
        let (mut api, _api3) = rust_share::ctp_futures::trader_api::unsafe_clone_api(api);

        info!("{}:{} Trader initialized.", ca.broker_id, ca.account);
        {
            let mut cmc = cmc.lock().await;
            for cd in state.sorted_cds.iter() {
                cmc.subscribe(&cd.exchange, &cd.symbol).await;
            }
        }

        let mut query_req: Option<(
            ReqMessage,
            oneshot::Sender<RspMessage>,
            Vec<rust_share::ctp_futures::trader_api::CThostFtdcTraderSpiOutput>,
        )> = None;
        loop {
            tokio::select! {
                Some(spi_msg) = stream.next() => {
                    let _ = Executor::handle_spi_msg(&spi_msg, &mut state, &ca, &cmc).await?;
                    use rust_share::ctp_futures::trader_api::CThostFtdcTraderSpiOutput::*;
                    use ReqMessage::*;
                    if let Some((req_msg, rsp_tx, mut response_packets)) = query_req.take() {
                        let (is_result, is_last) = match (req_msg, &spi_msg) {
                            (SetContractTarget(_), _) => panic!("SetContractTarget do not have response"),
                            (QueryPositionDetail, OnRspQryInvestorPositionDetail(p)) => (true, p.b_is_last),
                            (QueryTradingAccount, OnRspQryTradingAccount(p)) => (true, p.b_is_last),
                            (_, _) => (false, false),
                        };
                        if is_result {
                            response_packets.push(spi_msg);
                        }
                        if is_last {
                            let _ = rsp_tx.send(Ok(response_packets));
                            query_req = None;
                        }
                    }
                },
                Some((req_msg, rsp_tx)) = rx.recv() => {
                    if let ReqMessage::SetContractTarget(target) = req_msg {
                        let res = Executor::handle_set_contract_target(target, &mut state, &ca, &cmc, &mut api).await.map(|_|vec![]);
                        let _ = rsp_tx.send(res);
                    } else {
                        if query_req.is_some() {
                            let _ = rsp_tx.send(Err(Error::CtpLastQueryIsProceeding));
                        } else {
                            query_req = Some((req_msg, rsp_tx, vec![]));
                            Executor::handle_request_msg(&query_req.as_ref().unwrap().0, &mut state, &ca, &cmc, &mut api).await?;
                        }
                    }
                },
                else => break,
            }
        }

        api.join();
        api.release();
        info!("{}:{} trader_deamon退出", ca.broker_id, ca.account);
        Ok(())
    }
}

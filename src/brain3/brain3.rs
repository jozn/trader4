use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::dc_intel::{DCParent, FrameMem};
use crate::gate_api::{GateWay, NewPos, PosRes, UpdatePos};
use crate::ne::{NEFrame, NERoot};
use crate::offline::num5;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub type PairCandleCfg = (Pair, CandleConfig);

#[derive(Debug)]
pub struct Brain3 {
    pub con: Box<Arc<dyn GateWay>>,
    pub acted: HashSet<String>,
    pub open_pos: HashMap<u64, PosRes>,
    // From PairMemo
    pub pair: Pair,
    pub last_tick: Option<Tick>,
    pub last_trade_time: u64,
    pub ticks_arr: TimeSerVec<Tick>,
    pub dc_intl: DCParent,
    pub ne: NERoot,
}

impl Brain3 {
    pub fn new(backend: Arc<impl GateWay + 'static>, pair_conf: (Pair, CandleConfig)) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            last_trade_time: 0,
            ticks_arr: Default::default(),
            acted: Default::default(),
            open_pos: Default::default(),
            pair: Pair::EURUSD,
            last_tick: None,
            dc_intl: DCParent::new(),
            ne: NERoot::new(),
        };

        brain
    }

    // Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        println!("on_connect Brain2");
    }

    pub fn on_notify_position(&mut self, pos: PosRes) {
        // println!(">>> {:?}", pos);
        if pos.is_closed {
            self.open_pos.remove(&pos.pos_id);
        } else {
            self.open_pos.insert(pos.pos_id, pos);
            self.update_all_tailing_pos();
        }
    }
}

impl Brain3 {
    pub fn go_long2(&mut self, symbol_id: i64, kline_id: u64, tick: &Tick, frame: &NEFrame) {
        // let atr_pip = ta_big.atr * 10_000.;
        // let atr_pip = ta_med.atr * 10_000.;
        // let atr_pip = 12.;
        let atr_pip = frame.atr_p * 1.;
        // let profit_pip = atr_pip * 0.6;
        let profit_pip = atr_pip * 1.;
        // let loose_pip = -atr_pip * 0.6;
        let loose_pip = -atr_pip * 1.;
        // let atr_pip = 10.;
        let np = NewPos {
            symbol_id,
            is_short: false,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, profit_pip), // 10 pip
            stop_loose_price: cal_price(tick.price_raw, loose_pip),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            frame_ne: frame.clone(),
            // frame: frame.clone(),
            // ta_med: ta_med.clone(),
            // ta_big: ta_big.clone(),
            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        // println!("Open long {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    pub fn go_short2(&mut self, symbol_id: i64, kline_id: u64, tick: &Tick, frame: &NEFrame) {
        // let atr_pip = ta_big.atr * 10_000.;
        // let atr_pip = ta_med.atr * 10_000.;
        // let atr_pip = 12.;
        let atr_pip = frame.atr_p * 1.;
        // let profit_pip = atr_pip * 0.6;
        let profit_pip = -atr_pip * 1.;
        // let loose_pip = -atr_pip * 0.6;
        let loose_pip = atr_pip * 1.;
        // let atr_pip = 10.;
        let np = NewPos {
            symbol_id,
            is_short: true,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, profit_pip), // 10 pip
            stop_loose_price: cal_price(tick.price_raw, loose_pip),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            frame_ne: frame.clone(),
            // frame: frame.clone(),
            // ta_med: ta_med.clone(),
            // ta_big: ta_big.clone(),
            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        // println!("Open long {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    pub fn already_acted(&mut self, symbol_id: i64, kline_id: u64) -> bool {
        let time_sec = self.con.get_time_ms() / 1000;
        // println!("lat: {}", time_sec);
        if time_sec < self.last_trade_time + 1800 {
            return true;
        }

        let kids = format!("{}_{}", symbol_id, kline_id);
        if self.acted.contains(&kids) {
            return true;
        }
        self.last_trade_time = time_sec;
        self.acted.insert(kids);
        false
    }
}

/// for DC
impl Brain3 {
    pub fn go_long(&mut self, symbol_id: i64, kline_id: u64, tick: &Tick, frame: &FrameMem) {
        let atr_pip = frame.atr_p * 3.;
        // let profit_pip = atr_pip * 0.6;
        let profit_pip = atr_pip * 1.;
        // let loose_pip = -atr_pip * 0.6;
        let loose_pip = -atr_pip * 1.;
        // let atr_pip = 10.;
        let np = NewPos {
            symbol_id,
            is_short: false,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, profit_pip), // 10 pip
            stop_loose_price: cal_price(tick.price_raw, loose_pip),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            // frame_ne: frame.clone(),
            frame: frame.clone(),
            // ta_med: ta_med.clone(),
            // ta_big: ta_big.clone(),
            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        // println!("Open long {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    pub fn go_short(&mut self, symbol_id: i64, kline_id: u64, tick: &Tick, frame: &FrameMem) {
        let atr_pip = frame.atr_p * 3.;
        // let profit_pip = atr_pip * 0.6;
        let profit_pip = -atr_pip * 1.;
        // let loose_pip = -atr_pip * 0.6;
        let loose_pip = atr_pip * 1.;
        // let atr_pip = 10.;
        let np = NewPos {
            symbol_id,
            is_short: true,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, profit_pip), // 10 pip
            stop_loose_price: cal_price(tick.price_raw, loose_pip),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            // frame_ne: frame.clone(),
            frame: frame.clone(),
            // ta_med: ta_med.clone(),
            // ta_big: ta_big.clone(),
            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        // println!("Open long {:#?}", np);
        self.con.open_position_req_new(&np);
    }
}

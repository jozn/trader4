use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::dc_intel::DCParent;
use crate::gate_api::{GateWay, NewPos, PosRes, UpdatePos};
use crate::offline::num5;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub type PairCandleCfg = (Pair, CandleConfig);

#[derive(Debug)]
pub struct Brain2 {
    pub con: Box<Arc<dyn GateWay>>,
    pub acted: HashSet<String>,
    pub open: HashMap<u64, PosRes>,
    // From PairMemo
    pub pair: Pair,
    pub last_tick: Option<Tick>,
    pub last_trade_time: u64,
    pub ticks_arr: TimeSerVec<Tick>,
    pub candles_dep: CandleSeriesTA, // dep
    pub dc_intl: DCParent,
}

impl Brain2 {
    pub fn new(backend: Arc<impl GateWay + 'static>, pair_conf: (Pair, CandleConfig)) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            last_trade_time: 0,
            ticks_arr: Default::default(),
            acted: Default::default(),
            open: Default::default(),
            pair: Pair::EURUSD,
            last_tick: None,
            candles_dep: CandleSeriesTA::new(&pair_conf.1),
            dc_intl: DCParent::new(),
        };

        brain
    }

    // Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        println!("on_connect Brain2");
    }

    pub fn on_notify_position(&mut self, pos: PosRes) {
        if pos.is_closed {
            self.open.remove(&pos.pos_id);
        } else {
            self.open.insert(pos.pos_id, pos);
        }
    }
}

impl Brain2 {
    pub fn go_long(
        &mut self,
        symbol_id: i64,
        kline_id: u64,
        tick: &Tick,
        ta_med: &TA1,
        ta_big: &TA1,
    ) {
        let atr_pip = ta_big.atr * 10_000.;
        // let atr_pip = ta_med.atr * 10_000.;
        let profit_pip = atr_pip * 0.6;
        // let profit_pip = atr_pip * 1.;
        let loose_pip = -atr_pip * 0.6;
        // let loose_pip = -atr_pip * 1.;
        // let atr_pip = 10.;
        let np = NewPos {
            symbol_id,
            is_short: false,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, atr_pip), // 10 pip
            stop_loose_price: cal_price(tick.price_raw, loose_pip),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            ta_med: ta_med.clone(),
            ta_big: ta_big.clone(),
            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        // println!("Open long {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    // ta_main: Medium
    pub fn go_short(
        &mut self,
        symbol_id: i64,
        kline_id: u64,
        tick: &Tick,
        ta_med: &TA1,
        ta_big: &TA1,
    ) {
        let atr_pip = ta_big.atr * 10_000. * 0.5;
        // let atr_pip = 10.;
        let np = NewPos {
            symbol_id,
            is_short: true,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, -atr_pip),
            stop_loose_price: cal_price(tick.price_raw, atr_pip * 1.0),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            ta_med: ta_med.clone(),
            ta_big: ta_big.clone(),

            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        // println!("Open short {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    fn already_acted(&mut self, symbol_id: i64, kline_id: u64) -> bool {
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

use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, Tick, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::{GateWay, NewPos};
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::sync::Arc;

pub type PairCandleCfg = (Pair, CandleConfig);

#[derive(Debug)]
pub struct Brain {
    pub con: Box<Arc<dyn GateWay>>,
    pub db: Vec<PairMemory>,
    pub last_trade_time: u64, // tem
    pub acted: HashSet<String>,
}

impl Brain {
    pub fn new(
        backend: Arc<impl GateWay + 'static>,
        pairs_conf: Vec<(Pair, CandleConfig)>,
    ) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            db: vec![],
            last_trade_time: 0,
            acted: Default::default(),
        };

        for pc in pairs_conf {
            brain.db.push(PairMemory::new(pc.0, &pc.1))
        }

        brain
    }

    // Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        let ids = assets::get_all_symbols_ids();
        println!("ids {:?}", ids);
        self.con.subscribe_pairs_req(assets::get_all_symbols());
    }

    pub fn borrow_pair_meta(&mut self, si: i64) -> &mut PairMemory {
        let mut idx = 0;
        let mut found = false;
        for pm in &self.db {
            if pm.pair.to_symbol_id() == si {
                found = true;
                break;
            }
            idx += 1;
        }
        if !found {
            self.db.push(PairMemory::new(
                Pair::id_to_symbol(si),
                &CandleConfig::default(),
            ));
        }
        let m = self.db.get_mut(idx).unwrap();
        m
    }
}

impl Brain {
    pub fn go_long(
        &mut self,
        symbol_id: i64,
        kline_id: u64,
        tick: &Tick,
        ta_med: &TA1,
        ta_big: &TA1,
    ) {
        let np = NewPos {
            symbol_id,
            is_short: false,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, 10.), // 10 pip
            stop_loose_price: cal_price(tick.price_raw, -10.),
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
        let np = NewPos {
            symbol_id,
            is_short: true,
            size_usd: 10000,
            take_profit_price: cal_price(tick.price_raw, -10.),
            stop_loose_price: cal_price(tick.price_raw, 10.),
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

fn rond5(num: f64) -> f64 {
    ((num * 100_000.0) as u64) as f64 / 100_000.0
}

fn cal_price(price: f64, pip: f64) -> f64 {
    let p = 1. + (pip / 10_000.);
    rond5(price * p)
}

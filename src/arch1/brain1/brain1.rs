use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, TA1, Tick};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::{GateWay, NewPos, PosRes, UpdatePos};
use crate::offline::num5;

use super::*;

pub type PairCandleCfg = (Pair, CandleConfig);

#[derive(Debug)]
pub struct Brain1 {
    pub con: Box<Arc<dyn GateWay>>,
    pub db: Vec<PairMemory>,
    pub last_trade_time: u64, // tem
    pub acted: HashSet<String>,
    pub open: HashMap<u64, PosRes>,
}

impl Brain1 {
    pub fn new(
        backend: Arc<impl GateWay + 'static>,
        pairs_conf: Vec<(Pair, CandleConfig)>,
    ) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            db: vec![],
            last_trade_time: 0,
            acted: Default::default(),
            open: Default::default(),
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

    pub fn on_notify_position(&mut self, pos: PosRes) {
        if pos.is_closed {
            self.open.remove(&pos.pos_id);
        } else {
            self.open.insert(pos.pos_id, pos);
        }
    }

    pub fn update_all_tailing_pos(&mut self) {
        for (_, p) in self.open.clone() {
            // self.on_full_tailing_pl(p);
            // todo enalbe
            self.on_tailing(p);
        }
    }

    pub fn on_tailing(&mut self, pos: PosRes) {
        // println!("Notify {:?}", pos);
        // if pos.is_closed || true {
        if pos.is_closed {
            return;
        }

        // let old_ops = self.open.get(&pos.pos_id);
        let pari_db = self.borrow_pair_meta(pos.symbol_id);
        let tick = pari_db.last_tick.clone().unwrap();
        let price = tick.price_raw;
        let ta = pari_db.candles.big.klines_ta.last().unwrap();
        let atr = ta.ta1.atr;

        if pos.is_short {
        } else {
            // if we alredy in proift dispostin go on tailing
            if pos.low_exit_price >= pos.open_price {
                self.on_full_tailing_pl(pos);
                return;
            }

            let half_distance = (pos.high_exit_price - pos.open_price) / 2.;
            let half = pos.open_price + (pos.high_exit_price - pos.open_price) / 2.;
            // if we have travedl half of profit set new stop lose to not this trade be a looser
            if price > half && pos.low_exit_price < pos.open_price {
                let mut final_low = pos.open_price + half_distance * 0.1; // 0.1 for equality and fees
                let mut final_hihg = pos.open_price + 4. * half_distance; // 2

                let up = UpdatePos {
                    pos_id: pos.pos_id,
                    close: false,
                    take_profit_price: rond5(final_hihg),
                    stop_loose_price: rond5(final_low),
                    at_price: tick.price_raw,
                    time_s: tick.time_s,
                    // ta_med: Default::default(),
                    // ta_big: Default::default()
                };

                self.con.update_position(&up);
            }
        }
    }

    pub fn on_full_tailing_pl(&mut self, pos: PosRes) {
        // println!("Notify {:?}", pos);
        // if pos.is_closed || true {
        if pos.is_closed {
            return;
        }

        // let old_ops = self.open.get(&pos.pos_id);
        let pari_db = self.borrow_pair_meta(pos.symbol_id);
        let tick = pari_db.last_tick.clone().unwrap();
        let ta = pari_db.candles.big.klines_ta.last().unwrap();
        let atr = ta.ta1.atr;

        if pos.is_short {
        } else {
            let new_stop = tick.price_raw - atr / 2.;
            let new_profit = tick.price_raw + atr / 2.;

            let mut changed = false;
            let mut final_low = pos.low_exit_price;
            let mut final_hihg = pos.high_exit_price;
            if pos.low_exit_price < new_stop {
                final_low = new_stop;
                changed = true;
            }

            if pos.high_exit_price < new_profit {
                let atr_profit = pos.open_price + atr * 1.5;
                if atr_profit > new_profit {
                    final_hihg = atr_profit;
                } else {
                    final_hihg = new_profit;
                }
                changed = true;
            }

            if changed {
                let up = UpdatePos {
                    pos_id: pos.pos_id,
                    close: false,
                    take_profit_price: rond5(final_hihg),
                    stop_loose_price: rond5(final_low),
                    at_price: tick.price_raw,
                    time_s: tick.time_s,
                    // ta_med: Default::default(),
                    // ta_big: Default::default()
                };

                self.con.update_position(&up);
            }
        }
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

impl Brain1 {
    pub fn go_long(
        &mut self,
        symbol_id: i64,
        kline_id: u64,
        tick: &Tick,
        ta_med: &TA1,
        ta_big: &TA1,
    ) {
        let atr_pip = ta_big.atr * 10_000.;
        let profit_pip = atr_pip * 0.6;
        // let profit_pip = atr_pip * 1.;
        let loose_pip = -atr_pip * 0.4;
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

fn rond5(num: f64) -> f64 {
    ((num * 100_000.0) as u64) as f64 / 100_000.0
}

fn cal_price(price: f64, pip: f64) -> f64 {
    let p = 1. + (pip / 10_000.);
    rond5(price * p)
}

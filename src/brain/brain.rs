use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::Tick;
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::{GateWay, NewPos};
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct Brain {
    pub con: Box<Arc<dyn GateWay>>,
    pub db: Vec<PairMeta>,
    pub acted: HashSet<String>,
}

impl Brain {
    // Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        let ids = assets::get_all_symbols_ids();
        println!("ids {:?}", ids);
        self.con.subscribe_pairs_req(assets::get_all_symbols());
    }

    pub fn borrow_pair_meta(&mut self, si: i64) -> &mut PairMeta {
        // let mut pm = self.db.iter().find_position(|d| d.pair.to_symbol_id() == si ).unwrap();
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
            self.db.push(PairMeta::new(Pair::id_to_symbol(si)));
        }
        let m = self.db.get_mut(idx).unwrap();
        m
    }
}

impl Brain {
    pub fn go_long(&mut self, symbol_id: i64, kline_id: u64, tick: &Tick) {
        let np = NewPos {
            symbol_id,
            is_short: false,
            size_usd: 10000,
            take_profit_price: rond5(tick.price_raw * 1.001), // 10 pip
            stop_loose_price: rond5(tick.price_raw * 0.999),
            at_price: tick.price_raw,
            time_s: tick.time_s,
            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        println!("Open long {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    pub fn go_short(&mut self, symbol_id: i64, kline_id: u64, tick: &Tick) {
        let np = NewPos {
            symbol_id,
            is_short: true,
            size_usd: 10000,
            take_profit_price: rond5(tick.price_raw * 0.999), // 10 pip
            stop_loose_price: rond5(tick.price_raw * 1.001),
            at_price: tick.price_raw,
            time_s: tick.time_s,

            ..Default::default()
        };

        if self.already_acted(symbol_id, kline_id) {
            return;
        }

        println!("Open short {:#?}", np);
        self.con.open_position_req_new(&np);
    }

    fn already_acted(&mut self, symbol_id: i64, kline_id: u64) -> bool {
        let kids = format!("{}_{}", symbol_id, kline_id);
        if self.acted.contains(&kids) {
            return true;
        }
        self.acted.insert(kids);
        false
    }
}

fn rond5(num: f64) -> f64 {
    ((num * 100_000.0) as u64) as f64 / 100_000.0
}

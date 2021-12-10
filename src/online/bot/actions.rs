use super::*;
use crate::candle::*;
pub use crate::gate_api::NewPos;
use std::sync::Arc;

#[derive(Debug)]
pub struct Actor {
    pub con: Arc<CTrader>,
}

impl Actor {
    pub fn open_postion(&mut self, symbol_id: i64, is_short: bool, tick: &Tick) {
        println!("Open long postion");

        let np = NewPos {
            symbol_id,
            is_short,
            size_usd: 1000,
            take_profit_price: tick.price_raw * 1.001, // 10 pip
            stop_loose_price: tick.price_raw * 0.999,
        };
        self.con.open_postion_req_new(&np);
    }

    pub fn go_long(&mut self, symbol_id: i64, tick: &Tick) {
        let np = NewPos {
            symbol_id,
            is_short: false,
            size_usd: 10000,
            take_profit_price: rond5(tick.price_raw * 1.001), // 10 pip
            stop_loose_price: rond5(tick.price_raw * 0.999),
        };
        println!("Open long {:?}", np);
        self.con.open_postion_req_new(&np);
    }

    pub fn go_short(&mut self, symbol_id: i64, tick: &Tick) {
        let np = NewPos {
            symbol_id,
            is_short: true,
            size_usd: 10000,
            take_profit_price: rond5(tick.price_raw * 0.999), // 10 pip
            stop_loose_price: rond5(tick.price_raw * 1.001),
        };
        println!("Open short {:?}", np);
        self.con.open_postion_req_new(&np);
    }
}

/*#[derive(Debug)]
pub struct NewPos {
    pub symbol_id: i64,
    pub is_short: bool,
    pub size_usd: i64,
    pub take_profit_price: f64,
    pub stop_loose_price: f64,
}*/
fn rond5(num: f64) -> f64 {
    ((num * 100_000.0) as u64) as f64 / 100_000.0
}

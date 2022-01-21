// use super::loader::*;

use crate::candle::Tick;
use crate::configs::assets::Pair;

#[derive(Debug)]
pub struct TransTickData {
    pub timestamp: i64,
    pub bid_price: i64,
    pub ask_price: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct BTickData {
    pub date_str: String,
    pub timestamp_sec: i64,
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}

impl BTickData {
    pub fn get_price(&self) -> f64 {
        self.bid_price
    }

    pub fn get_spread_pip(&self, pair: &Pair) -> f64 {
        (self.ask_price - self.bid_price) * pair.get_pip_multi()
    }

    pub fn to_tick(&self) -> Tick {
        let multi = 100_000.;
        Tick {
            time_s: self.timestamp_sec as u64,
            // price_raw: self.bid_price * multi,
            price_raw: self.bid_price,
            multi: 1.,
            qty: 0.0,
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price,
        }
    }

    pub fn to_fast_bin(&self) -> TickBinFast {
        TickBinFast {
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price
        }
    }
}

#[derive( serde::Serialize,  serde::Deserialize, PartialEq, Debug)]
pub struct TickBinFast {
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}

impl TickBinFast {
    pub fn to_tick(&self) -> BTickData {
        BTickData {
            date_str: "".to_string(),
            timestamp_sec: self.timestamp / 1000,
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price,
        }
    }
}
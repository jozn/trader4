// use super::loader::*;

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
}

#[derive( serde::Serialize,  serde::Deserialize, PartialEq, Debug)]
pub struct TickBinFast {
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}

impl BTickData {
    pub fn to_fast_bin(&self) -> TickBinFast {
        TickBinFast {
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price
        }
    }
}
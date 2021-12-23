use crate::candle::TA1;
use crate::configs::assets::Pair;
use std::fmt::Debug;

pub trait GateWay: Debug {
    // Calls from brain1
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>);
    fn open_position_req_new(&self, new_pos: &NewPos);
    fn update_position(&self, update: &UpdatePos);
    // Others
    fn get_time_ms(&self) -> u64;
}

#[derive(Debug, Clone, Default)]
pub struct NewPos {
    pub symbol_id: i64,
    pub is_short: bool,
    pub size_usd: i64, // Could be other currency -- as the cTrader works this way we do not use XLot
    pub take_profit_price: f64,
    pub stop_loose_price: f64,

    // Informative
    pub at_price: f64,
    pub time_s: u64, // Brain time
    pub ta_med: TA1,
    pub ta_big: TA1,
    // add comment, label too
}

#[derive(Debug, Clone, Default)]
pub struct UpdatePos {
    pub pos_id: u64,
    pub close: bool,
    pub take_profit_price: f64,
    pub stop_loose_price: f64,

    // Informative
    pub at_price: f64,
    pub time_s: u64, // Brain time
                     // pub ta_med: TA1,
                     // pub ta_big: TA1,
}

// Send from backend, cTrader to Brain
#[derive(Debug, Clone, Default)]
pub struct PosRes {
    pub pos_id: u64,
    pub symbol_id: i64,
    pub is_closed: bool,
    pub is_short: bool,
    pub pos_size_usd: f64,
    pub open_time: u64,
    pub open_price: f64,
    pub high_exit_price: f64,
    pub low_exit_price: f64,
}

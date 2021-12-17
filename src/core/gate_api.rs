use crate::candle::TA1;
use crate::configs::assets::Pair;
use std::fmt::Debug;

pub trait GateWay: Debug {
    // Calls from brain
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>);
    fn open_position_req_new(&self, new_pos: &NewPos);
    fn update_position(&self);
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

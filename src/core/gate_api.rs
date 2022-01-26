use crate::candle::TA1;
use crate::configs::assets::Pair;
// use crate::dc_intel::FrameMem;
use crate::ne3::NEFrame;
use std::fmt::Debug;

// todo: clean old and remvoe {}
pub trait GateWay: Debug {
    // Calls from brain1
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>);
    fn open_position_req_new_dep(&self, new_pos: &NewPosDep) {}
    fn open_position_req_new(&self, new_pos: &NewPos) {}
    fn update_position(&self, update: &UpdatePos);
    // Others
    fn get_time_ms(&self) -> u64;
}

#[derive(Debug, Clone, Default)]
pub struct NewPos {
    pub pair: Pair,
    pub is_short: bool,
    pub base_asset_size: f64,
    pub exit_high_price: f64,
    pub exit_low_price: f64,
    // Informative
    pub at_price: f64,
    pub time_sec: u64, // Brain time
    pub frame: crate::ne4::NEFrame,
}

// Send from backend, cTrader to Brain
#[derive(Debug, Clone, Default)]
pub struct EventPosition {
    pub pos_id: u64,
    pub pair: Pair,
    pub is_closed: bool,
    pub is_short: bool,
    pub base_asset_size: f64,
    pub quote_asset_size: f64,
    pub exit_high_price: f64,
    pub exit_low_price: f64,
    pub open_time: u64,
    pub open_price: f64,
}

#[derive(Debug, Clone, Default)]
pub struct UpdatePos {
    pub pos_id: u64,
    pub close: bool,
    pub exit_high_price: f64,
    pub exit_low_price: f64,
    // Informative
    pub at_price: f64,
    pub time_s: u64, // Brain time
}

///// Deprecated > replace with new one for new engine /////

#[derive(Debug, Clone, Default)]
pub struct NewPosDep {
    pub pair: Pair,
    pub symbol_id_dep: i64, // todo remove
    pub is_short: bool,
    // Base currency - USD, Euro, typical higher valued (BTC vs USD) ( Euro vs USD)
    //  -- as the cTrader works this way we do not use XLot
    pub size_base: i64,
    pub take_profit_price: f64,
    pub stop_loose_price: f64,

    // Informative
    pub at_price: f64,
    pub time_s: u64, // Brain time
    // pub ta_med: TA1,
    // pub ta_big: TA1,
    // pub frame: FrameMem,
    pub frame_ne2: NEFrame,
    // add comment, label too
}

// Send from backend, cTrader to Brain
#[derive(Debug, Clone, Default)]
pub struct PosResDep {
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

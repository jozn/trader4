use crate::configs::assets::Pair;
use crate::offline::Position;
use std::fmt::Debug;

pub trait GateWay: Debug {
    // Calls from brain1
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>);
    fn open_position_req_new(&self, new_pos: &NewPosReq) {}
    fn update_position(&self, update: &UpdatePosReq);
    // Others
    fn get_time_ms(&self) -> u64;
}

#[derive(Debug, Clone, Default)]
pub struct NewPosReq {
    pub pair: Pair,
    pub is_short: bool,
    pub base_asset_size: f64,
    pub exit_high_price: f64,
    pub exit_low_price: f64,
    // Virual: only used for Brain virtual sims not offline sims
    pub virtual_id: u64,
    pub is_virtual: bool, // set when not actual money is being set:
    pub signal_key: String,
    // Informative
    pub at_price: f64,
    pub time_sec: u64, // Brain time
    // pub frame: crate::sky_eng::SFrame,
    pub frame: crate::sig_engs::ml_eng::MLFrameInsight,
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
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdatePosReq {
    pub pos_id: u64,
    pub close: bool,
    pub exit_high_price: f64,
    pub exit_low_price: f64,
    // Informative
    pub at_price: f64,
    pub time_s: u64, // Brain time
}

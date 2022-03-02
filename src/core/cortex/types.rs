use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ActionSignal {
    pub small_kid: i32,
    pub consumed: bool,
    pub long: bool,
    pub profit: f64,
    pub loss: f64,
    pub time_sec: i64,
}

// Internal to signals engines
// ps_ : primary_signal
// fs_ : final_signal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalMem {
    pub ps_buy: bool,
    pub ps_medium_bar_id: i32,
    pub ps_time_sec: i64,
    pub fs_buy: bool,
    pub fs_small_bar_id: i32,
    pub fs_time_sec: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub marker_id: i64,
    pub parent_id: i64, // if this marker has a parent marker(ex: early signals)
    pub time_sec: i64,
    pub m_type: MarkerType,
}

// For presentaion to json/graphs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkerType {
    LongEarly,
    LongFinal,
}

// todo merge SignalMem + rename
#[derive(Debug, Clone)]
pub struct PairSignalsMemory {
    pub key: String,
    pub primary_signal: bool,
    pub ps_small_bar_id: i32,
    pub final_buy: bool,
    pub fb_small_bar_id: i32,
}

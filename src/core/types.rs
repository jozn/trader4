use serde::{Deserialize, Serialize};
// todo move all to cortex

pub struct WeekData {
    pub week_id: u16,
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ActionSignalDep {
    pub small_kid: i32,
    pub long: bool,
    pub profit: f64,
    pub loss: f64,
}

// Internal to signals engines
// ps_ : primary_signal
// fs_ : final_signal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalMemDep {
    pub ps_buy: bool,
    pub ps_small_bar_id: i32,
    pub fs_buy: bool,
    pub fs_small_bar_id: i32,
}

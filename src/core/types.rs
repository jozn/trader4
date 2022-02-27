use serde::{Deserialize, Serialize};

pub struct WeekData {
    pub week_id: u16,
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ActionSignal {
    pub small_kid: i32,
    pub long: bool,
    pub profit: f64,
    pub loss: f64,
}

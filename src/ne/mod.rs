// Next Engine
pub mod frame;
pub mod parent;
pub mod strength;
pub mod rel;

pub use frame::*;
pub use parent::*;
pub use strength::*;
pub use rel::*;

pub use serde::{Deserialize, Serialize};
use crate::candle::Kline;
use crate::helper;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NECandle {
    #[serde(skip)]
    pub open_time: u64, // in mill seconds
    pub open_time_str: String,
    #[serde(skip)]
    pub close_time: u64,
    pub tick_count: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pip_hl: f64,
    pub pip_co: f64,
}

impl NECandle {
    pub fn new(k: &Kline) -> Self {
        let nc = NECandle {
            open_time: k.open_time,
            open_time_str: helper::to_time_string(k.open_time as i64),
            close_time: k.close_time,
            tick_count: k.tick_count,
            open: k.open,
            high: k.high,
            low: k.low,
            close: k.close,
            pip_hl: (k.high - k.low) * 10_000.,
            pip_co: (k.close - k.open).abs() * 10_000.
        };

        nc
    }
}
pub mod bar;
pub mod frame;
pub mod score;
pub mod sky_eng;

pub use bar::*;
pub use frame::*;
pub use score::*;
pub use sky_eng::*;
// Sky Engine

use crate::candle::Kline;
use crate::helper;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SCandle {
    #[serde(skip)]
    pub open_time: u64, // in mill seconds
    pub open_time_str: String,
    #[serde(skip)]
    pub close_time: u64,
    #[serde(rename = "ticks")]
    pub tick_count: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pip_hl: f64,
    pub pip_co: f64,
}

impl SCandle {
    pub fn new(k: &Kline) -> Self {
        let nc = SCandle {
            open_time: k.open_time,
            open_time_str: helper::to_time_string(k.open_time as i64),
            close_time: k.close_time,
            tick_count: k.tick_count,
            open: k.open,
            high: k.high,
            low: k.low,
            close: k.close,
            pip_hl: (k.high - k.low) * 10_000.,
            pip_co: (k.close - k.open).abs() * 10_000.,
        };

        nc
    }
}

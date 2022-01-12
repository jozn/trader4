// Next Engine
pub mod crossing;
pub mod dc;
pub mod frame;
pub mod parent;
pub mod rel;
pub mod strength;

pub use crossing::*;
pub use dc::*;
pub use frame::*;
pub use parent::*;
pub use rel::*;
pub use strength::*;

/// This engine is mainly a successful engine:
///  (all below result are on longs trades only)
/// - we was postive +0.07 (transactions) on Euro/USD pair in long direction.
/// - USD/GBP was +0.11 of transactions.
/// - USD/CHF was particaly good +0.34 of all transaction in long with a consistance proftis almost in all weeks.
/// - USD/CAD was +0.11 with a good looking balance graph.
/// - USD/AUD was +0.22 a good balance graph - 61% win rate
/// - USD/NZD was +0.22 a good balance graph - 62% win rate
///
///  In the next engine we should consider more or long term trends.
///
///  This moudle uses a self contained Candle builder.
use crate::candle::Kline;
use crate::helper;
pub use serde::{Deserialize, Serialize};

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
            pip_co: (k.close - k.open).abs() * 10_000.,
        };

        nc
    }
}

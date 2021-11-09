use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub use candle_series::*;
pub use candle_series_ta::*;
pub use kline::*;
pub use kline_ta::*;
pub use util::*;

pub mod candle_series;
pub mod candle_series_ta;
pub mod kline;
pub mod kline_ta;
pub mod util;

pub type TResult<T> = std::result::Result<T, TErr>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TErr {
    KlineDurationNotSmallErr,
    EmptyTradesErr,
    TickTimeErr,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Tick {
    pub time_s: u64,
    pub price: f64, // multi
    pub price_raw: f64,
    pub price_multi: f64,
    pub qty: f64, // todo
}

impl TimeKey for Tick {
    fn get_time(&self) -> u64 {
        self.time_s
    }
}

const SMALL_TICK: u64 = 15;
const MEDIUM_TICK: u64 = 10; // 50
const BIG_TICK: u64 = 30; // 150

// todo - migrate
//  ticker + some namnign convention to not get confused

// Notes:
//  + SerVecUnique is not used
//  + SerVec used for holding ticks

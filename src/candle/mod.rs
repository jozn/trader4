use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub use kline::*;
pub use kline_ta::*;
pub use position::*;
pub use util::*;
pub use volume_candle_series_raw::*;

pub mod kline;
pub mod kline_ta;
pub mod position;
pub mod util;
pub mod volume_candle_series_raw;
pub mod volume_candle_series_ta;

pub type TResult<T> = std::result::Result<T, TErr>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TErr {
    KlineDurationNotSmallErr,
    EmptyTradesErr,
    TradesTimeErr,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tick {
    pub time: u64,
    pub price: f64,
    pub qty: f64, // todo
} // todo change to Tick

impl TimeKey for Tick {
    fn get_time(&self) -> u64 {
        self.time
    }
}

const SMALL_TIME: u64 = 60_000;
const MEDIUM_TIME: u64 = 7 * SMALL_TIME;
const BIG_TIME: u64 = 13 * SMALL_TIME;

const SMALL_VOLUME: f64 = 10.;
const MEDIUM_VOLUME: u64 = 5;
const BIG_VOLUME: u64 = 15;

// todo - migrate
//  ticker

// Notes:
//  + SerVecUnique is not used
//  + SerVec used for holding ticks

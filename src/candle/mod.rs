pub mod candle_series_raw;
pub mod candle_series_ta;
pub mod candles;
pub mod cross;
pub mod csv_out;
pub mod data_utils;
pub mod events;
pub mod kline;
pub mod kline_ta;
pub mod loader_kline;
pub mod loader_trade;
pub mod ml;
pub mod ohlcv;
pub mod play_dep;
pub mod position;
pub mod proc;
pub mod ser_vec;
pub mod ticker;
pub mod vol_ser_vec;
pub mod vol_vec;
pub mod volume_candle_series_raw;
// pub mod volume_candle_series_raw_dep;
pub mod volume_candle_series_ta;

pub use candle_series_raw::*;
pub use candle_series_ta::*;
pub use cross::*;
pub use csv_out::*;
pub use data_utils::*;
pub use events::*;
pub use kline::*;
pub use kline_ta::*;
pub use loader_kline::*;
pub use loader_trade::*;
pub use ohlcv::*;
pub use position::*;
pub use ser_vec::*;
pub use ticker::*;
pub use vol_ser_vec::*;
pub use vol_vec::*;
pub use volume_candle_series_raw::*;

pub type TResult<T> = std::result::Result<T, TErr>;

use serde::{Deserialize, Serialize};
use std::rc::Rc;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TErr {
    KlineDurationNotSmallErr,
    EmptyTradesErr,
    TradesTimeErr,
}

const SMALL_TIME: u64 = 60_000;
const MEDIUM_TIME: u64 = 7 * SMALL_TIME;
const BIG_TIME: u64 = 13 * SMALL_TIME;

const SMALL_VOLUME: f64 = 10.;
const MEDIUM_VOLUME: u64 = 5;
const BIG_VOLUME: u64 = 15;

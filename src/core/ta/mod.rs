use serde::{Deserialize, Serialize};

pub use average_absolute_deviation::*;
pub use average_true_range::*;
pub use bollinger_band::*;
pub use cci::*;
pub use dc_snake::*;
pub use dcs::*;
pub use dcs2::*;
pub use dmi::*;
pub use dmmd::*;
pub use donchain_channel::*;
pub use ema::*;
pub use fisher::*;
pub use gorilla_band::*;
pub use hull::*;
pub use line_dir::*;
pub use ma_mom::*;
pub use ma_mom_dep::*;
pub use ma_trend::*;
pub use macd::*;
pub use macd_dep::*;
pub use maximum::*;
pub use minimum::*;
pub use momentum::*;
pub use nwave::*;
pub use rel_dc::*;
pub use rel_price::*;
pub use rel_price_dep::*;
pub use roc::*;
pub use rpc::*;
pub use rpi::*;
pub use rsi::*;
pub use rti::*;
pub use sma::*;
pub use snake_band::*;
pub use standard_deviation::*;
pub use stoch::*;
pub use stoch_rsi::*;
pub use trend_direction::*;
pub use true_range::*;
pub use vel::*;
pub use vel2::*;
pub use vel_mom::*;
pub use wave::*;
pub use window::*;
pub use wma::*;
// pub use vel_trend::*;

pub mod average_absolute_deviation;
pub mod average_true_range;
pub mod bollinger_band;
pub mod cci;
pub mod dc_snake;
pub mod dcs;
pub mod dcs2;
pub mod dmi;
pub mod dmmd;
pub mod donchain_channel;
pub mod ema;
pub mod fisher;
pub mod gorilla_band;
pub mod hull;
pub mod line_dir;
pub mod ma_mom;
pub mod ma_mom_dep;
pub mod ma_trend;
pub mod macd;
pub mod macd_dep;
pub mod maximum;
pub mod minimum;
pub mod momentum;
pub mod nwave;
pub mod rel_dc;
pub mod rel_price;
pub mod rel_price_dep;
pub mod roc;
pub mod rpc;
pub mod rpi;
pub mod rsi;
pub mod rti;
pub mod sma;
pub mod snake_band;
pub mod standard_deviation;
pub mod stoch;
pub mod stoch_rsi;
pub mod trend_direction;
pub mod true_range;
pub mod vel;
pub mod vel2;
pub mod vel_mom;
pub mod wave;
pub mod window;
pub mod wma;
pub mod zigzag;
// pub mod vel_trend;

////////// Some other shared types //////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TAErr {
    WrongArgs,
}

pub type TAResult<T> = std::result::Result<T, TAErr>;

pub fn round(num: f64) -> f64 {
    (num * 1000.0).round() / 1000.00
}

// Notes:
// Spec: No more next_peek() methods > cumbersome to develop, inconsistent results, not worth it >
//   > just clone when needed.

// + There is not much value in tripple moving average > it's like EMA with more inaccuate data.
// + Do not use hull or Tripple Moving average > EMA/WMA with lower period is enough.
// + Hull and Triple EMA gives false number of price > number it has not touched yet.

// todo: remove peek functions

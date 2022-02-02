use serde::{Deserialize, Serialize};

pub use average_absolute_deviation::*;
pub use average_true_range::*;
pub use cci::*;
pub use dcs::*;
pub use dcs2::*;
pub use dmi::*;
pub use donchain_channel::*;
pub use ema::*;
pub use fisher::*;
pub use hull::*;
pub use ma_trend::*;
pub use macd::*;
pub use macd_dep::*;
pub use maximum::*;
pub use minimum::*;
pub use momentum::*;
pub use roc::*;
pub use rsi::*;
pub use rti::*;
pub use sma::*;
pub use stoch::*;
pub use stoch_rsi::*;
pub use true_range::*;
pub use vel::*;
pub use vel2::*;
pub use window::*;
pub use wma::*;
pub use rpc::*;

pub mod average_absolute_deviation;
pub mod average_true_range;
pub mod cci;
pub mod dcs;
pub mod dcs2;
pub mod dmi;
pub mod donchain_channel;
pub mod ema;
pub mod fisher;
pub mod hull;
pub mod ma_trend;
pub mod macd;
pub mod macd_dep;
pub mod maximum;
pub mod minimum;
pub mod momentum;
pub mod roc;
pub mod rsi;
pub mod rti;
pub mod sma;
pub mod stoch;
pub mod stoch_rsi;
pub mod true_range;
pub mod vel;
pub mod vel2;
pub mod window;
pub mod wma;
pub mod rpc;

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

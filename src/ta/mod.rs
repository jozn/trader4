use serde::{Deserialize, Serialize};

pub use average_absolute_deviation::*;
pub use average_true_range::*;
pub use cci::*;
pub use ema::*;
pub use fisher::*;
pub use hull::*;
pub use macd::*;
pub use maximum::*;
pub use minimum::*;
pub use momentum::*;
pub use roc::*;
pub use rsi::*;
pub use sma::*;
pub use true_range::*;
pub use window::*;
pub use wma::*;

pub mod average_absolute_deviation;
pub mod average_true_range;
pub mod cci;
pub mod ema;
pub mod fisher;
pub mod hull;
pub mod macd;
pub mod maximum;
pub mod minimum;
pub mod momentum;
pub mod roc;
pub mod rsi;
pub mod sma;
pub mod true_range;
pub mod window;
pub mod wma;

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

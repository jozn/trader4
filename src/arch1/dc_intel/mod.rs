pub mod dc_candle;
pub mod frame;
pub mod parent;
pub mod strength;

pub use dc_candle::*;
pub use frame::*;
pub use parent::*;
pub use strength::*;

use serde::{Deserialize, Serialize};

pub type TResult<T> = std::result::Result<T, TErr>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TErr {
    EmptyTradesErr,
}

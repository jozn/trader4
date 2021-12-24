pub mod dc_candle;
pub mod frame;
pub mod parent;

pub use dc_candle::*;
pub use frame::*;
pub use parent::*;

use serde::{Deserialize, Serialize};

pub type TResult<T> = std::result::Result<T, TErr>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TErr {
    EmptyTradesErr,
}

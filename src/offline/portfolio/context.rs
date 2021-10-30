use super::*;
use serde::{Deserialize, Serialize};
use crate::candle::TA1;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub xxxxx: f64,
    #[serde(skip)]
    pub ta: TA1,

    pub s_ema: f64,
    pub s_mom: f64,
    pub s_roc: f64,
    pub s_rsi: f64,
    pub s_cci: f64,
    pub s_macd: f64,
    pub s_fisher: f64,
}

use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;

// Relative Donchain Channel
//  The formula source is taken from DCS2

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RDCRes {
    pub perc_med: f64,
    pub perc_big: f64,
    pub height_med: f64,
    pub height_big: f64,
    pub dcres_med: DCRes,
    pub dcres_big: DCRes,
}
pub type RelDc = RDC; // Relative DC

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RDC {
    dc_medium: DC,
    dc_big: DC,
    ma_height_med: SMA,
    ma_height_big: SMA,
    past: VecDeque<RDCRes>,
}

impl RDC {
    pub fn new(period_med: usize, period_big: usize) -> TAResult<Self> {
        if period_med == 0 || period_big == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                dc_medium: DC::new(period_med).unwrap(),
                dc_big: DC::new(period_big).unwrap(),
                ma_height_med: SMA::new(period_med * 2).unwrap(),
                ma_height_big: SMA::new(period_big * 2).unwrap(),
                past: VecDeque::new(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> RDCRes {
        let dc_med = self.dc_medium.next(&candle);
        let dc_big = self.dc_big.next(&candle);

        let price = candle.close();

        let perc_med = (price - dc_med.low) / (dc_med.high - dc_med.low);
        let perc_big = (price - dc_big.low) / (dc_big.high - dc_big.low);

        let perc_med = (price - dc_med.low) / self.ma_height_med.next(dc_med.high - dc_med.low);
        let perc_big = (price - dc_big.low) / self.ma_height_big.next(dc_big.high - dc_big.low);

        let height_med = (dc_med.high - dc_med.low) * 1.; // 10_000.;
        let height_big = (dc_big.high - dc_big.low) * 1.; // 10_000.;

        // todo migrate weight,ratio,.. from DCS2

        let out = RDCRes {
            perc_med,
            perc_big,
            height_med,
            height_big,
            dcres_med: dc_med.clone(),
            dcres_big: dc_big.clone(),
        };
        out
    }
}

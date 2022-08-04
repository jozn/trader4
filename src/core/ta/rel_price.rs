use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;
// This is a simplified of RelPriceDep with removing of medium and big time frame, only one
//  timeframe is used and the reset

// RelativePrice
//  The formula is a rewrite of RelPriceDep which is from rel_dc which itself is taken from DCS2

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RelPriceRes {
    pub dc_high: f64,
    pub dc_middle: f64,
    pub dc_low: f64,
    pub oversold: f64,        // os
    pub os_index: f64,        // not overly chart but sub inditcator
    pub os_stoch_main: f64,   // os: oversold
    pub os_stoch_smooth: f64, // os: oversold
    pub height: f64,
    pub height_ma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelPrice {
    dc: DC,
    height_ma: SMA,
    stoch_med: Stoch,
}

impl RelPrice {
    pub fn new(period: usize) -> TAResult<Self> {
        if period == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                dc: DC::new(period).unwrap(),
                height_ma: SMA::new(period * 2).unwrap(),
                stoch_med: Stoch::new(period, 3, 5).unwrap(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> RelPriceRes {
        let dc = self.dc.next(&candle);
        let price = candle.close();

        let height = dc.high - dc.low;
        let height_ma = self.height_ma.next(height);

        let oversold = dc.high - height_ma;
        let os_index = (price - dc.low) / height_ma;

        let stoch_res = self.stoch_med._next_raw(os_index, os_index, os_index);

        RelPriceRes {
            dc_high: dc.high,
            dc_middle: dc.middle,
            dc_low: dc.low,
            oversold,
            os_index,
            os_stoch_main: stoch_res.main_k,
            os_stoch_smooth: stoch_res.smooth_d,
            height,
            height_ma,
        }
    }
}

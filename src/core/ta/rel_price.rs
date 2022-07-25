use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;

// RelativePrice
//  The formula is a rewrite of rel_dc which itself is taken from DCS2

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RelPriceRes {
    pub oversold_med: f64,
    pub oversold_big: f64,
    pub osi_med: f64, // osi: oversold index: not overly chart but sub inditcator
    pub osi_big: f64,
    pub os_stoch_main: f64,   // os: oversold
    pub os_stoch_smooth: f64, // os: oversold
    pub perc_med: f64,        // old
    pub perc_big: f64,        // old
    pub height_med: f64,
    pub height_big: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelPrice {
    dc_medium: DC,
    dc_big: DC,
    ma_height_med: SMA,
    ma_height_big: SMA,
    stoch_med: Stoch,
    past: VecDeque<RelPriceRes>,
}

impl RelPrice {
    pub fn new(period_med: usize, period_big: usize) -> TAResult<Self> {
        if period_med == 0 || period_big == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                dc_medium: DC::new(period_med).unwrap(),
                dc_big: DC::new(period_big).unwrap(),
                ma_height_med: SMA::new(period_med * 2).unwrap(),
                ma_height_big: SMA::new(period_big * 2).unwrap(),
                stoch_med: Stoch::new(period_big, 3, 5).unwrap(),
                past: VecDeque::new(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> RelPriceRes {
        let dc_med = self.dc_medium.next(&candle);
        let dc_big = self.dc_big.next(&candle);

        let price = candle.close();
        let price_high = candle.high(); // todo no need for high.low > use clsoe

        let perc_med = (price - dc_med.low) / (dc_med.high - dc_med.low);
        let perc_big = (price - dc_big.low) / (dc_big.high - dc_big.low);

        let ma_height_med = self.ma_height_med.next(dc_med.high - dc_med.low);
        let ma_height_big = self.ma_height_big.next(dc_big.high - dc_big.low);

        let oversold_med = dc_med.high - ma_height_med;
        let oversold_big = dc_big.high - ma_height_big;

        let osi_med = (price_high - dc_med.low) / ma_height_med;
        let osi_big = (price_high - dc_big.low) / ma_height_big;

        let stoch_res = self.stoch_med._next_raw(osi_med, osi_med, osi_med);

        let perc_med = (price - dc_med.low) / ma_height_med;
        let perc_big = (price - dc_big.low) / ma_height_big;

        let height_med = ma_height_med;
        let height_big = ma_height_big;
        let height_med = (dc_med.high - dc_med.low) * 1.; // 10_000.;
        let height_big = (dc_big.high - dc_big.low) * 1.; // 10_000.;

        let out = RelPriceRes {
            oversold_med,
            oversold_big,
            osi_med,
            osi_big,
            os_stoch_main: stoch_res.main_k,
            os_stoch_smooth: stoch_res.smooth_d,
            perc_med,
            perc_big,
            height_med,
            height_big,
        };
        out
    }
}

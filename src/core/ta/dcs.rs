use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;

// Donchain Channel Signal

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DCSRes {
    pub x_pip: f64,
    pub x_high: f64,
    pub x_low: f64,
    pub low_sig: bool, // go for bull
    pub up_sig: bool,  // go for short
    pub x_perc: f64,
    pub hh: bool,
    pub ll: bool,
    pub b_high: f64,
    pub b_low: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCS {
    max: Maximum,
    max_big: Maximum,
    min: Minimum,
    min_big: Minimum,
    cross_high: SimpleCross,
    cross_low: SimpleCross,
    past: VecDeque<DCSRes>,
}

// Note: the logic of functionality is taken from tradingview's "DCSastic" indicator.
//  use smooth_k 1 for no effect of EMA
impl DCS {
    pub fn new(period_k: usize) -> TAResult<Self> {
        match period_k {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                max: Maximum::new(period_k)?,
                max_big: Maximum::new(period_k * 3)?,
                min: Minimum::new(period_k)?,
                min_big: Minimum::new(period_k * 3)?,
                cross_high: SimpleCross::new(),
                cross_low: SimpleCross::new(),
                past: VecDeque::new(),
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> DCSRes {
        let high = self.max.next(candle.high());
        let low = self.min.next(candle.low());

        let big_high = self.max_big.next(candle.high());
        let big_low = self.min_big.next(candle.low());

        let price = candle.close();

        let perc = (price - low) / (high - low);
        let cut_off = (high - low) / 5.; // 0.2
        let high_cross_line = high - cut_off;
        let low_cross_line = low + cut_off;

        let high_cross_res = self.cross_high.next_v2(price, high_cross_line);
        let low_cross_res = self.cross_low.next_v2(price, low_cross_line);

        let mut now = DCSRes {
            x_pip: (high - low) * 10_000.,
            x_high: high,
            x_low: low,
            up_sig: high_cross_res.crossed_under,
            low_sig: low_cross_res.crossed_above,
            x_perc: perc,
            b_high: big_high,
            b_low: big_low,
            ..Default::default()
        };

        let pre_opt = self.past.front();
        match pre_opt {
            None => {}
            Some(pre) => {
                if now.x_high > pre.x_high {
                    now.hh = true;
                }
                if now.x_low < pre.x_low {
                    now.ll = true;
                }
            }
        }

        if self.past.len() == 200 {
            self.past.pop_back();
        }
        self.past.push_front(now.clone());

        now
    }
}

impl Default for DCS {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(DCS::new(0).is_err());
        assert!(DCS::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _s = DCS::new(3).unwrap();
    }

    #[test]
    fn test_default() {
        DCS::default();
    }
}

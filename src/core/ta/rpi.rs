use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};

use super::*;

// Designed by me:
// Relative Price Index - like RPC with transform

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RPIRes {
    pub high: f64,
    pub low: f64,
    pub p_high: f64,
    pub high_sig: f64,
    pub p_low: f64,
    pub low_sig: f64,
    pub p_close: f64,
    pub buy_low: bool,
    pub buy_high: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPI {
    min_ma: EMA,
    min_sig: EMA,
    max_ma: EMA,
    max_sig: EMA,
    atr: ATR,
    atr_per: f64,
    cross_high: SimpleCross,
    cross_low: SimpleCross,
}

impl RPI {
    pub fn new(period: usize, signal: usize, atr_per: f64) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                min_ma: EMA::new(period)?,
                min_sig: EMA::new(signal)?,
                max_ma: EMA::new(period)?,
                max_sig: EMA::new(signal)?,
                atr: ATR::new(period)?,
                atr_per,
                cross_high: SimpleCross::new(),
                cross_low: SimpleCross::new(),
            }),
        }
    }
    // simple - no trans
    pub fn next(&mut self, candle: impl OHLCV) -> RPIRes {
        let atr = self.atr.next(&candle);
        let min = self.min_ma.next(candle.low());
        let max = self.max_ma.next(candle.high());

        let low_ch = min - self.atr_per * atr;
        let high_ch = max + self.atr_per * atr;

        let p_high = candle.high();
        let p_low = candle.low();
        let p_close = candle.close();

        let min_sig = self.min_sig.next(p_low);
        let max_sig = self.max_sig.next(p_high);

        let cr_high = self.cross_high.next_v2(p_high, max);
        let cr_low = self.cross_low.next_v2(p_low, min);
        // let cr_high = self.cross_high.next_v2(p_high, max_sig);
        // let cr_low = self.cross_low.next_v2(p_low, min_sig);

        RPIRes {
            high: high_ch,
            high_sig: max_sig,
            low: low_ch,
            low_sig: min_sig,
            p_high,
            p_low,
            p_close,
            buy_low: cr_low.crossed_above,
            buy_high: cr_high.crossed_under,
        }
    }

    pub fn next_trans(&mut self, candle: impl OHLCV) -> RPIRes {
        let atr = self.atr.next(&candle);
        let min = self.min_ma.next(candle.low());
        let max = self.max_ma.next(candle.high());

        let low_ch = min - self.atr_per * atr;
        let high_ch = max + self.atr_per * atr;

        let zero = (high_ch + low_ch) / 2.;
        let dist = (high_ch - low_ch) / 2.;

        let p_high = (candle.high() - zero) / dist;
        let p_low = (candle.low() - zero) / dist;
        let p_close = (candle.close() - zero) / dist;

        let min_sig = self.min_sig.next(p_low);
        let max_sig = self.max_sig.next(p_high);

        let cr_high = self.cross_high.next_v2(p_high, max_sig);
        let cr_low = self.cross_low.next_v2(p_low, min_sig);

        // let min_sig = self.min_sig.next(min);
        // let max_sig = self.max_sig.next(max);
        //
        // let cr_high = self.cross_high.next_v2(max, max_sig);
        // let cr_low = self.cross_low.next_v2(min, min_sig);

        RPIRes {
            high: 1.,
            high_sig: max_sig,
            low: -1.,
            low_sig: min_sig,
            p_high,
            p_low,
            p_close,
            buy_low: cr_low.crossed_above,
            buy_high: cr_high.crossed_under,
        }
    }
}

impl Default for RPI {
    fn default() -> Self {
        Self::new(14, 3, 0.4).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        RPI::default();
    }
}

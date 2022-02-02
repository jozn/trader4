use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};

use super::*;

// Designed by me:
// Relative Price Channel

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RPCRes {
    pub high: f64,
    pub low: f64,
    pub p_high: f64,
    pub p_low: f64,
    pub p_close: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPC {
    min_ma: EMA,
    max_ma: EMA,
    atr: ATR,
    atr_per: f64,
    cross: SimpleCross,
}

impl RPC {
    pub fn new(period: usize, atr_per: f64) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                min_ma: EMA::new(period)?,
                max_ma: EMA::new(period)?,
                atr: ATR::new(period)?,
                atr_per,
                cross: SimpleCross::new(),
            }),
        }
    }

    pub fn next_bk(&mut self, candle: impl OHLCV) -> RPCRes {
        let atr = self.atr.next(&candle);
        let min = self.min_ma.next(candle.low());
        let max = self.max_ma.next(candle.high());

        let low_ch = min - self.atr_per * atr;
        let high_ch = max + self.atr_per * atr;

        RPCRes {
            high: high_ch,
            low: low_ch,
            p_high: candle.high(),
            p_low: candle.low(),
            p_close: candle.close(),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> RPCRes {
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

        RPCRes {
            high: 1.,
            low: -1.,
            p_high,
            p_low,
            p_close,
        }
    }
}

impl Default for RPC {
    fn default() -> Self {
        Self::new(14, 0.4).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        RPC::default();
    }
}

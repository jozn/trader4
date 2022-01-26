use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

// Moving Average Trend:
// You can see an impl of this in TradingView with name of "SSL Channel"

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MATrend {
    is_new: bool,
    pre_dir: i8,
    high_ma: EMA,
    low_ma: EMA,
    cross: SimpleCross,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MATrendOut {
    pub bull_line: f64,
    pub bear_line: f64,
    pub bull_above: bool, // true when plus crossed above the minus line - bullish
    pub bull_under: bool, // true when plus crossed under the minus line - bearish
}

impl MATrendOut {
    pub fn is_bullish(&self) -> bool {
        self.bull_line > self.bear_line
    }

    pub fn is_bearish(&self) -> bool {
        self.bull_line < self.bear_line
    }
}

impl MATrend {
    pub fn new(period: usize) -> TAResult<Self> {
        if period == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                is_new: false,
                pre_dir: 0,
                high_ma: EMA::new(period)?,
                low_ma: EMA::new(period)?,
                cross: Default::default(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> MATrendOut {
        let high_ma = self.high_ma.next(candle.high());
        let low_ma = self.low_ma.next(candle.low());

        let close = candle.close();

        let dir = if close > high_ma {
            1
        } else if close < low_ma {
            -1
        } else {
            self.pre_dir
        };
        self.pre_dir = dir;

        // Code below orgonized in a manner to start with bull in first candles.
        let bull_line = if dir > 0 { high_ma } else { low_ma };

        let bear_line = if dir > 0 { low_ma } else { high_ma };

        let cr = self.cross.next_v2(bull_line, bear_line);

        MATrendOut {
            bull_line,
            bear_line,
            bull_above: cr.crossed_above,
            bull_under: cr.crossed_under,
        }
    }
}

impl Default for MATrend {
    fn default() -> Self {
        Self::new(10).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MACDOutput;
    use super::*;
}

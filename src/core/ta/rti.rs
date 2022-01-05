use serde::{Deserialize, Serialize};

use super::*;

// Note: this indicator is designed by ourself. Inspired by RSI and Donchain Channel for trend detection.

pub type RTI = RelativeTrendIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeTrendIndex {
    period: usize,
    bull_ma: EMA,
    // bull_ma: SMA,
    bear_ma: EMA,
    // bear_ma: SMA,
    prev_high: f64,
    prev_low: f64,
    is_new: bool,
}

impl RelativeTrendIndex {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                bull_ma: EMA::new_alpha(period, 1.).unwrap(),
                // bull_ma: SMA::new(period).unwrap(),
                bear_ma: EMA::new_alpha(period, 1.).unwrap(),
                // bear_ma: SMA::new(period).unwrap(),
                prev_high: 0.0,
                prev_low: 0.0,
                is_new: true,
            }),
        }
    }

    pub(crate) fn next(&mut self, next_high: f64, next_low: f64) -> f64 {
        let mut bull = 0.0;
        let mut bear = 0.0;

        let next_high = if next_high.is_nan() { 0. } else { next_high };
        // println!("nan: {}", nex_val);

        if self.is_new {
            self.is_new = false;
            // Initialize with some small seed numbers to avoid division by zero,
            //  this numbers should be very low in order to avoid small forex numbers.
            bull = 0.00001;
            bear = 0.00001;
        } else {
            // bull : higher high + higher low
            if next_high > self.prev_high {
                bull += next_high - self.prev_high;
            }
            if next_low > self.prev_low {
                bull += next_low - self.prev_low;
            }

            // bear : lower low + lower high
            if next_low < self.prev_low {
                bear += self.prev_low - next_low;
            }
            if next_high < self.prev_high {
                bear += self.prev_high - next_high;
            }
        }

        self.prev_high = next_high;
        self.prev_low = next_low;

        // let up_ema = self.bull_ma.next(bull).max(0.00001);
        let up_ema = self.bull_ma.next(bull);
        // let down_ema = self.bear_ma.next(bear).max(0.00001);
        let down_ema = self.bear_ma.next(bear);

        if up_ema != 0. || down_ema != 0. {
            let rs = up_ema / down_ema;
            // println!("down: {}  {}", rs, down_ema);
            // 100.0 * up_ema / (up_ema + down_ema) // old formula - from one rust lib
            100. - (100. / (1. + rs)) // real formula from tradingview and investpeida and wiki
        } else {
            50.
        }
    }
}

impl Default for RelativeTrendIndex {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(RelativeTrendIndex::new(0).is_err());
        assert!(RelativeTrendIndex::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rsi = RelativeTrendIndex::new(3).unwrap();
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);
        assert_eq!(rsi.next(10.0).round(), 35.0);
        assert_eq!(rsi.next(9.5).round(), 16.0);
    }

    #[test]
    fn test_default() {
        RelativeTrendIndex::default();
    }
}

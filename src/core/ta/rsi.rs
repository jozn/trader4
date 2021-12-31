use serde::{Deserialize, Serialize};

use super::*;

pub type RSI = RelativeStrengthIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeStrengthIndex {
    period: usize,
    up_ma: EMA,
    down_ma: EMA,
    prev_val: f64,
    is_new: bool,
}

impl RelativeStrengthIndex {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                up_ma: EMA::new(period).unwrap(),
                down_ma: EMA::new(period).unwrap(),
                prev_val: 0.0,
                is_new: true,
            }),
        }
    }

    pub(crate) fn next(&mut self, next_val: f64) -> f64 {
        let mut up = 0.0;
        let mut down = 0.0;

        if self.is_new {
            self.is_new = false;
            // Initialize with some small seed numbers to avoid division by zero,
            //  this numbers should be very low in order to avoid small forex numbers.
            up = 0.00001;
            down = 0.00001;
        } else {
            if next_val > self.prev_val {
                up = next_val - self.prev_val;
            } else {
                down = self.prev_val - next_val;
            }
        }

        self.prev_val = next_val;
        // up *= 10_000.;
        // down *= 10_000.;
        // todo correct?
        let up_ema = self.up_ma.next(up);
        let down_ema = self.down_ma.next(down);

        if up_ema != 0. || down_ema != 0. {
            let rs = up_ema / down_ema;
            // 100.0 * up_ema / (up_ema + down_ema) // old formula - from one rust lib
            100. - (100. / (1. + rs)) // real formula from tradingview and investpeida and wiki
        } else {
            50.
        }
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(RelativeStrengthIndex::new(0).is_err());
        assert!(RelativeStrengthIndex::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rsi = RelativeStrengthIndex::new(3).unwrap();
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);
        assert_eq!(rsi.next(10.0).round(), 35.0);
        assert_eq!(rsi.next(9.5).round(), 16.0);
    }

    #[test]
    fn test_default() {
        RelativeStrengthIndex::default();
    }
}

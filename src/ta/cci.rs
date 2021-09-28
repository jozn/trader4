use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCI {
    period: usize,
    sma: SMA,
    mad: AverageAbsoluteDeviation,
    is_new: bool,
}

impl CCI {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                sma: SMA::new(period).unwrap(),
                mad: AverageAbsoluteDeviation::new(period).unwrap(),
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> f64 {
        let tp = (candle.high() + candle.low() + candle.close()) / 3.0;
        self._next_(tp as f64)
    }

    fn _next_(&mut self, typical_price: f64) -> f64 {
        let sma_val = self.sma.next(typical_price);
        let mad_val = self.mad.next(typical_price);

        if mad_val == 0.0 {
            1.0
        } else {
            (typical_price - sma_val) / (mad_val * 0.015)
        }
    }
}

impl Default for CCI {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(CCI::new(0).is_err());
        assert!(CCI::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut cci = CCI::new(3).unwrap();
        /*
                assert_eq!(cci._next_(2.0), 2.0);
                assert_eq!(cci._next_(5.0), 3.5);
                assert_eq!(cci._next_(1.0), 2.25);
                assert_eq!(cci._next_(6.25), 4.25);
        */
    }

    #[test]
    fn test_default() {
        CCI::default();
    }
}

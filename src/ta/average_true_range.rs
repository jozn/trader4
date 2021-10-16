use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

type ATR = AverageTrueRange;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AverageTrueRange {
    true_range: TrueRange,
    ema: EMA,
}

impl AverageTrueRange {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                true_range: Default::default(),
                ema: EMA::new(period).unwrap(),
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> f64 {
        let tr_val = self.true_range.next(candle);
        let atr_val = self.ema.next(tr_val);
        atr_val
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(AverageTrueRange::new(0).is_err());
        assert!(AverageTrueRange::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _atr = AverageTrueRange::new(3).unwrap();
    }

    #[test]
    fn test_default() {
        AverageTrueRange::default();
    }
}

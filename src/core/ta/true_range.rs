use serde::{Deserialize, Serialize};

use super::*;
use crate::base::OHLCV;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrueRange {
    prev_close: Option<f64>,
}

impl TrueRange {
    pub fn new() -> Self {
        Self { prev_close: None }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> f64 {
        let dis = match self.prev_close {
            None => candle.high() - candle.low(),
            Some(prev_close) => {
                let dist1 = candle.high() - candle.low();
                let dist2 = (candle.high() - prev_close).abs();
                let dist3 = (candle.low() - prev_close).abs();
                dist1.max(dist2).max(dist3)
            }
        };
        // println!("{}", dis);
        self.prev_close = Some(candle.close());

        dis
    }
}

impl Default for TrueRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(TrueRange::new().prev_close.is_none());
    }

    #[test]
    fn test_next() {
        let vals = vec![
            // Data structure: (OHLCV, True_Range_Result)
            ((10., 12., 9., 11., 0.), 3.0),
            ((11., 12.5, 10.5, 11., 0.), 2.),
            ((11., 15., 10., 14., 0.), 5.),
        ];
        let mut tr = TrueRange::new();

        for v in vals {
            assert_eq!(tr.next(v.0), v.1);
        }
    }

    #[test]
    fn test_default() {
        TrueRange::default();
    }
}

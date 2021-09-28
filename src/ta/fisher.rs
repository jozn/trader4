use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fisher {
    signal_period: usize,
    low: Minimum,
    high: Maximum,
    prev_val: f64,
    prev_x: f64,
    cross: SimpleCross,
    res_window: Window,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FisherRes {
    pub fisher: f64,
    pub signal: SignalsRes,
}

impl Fisher {
    pub fn new(period: usize, signal_period: usize) -> TAResult<Self> {
        if period == 0 || signal_period == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                signal_period: signal_period,
                low: Minimum::new(period).unwrap(),
                high: Maximum::new(period).unwrap(),
                prev_val: 0.0,
                prev_x: 0.0,
                cross: SimpleCross::new(),
                res_window: Window::new(signal_period).unwrap(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> FisherRes {
        let next_val = candle.hl2();
        let tp = candle.hl2() as f64;

        let _alpha = 0.33;
        let high = self.high.next(tp);
        let low = self.low.next(tp);
        let (x, y) = if low != high {
            let x = 0.66 * ((next_val - low) / (high - low) - 0.5) + 0.67 * self.prev_x;
            let y = fisher_transform(x);

            (x, y)
        } else {
            (0., 0.)
        };

        let last_y = y + 0.5 * self.prev_val;

        let event = self.cross.next(last_y, self.prev_val);

        self.prev_val = last_y;
        self.prev_x = x;
        self.res_window.push(last_y);

        FisherRes {
            fisher: last_y,
            signal: event,
        }
    }
}

fn fisher_transform(v: f64) -> f64 {
    let v = if v > 0.999 {
        0.999
    } else if v < -0.999 {
        -0.999
    } else {
        v
    };

    // note f64::atanh() should works the same but the source formula seems to be diffrent
    0.5 * ((1. + v) / (1. - v)).ln()
}

impl Default for Fisher {
    fn default() -> Self {
        Self::new(9, 6).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Fisher::new(0, 12).is_err());
        assert!(Fisher::new(1, 1).is_ok());
    }

    #[test]
    fn test_fisher() {}

    #[test]
    fn test_default() {
        Fisher::default();
    }
}

use serde::{Deserialize, Serialize};

use super::*;
use crate::base::OHLCV;

pub type DC = DonchainChannel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonchainChannel {
    period: usize,
    window_low: Window, // todo may change to Max and Min
    window_high: Window,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DCRes {
    pub high: f64,
    pub middle: f64,
    pub low: f64,
}

impl DonchainChannel {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                window_low: Window::new(period).unwrap(),
                window_high: Window::new(period).unwrap(),
            }),
        }
    }

    pub(crate) fn next(&mut self, candle: impl OHLCV) -> DCRes {
        self.window_low.push(candle.low());
        self.window_high.push(candle.high());

        let low = self.window_low.iter().fold(f64::MAX, |m, i| m.min(*i));
        let high = self
            .window_high
            .iter()
            .fold(f64::MIN, |m: f64, i| m.max(*i));

        DCRes {
            high,
            middle: (high + low) / 2.,
            low,
        }
    }
}

impl Default for DonchainChannel {
    fn default() -> Self {
        Self::new(20).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(DonchainChannel::new(0).is_err());
        assert!(DonchainChannel::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let vals = vec![
            // Data structure: (OHLCV, (high,low))
            ((10., 12., 9., 11., 0.), (12., 9.)),
            ((11., 13., 10.5, 12., 0.), (13., 9.)),
            ((12., 12.5, 11.5, 12., 0.), (13., 9.)),
            ((12., 14., 8., 11.5, 0.), (14., 8.)),
        ];

        let mut dc = DonchainChannel::new(3).unwrap();

        for v in vals {
            let r = dc.next(v.0);
            assert_eq!(r.high, v.1 .0);
            assert_eq!(r.low, v.1 .1);
        }
    }

    #[test]
    fn test_default() {
        DonchainChannel::default();
    }
}

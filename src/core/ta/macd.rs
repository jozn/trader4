use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

pub type MACD = MovingAverageConvergenceDivergence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverageConvergenceDivergence {
    fast_ma: EMA,
    slow_ma: EMA,
    signal_ma: EMA,
    cross: SimpleCross,
    vel: Vel,
    last: Option<MACDOutput>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MACDOutput {
    pub macd: f64, // formula (fast_ma - slow_ma) -- the changing line (blue in TradingView)
    #[serde(skip)]
    pub macd_pop: f64, // maybe delete - just for debug
    pub signal: f64, // EMA of macd
    pub histogram: f64,
    pub macd_above: bool, // true when macd crossed above the signal line - bullish
    pub macd_under: bool, // true when macd crossed under the signal line - bearish
    pub dir: f64,
    pub color: f64, // -2: red and decresing , -1: red but increasing(+) , +1: blue but decresing, 2: blue and increains
}

impl MovingAverageConvergenceDivergence {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> TAResult<Self> {
        if fast_period == 0 || slow_period == 0 || signal_period == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                fast_ma: EMA::new(fast_period)?,
                slow_ma: EMA::new(slow_period)?,
                signal_ma: EMA::new(signal_period)?,
                cross: Default::default(),
                vel: Vel::new(signal_period)?,
                last: None,
            })
        }
    }

    pub fn next(&mut self, next_val: f64) -> MACDOutput {
        let fast_ma = self.fast_ma.next(next_val);
        let slow_ma = self.slow_ma.next(next_val);

        let macd = fast_ma - slow_ma;
        let signal = self.signal_ma.next(macd);
        let histogram = macd - signal;

        let cr = self.cross.next_v2(macd, signal);
        let vel = self.vel.next(macd);
        let dir = if vel.avg_vel_pip > 0. { 1. } else { -1. };
        let color = match &self.last {
            None => 0.,
            Some(lo) => {
                if histogram > 0. {
                    if histogram > lo.histogram {
                        2.
                    } else {
                        1.
                    }
                } else if histogram < 0. {
                    if histogram < lo.histogram {
                        -2.
                    } else {
                        -1.
                    }
                } else {
                    0.
                }
            }
        };

        let out = MACDOutput {
            macd,
            macd_pop: macd * 10_000.,
            signal: signal,
            histogram,
            macd_above: cr.crossed_above,
            macd_under: cr.crossed_under,
            dir,
            color,
        };

        self.last = Some(out.clone());

        out
    }
}

impl Default for MovingAverageConvergenceDivergence {
    fn default() -> Self {
        Self::new(12, 26, 9).unwrap()
    }
}

impl From<MACDOutput> for (f64, f64, f64) {
    fn from(mo: MACDOutput) -> Self {
        (mo.macd, mo.signal, mo.histogram)
    }
}

#[cfg(test)]
mod tests {
    use super::MACDOutput;
    use super::*;

    #[test]
    fn test_new() {
        assert!(MACD::new(0, 12, 3).is_err());
        assert!(MACD::new(1, 1, 1).is_ok());
    }

    #[test]
    fn test_macd() {
        let mut macd = MACD::new(3, 6, 4).unwrap();

        assert_eq!(round(macd.next(2.0).into()), (0.0, 0.0, 0.0));
        assert_eq!(round(macd.next(3.0).into()), (0.21, 0.09, 0.13));
        assert_eq!(round(macd.next(4.2).into()), (0.52, 0.26, 0.26));
        assert_eq!(round(macd.next(7.0).into()), (1.15, 0.62, 0.54));
        assert_eq!(round(macd.next(6.7).into()), (1.15, 0.83, 0.32));
        assert_eq!(round(macd.next(6.5).into()), (0.94, 0.87, 0.07));
    }

    #[test]
    fn test_default() {
        MACD::default();
    }

    fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        let n2 = (nums.2 * 100.0).round() / 100.0;
        (n0, n1, n2)
    }
}

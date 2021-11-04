use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

// Velocity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vel {
    period: usize,
    ema: EMA,
    diff: Momentum, // diferece to prev
    window: Window, // diferece to prev
    _buff: Window,  // A buffer to avoid allocating in each call
    is_new: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VelRes {
    pub start_vel: f64,
    pub count: u32, // all positive/negative candles counts
    pub avg_vel: f64,
    pub end_vel: f64,
}

impl Vel {
    pub fn new(period: usize) -> TAResult<Self> {
        println!("period: {}", period);
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                ema: EMA::new(period).unwrap(),
                diff: Momentum::new(1).unwrap(),
                window: Window::new(200).unwrap(),
                _buff: Window::new(200).unwrap(),
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> VelRes {
        let tp = (candle.high() + candle.low() + candle.close()) / 3.0;
        self._next_(tp as f64)
    }

    fn _next_(&mut self, typical_price: f64) -> VelRes {
        let p = typical_price;
        let sma_val = self.ema.next(typical_price);
        let end_vel = self.diff.next(sma_val);

        self.window.push(end_vel);

        let mut sum = 0.0;
        let mut count = 0;
        self._buff.clear();

        if end_vel > 0. {
            // MARK A
            for v in self.window.iter().rev() {
                let v = *v;
                if v > 0. {
                    sum += v;
                    count += 1;
                    self._buff.push(v);
                } else {
                    break;
                }
            }
        } else {
            // COPY OF A WITH NEGATIVE CHECK
            for v in self.window.iter().rev() {
                let v = *v;
                if v < 0. {
                    sum += v;
                    count += 1;
                    self._buff.push(v);
                } else {
                    break;
                }
            }
        }

        let start_sum: f64 = self._buff.iter().rev().take(3).sum();
        let start_size: f64 = self._buff.iter().rev().take(3).count() as f64;

        VelRes {
            start_vel: start_sum / sma_val,
            count: count,
            avg_vel: sum / count as f64,
            end_vel,
        }
    }
}

impl Default for Vel {
    fn default() -> Self {
        Self::new(99).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Vel::new(0).is_err());
        assert!(Vel::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut cci = Vel::new(3).unwrap();
        /*
                assert_eq!(cci._next_(2.0), 2.0);
                assert_eq!(cci._next_(5.0), 3.5);
                assert_eq!(cci._next_(1.0), 2.25);
                assert_eq!(cci._next_(6.25), 4.25);
        */
    }

    #[test]
    fn test_default() {
        Vel::default();
    }
}

use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;
use std::collections::VecDeque;

// Velocity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelDep {
    period: usize,
    ema: EMA,
    last_ema: f64,
    diff: Momentum, // diferece to prev
    store: VecDeque<f64>,
    store_ema: VecDeque<f64>,
    window_1: Window,    // diferece to prev
    buff: VecDeque<f64>, // A buffer to avoid allocating in each call
    is_new: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VelResDep {
    pub start_vel: f64,
    pub count: u32, // all positive/negative candles counts
    pub avg_vel: f64,
    pub end_vel: f64,
}

impl VelDep {
    pub fn new(period: usize) -> TAResult<Self> {
        // println!("period: {}", period);
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                ema: EMA::new(period).unwrap(),
                last_ema: 0.0,
                diff: Momentum::new(1).unwrap(),
                store: VecDeque::with_capacity(200),
                store_ema: VecDeque::with_capacity(200),
                window_1: Window::new(200).unwrap(),
                // _buff: Window::new(200).unwrap(),
                buff: VecDeque::with_capacity(200),
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> VelResDep {
        let tp = (candle.high() + candle.low() + candle.close()) / 3.0;
        self._next_(tp as f64)
    }

    fn _next_(&mut self, typical_price: f64) -> VelResDep {
        let p = typical_price;
        let new_ema = self.ema.next(typical_price);
        if self.is_new {
            self.is_new = false;
            self.last_ema = new_ema;
        }

        let end_vel = new_ema - self.last_ema;
        self.last_ema = new_ema;

        self.store.push_front(end_vel);
        self.store_ema.push_front(new_ema);
        if self.store.len() == self.period {
            self.store.pop_back(); // remove
            self.store_ema.pop_back(); // remove
        }

        let mut sum = 0.0;
        let mut count = 0;
        self.buff.clear();

        /*
             println!("eeema dif: {:?}", self.store);
             println!("eeema: {:?}", self.store_ema);
             println!("=============");
        */

        if end_vel > 0. {
            // MARK A
            for v in self.store.iter() {
                let v = *v;
                if v > 0. {
                    sum += v;
                    count += 1;
                    self.buff.push_front(v);
                } else {
                    break;
                }
            }
        } else {
            // COPY OF A WITH NEGATIVE CHECK
            for v in self.store.iter() {
                let v = *v;
                if v < 0. {
                    sum += v;
                    count += 1;
                    self.buff.push_front(v);
                } else {
                    break;
                }
            }
        }

        let start_sum: f64 = self.buff.iter().take(3).sum();
        let start_size: f64 = self.buff.iter().take(3).count() as f64;

        VelResDep {
            start_vel: start_sum / new_ema,
            count: count,
            avg_vel: sum / count as f64,
            end_vel,
        }
    }
}

impl Default for VelDep {
    fn default() -> Self {
        Self::new(99).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(VelDep::new(0).is_err());
        assert!(VelDep::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut cci = VelDep::new(3).unwrap();
        /*
                assert_eq!(cci._next_(2.0), 2.0);
                assert_eq!(cci._next_(5.0), 3.5);
                assert_eq!(cci._next_(1.0), 2.25);
                assert_eq!(cci._next_(6.25), 4.25);
        */
    }

    #[test]
    fn test_default() {
        VelDep::default();
    }
}

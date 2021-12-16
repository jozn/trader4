use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;
use std::collections::VecDeque;

const MULTIPLIER: f64 = 1_000_000.0;

// Velocity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vel {
    ema: EMA,
    last_ema: f64,
    store_ema: VecDeque<f64>,
    buff: VecDeque<f64>, // A buffer to avoid allocating in each call
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
    pub fn new(ema_period: usize) -> TAResult<Self> {
        // println!("period: {}", period);
        match ema_period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                ema: EMA::new(ema_period).unwrap(),
                last_ema: 0.,
                store_ema: VecDeque::with_capacity(200),
                buff: VecDeque::with_capacity(200),
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> VelRes {
        let tp = (candle.high() + candle.low() + candle.close()) / 3.0;
        self._next_(tp)
    }

    pub fn _next_(&mut self, typical_price: f64) -> VelRes {
        let new_ema = self.ema.next(typical_price);
        // let new_ema_u64 = (new_ema * MULTIPLIER) as u64 ;
        if self.is_new {
            self.is_new = false;
            self.last_ema = new_ema;
        }
        let last_ema = self.last_ema;

        // Note: push_front means the start of vector
        self.store_ema.push_front(new_ema);
        if self.store_ema.len() == 100 {
            self.store_ema.pop_back(); // remove
        }

        let mut count = 0;
        // self.buff.clear();

        let mut buff = vec![];
        let end_vel = new_ema - last_ema;
        // let first = new_ema;
        let mut last = new_ema;

        if end_vel > 0. {
            // bullish
            // MARK A
            for v in self.store_ema.iter() {
                let v = *v;
                if last >= v {
                    last = v;
                    // sum += v;
                    count += 1;
                    buff.push(v);
                } else {
                    break;
                }
            }
        } else {
            // COPY OF A WITH NEGATIVE CHECK
            for v in self.store_ema.iter() {
                let v = *v;
                if last <= v {
                    last = v;
                    // sum += v;
                    count += 1;
                    buff.push(v);
                } else {
                    break;
                }
            }
        }

        // println!("{:?}", buff);

        let avg_vel = if count >= 2 {
            // Note: can divide by zero in f64
            (new_ema - last) / (count - 1) as f64
        } else {
            0.
        };
        // println!("eeema: {:?}", self.store_ema);
        // println!("=============");

        let start_vec: Vec<f64> = buff.iter().rev().take(3).map(|v| *v).rev().collect();
        let start_vel = if start_vec.len() >= 2 {
            let first = start_vec.first().unwrap();
            let last = start_vec.last().unwrap();
            (first - last) / (start_vec.len() - 1) as f64
        } else {
            0.
        };

        self.last_ema = new_ema;

        VelRes {
            start_vel,
            count: count - 1,
            avg_vel,
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
        let nums = vec![
            1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.5, 1.3, 1.2, 1.1, 1.0, 0.9, 0.8, 0.7, 0.6,
        ];

        for p in nums {
            let r = cci._next_(p);
            println!("{} - {:#?}  {:#?}", p, r, cci);
        }
    }

    #[test]
    fn test_default() {
        Vel::default();
    }
}

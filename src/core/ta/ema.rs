use serde::{Deserialize, Serialize};

use super::*;

pub type EMA = ExponentialMovingAverage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialMovingAverage {
    period: usize,
    k: f64,
    current: f64,
    is_new: bool,
}

impl ExponentialMovingAverage {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                k: 2.0 / (period + 1) as f64,
                current: 0.0,
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        if self.is_new {
            self.is_new = false;
            self.current = next_val;
        } else {
            self.current = (next_val * self.k) + (self.current * (1. - self.k))
        }
        self.current
    }

    fn next_peek(&self, next_val: f64) -> f64 {
        if self.is_new {
            return next_val;
        } else {
            (next_val * self.k) + (self.current * (1. - self.k))
        }
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(ExponentialMovingAverage::new(0).is_err());
        assert!(ExponentialMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next(6.25), 4.25);
    }

    #[test]
    fn test_next_one_period() {
        let mut ema = ExponentialMovingAverage::new(1).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 5.0);
        assert_eq!(ema.next(5.5), 5.5);
        // assert_eq!(ema.next(6.25), 4.25);
    }

    #[test]
    fn test_next_peek() {
        let mut ema = ExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next_peek(2.0), 2.0);
        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next_peek(5.0), 3.5);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next_peek(1.0), 2.25);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next_peek(6.25), 4.25);
        assert_eq!(ema.next(6.25), 4.25);
    }

    #[test]
    fn test_default() {
        ExponentialMovingAverage::default();
    }
}

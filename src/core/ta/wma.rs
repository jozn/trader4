use serde::{Deserialize, Serialize};

use super::*;

pub type WMA = WeightedMovingAverage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedMovingAverage {
    period: usize,
    sum_weights: f64,
    total: f64,
    numerator: f64,
    is_new: bool,
    window: Window,
}

impl WeightedMovingAverage {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                sum_weights: 0.0,
                total: 0.0,
                numerator: 0.0,
                is_new: true,
                window: Window::new(period).unwrap(),
            }),
        }
    }

    // The algoritm for this one is a bit complicated to avoid a loop (faster) - see wiki page
    pub fn next(&mut self, next_val: f64) -> f64 {
        if self.is_new {
            let length = self.period as f64;
            let sum = (length * (length + 1.)) / 2.;

            self.sum_weights = sum;
            self.total = next_val * length;
            self.numerator = next_val * sum;
            self.window.push(next_val);
            self.is_new = false;

            return next_val;
        }

        let prev_val_opt = self.window.push(next_val);
        let prev_val = match prev_val_opt {
            None => self.window.tail().unwrap(),
            Some(tail) => tail,
        };

        self.numerator = self.numerator + (self.period as f64 * next_val) - self.total;
        self.total = self.total + next_val - prev_val;

        self.numerator / self.sum_weights
    }
}

impl Default for WeightedMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(WeightedMovingAverage::new(0).is_err());
        assert!(WeightedMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut wma = WeightedMovingAverage::new(3).unwrap();

        assert_eq!(round(wma.next(10.0)), 10.0);
        assert_eq!(round(wma.next(10.0)), 10.0);
        assert_eq!(round(wma.next(10.0)), 10.0);
        assert_eq!(round(wma.next(12.0)), 11.0);
        assert_eq!(round(wma.next(14.0)), 12.667);
    }

    #[test]
    fn test_next_entry() {
        let mut wma = WeightedMovingAverage::new(3).unwrap();

        assert_eq!(round(wma.next(10.0)), 10.0);
        assert_eq!(round(wma.next(12.0)), 11.0);
        assert_eq!(round(wma.next(14.0)), 12.667);
    }

    #[test]
    fn test_default() {
        WeightedMovingAverage::default();
    }
}

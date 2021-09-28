use serde::{Deserialize, Serialize};

use super::*;

pub type SMA = SimpleMovingAverage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleMovingAverage {
    period: usize,
    sum: f64,
    window: Window,
}

impl SimpleMovingAverage {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                sum: 0.0,
                window: Window::new(period).unwrap(),
            }),
        }
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        let old_val = self.window.push(next_val);
        match old_val {
            None => {}
            Some(last_val) => self.sum -= last_val,
        }
        self.sum += next_val;

        self.sum / self.window.size() as f64
    }

    pub fn next_peek(&self, next_val: f64) -> f64 {
        let mut size = self.window.size();
        let mut sum = self.sum;

        if size == 0 {
            return next_val;
        }

        // Only subtract the tail value if the period is full
        if size == self.period {
            let old_val = self.window.peek_period_tail();
            match old_val {
                None => {}
                Some(last_val) => {
                    sum -= last_val;
                    size -= 1;
                }
            }
        }

        sum += next_val;
        size += 1; // +1: new_val

        sum / size as f64
    }
}

impl Default for SimpleMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut sma = SimpleMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 4.5);
        // Check if tail is not discounted in not fully filled periods
        assert_eq!(sma.next_peek(6.0), 5.0);

        assert_eq!(sma.next(6.0), 5.0);
        assert_eq!(sma.next(6.0), 5.25);
        assert_eq!(sma.next(6.0), 5.75);
        assert_eq!(sma.next(6.0), 6.0);
        assert_eq!(sma.next(2.0), 5.0);
        assert_eq!(sma.next_peek(10.0), 6.0);
        assert_eq!(sma.next_peek(2.0), 4.0);
    }

    #[test]
    fn test_next_peek() {
        let mut sma = SimpleMovingAverage::new(4).unwrap();
        assert_eq!(sma.next_peek(4.0), 4.0);
        assert_eq!(sma.next_peek(4.0), 4.0);
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 4.5);
        assert_eq!(sma.next(6.0), 5.0);
        // check if tail is not discounted in not fully filled periods
        assert_eq!(sma.next_peek(1.0), 4.0);
        assert_eq!(sma.next_peek(7.0), 5.5);

        assert_eq!(sma.next(7.0), 5.5);
        assert_eq!(sma.next(2.0), 5.0);
    }
}

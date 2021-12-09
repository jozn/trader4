use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AverageAbsoluteDeviation {
    period: usize,
    sum: f64,
    window: Window,
}

impl AverageAbsoluteDeviation {
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
            Some(last_val) => {
                self.sum -= last_val;
            }
        }
        self.sum += next_val;
        let mean = self.sum / self.window.size() as f64;
        let mut mad = 0.;

        // Note:
        //   this loop can be avoided with some fancy algorithms but it not worth the complexity
        for v in self.window.iter() {
            mad += (v - mean).abs();
        }

        mad / self.window.size() as f64
    }

    pub fn next_peek(&self, next_val: f64) -> f64 {
        let mut sum = self.sum;
        let mut size = self.window.size();
        let old_val = self.window.peek_period_tail();
        let mut old_val_real = 0.;
        match old_val {
            None => {}
            Some(last_val) => {
                sum -= last_val;
                size -= 1;
                old_val_real = last_val;
            }
        }
        sum += next_val;
        size += 1;
        let mean = sum / size as f64;
        let mut mad = 0.;

        let mut size_win = self.window.size();
        for v in self.window.iter() {
            mad += (v - mean).abs();
        }
        // For next value
        mad += (next_val - mean).abs();
        // Remove the last tailed value if needed
        if old_val_real > 0. {
            mad -= (old_val_real - mean).abs();
        }

        mad / size as f64
    }
}

impl Default for AverageAbsoluteDeviation {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut aad = AverageAbsoluteDeviation::new(5).unwrap();

        assert_eq!(round(aad.next(1.5)), 0.0);
        assert_eq!(round(aad.next(4.0)), 1.25);
        assert_eq!(round(aad.next(8.0)), 2.333);
        assert_eq!(round(aad.next(4.0)), 1.813);
        assert_eq!(round(aad.next(4.0)), 1.48);
        assert_eq!(round(aad.next(1.5)), 1.48);
    }

    #[test]
    fn test_next_peek() {
        let mut aad = AverageAbsoluteDeviation::new(5).unwrap();
        assert_eq!(0.0, 0.);
        assert_eq!(round(aad.next_peek(1.5)), 0.0);
        assert_eq!(round(aad.next(1.5)), 0.0);
        assert_eq!(round(aad.next_peek(4.0)), 1.25);
        assert_eq!(round(aad.next(4.0)), 1.25);
        assert_eq!(round(aad.next_peek(8.0)), 2.333);
        assert_eq!(round(aad.next(8.0)), 2.333);
        assert_eq!(round(aad.next_peek(4.0)), 1.813);
        assert_eq!(round(aad.next(4.0)), 1.813);
        assert_eq!(round(aad.next_peek(4.0)), 1.48);
        assert_eq!(round(aad.next(4.0)), 1.48);
        assert_eq!(round(aad.next_peek(1.5)), 1.48);
        assert_eq!(round(aad.next(1.5)), 1.48);
    }

    pub fn round(num: f64) -> f64 {
        (num * 1000.0).round() / 1000.00
    }
}

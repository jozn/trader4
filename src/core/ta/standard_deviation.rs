use serde::{Deserialize, Serialize};

use super::*;

// https://www.investopedia.com/terms/s/standarddeviation.asp
// https://en.wikipedia.org/wiki/Standard_deviation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardDeviation {
    ma: SMA,
    window: Window,
}

impl StandardDeviation {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                ma: SMA::new(period)?,
                window: Window::new(period)?,
            }),
        }
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        let ma = self.ma.next(next_val);
        let _old_val = self.window.push(next_val);
        let n_1 = if self.window.size() == 1 {
            1.
        } else {
            self.window.size() as f64 - 1.
        };

        let mut sum = 0.;
        for price in self.window.iter() {
            sum += (price - ma).powf(2.);
        }

        (sum / n_1).sqrt()
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut _aad = StandardDeviation::new(5).unwrap();
    }
}

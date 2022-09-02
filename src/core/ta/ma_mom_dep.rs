use serde::{Deserialize, Serialize};

use super::*;

// Moving Average Momentum

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAMomDep {
    period: usize,
    ma: EMA,
    window: Window,
}

impl MAMomDep {
    pub fn new(period: usize, mom_diff: usize) -> TAResult<Self> {
        if period == 0 || mom_diff == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                period,
                ma: EMA::new(period)?,
                window: Window::new(mom_diff + 1)?,
            })
        }
    }

    pub(crate) fn next(&mut self, next_val: f64) -> f64 {
        let ma = self.ma.next(next_val);
        let _last = self.window.push(ma);
        let tail = self.window.tail().unwrap();

        (ma - tail) * 10_000.
    }
}

impl Default for MAMomDep {
    fn default() -> Self {
        Self::new(9, 3).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(MAMomDep::new(0, 0).is_err());
        assert!(MAMomDep::new(1, 1).is_ok());
    }

    #[test]
    fn test_next() {}

    #[test]
    fn test_default() {
        MAMomDep::default();
    }
}

use serde::{Deserialize, Serialize};

use super::*;

// Moving Average Momentum

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MAMomOut {
    pub ma: f64,
    pub mom: f64,
    pub mom_mom: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAMom {
    period: usize,
    ma: EMA,
    mom: Momentum,
    mom_mom: Momentum,
}

impl MAMom {
    pub fn new(period: usize, mom_diff: usize, mom_mom: usize) -> TAResult<Self> {
        if period == 0 || mom_diff == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                period,
                ma: EMA::new(period)?,
                // window: Window::new(mom_diff + 1)?,
                mom: Momentum::new(mom_diff)?,
                mom_mom: Momentum::new(mom_mom)?,
            })
        }
    }

    pub(crate) fn next(&mut self, next_val: f64) -> MAMomOut {
        let ma = self.ma.next(next_val);
        let mom = self.mom.next(ma);
        let mom_mom = self.mom_mom.next(mom);

        MAMomOut { ma, mom, mom_mom }
    }
}

impl Default for MAMom {
    fn default() -> Self {
        Self::new(10, 5, 5).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(MAMom::new(0, 0, 0).is_err());
        assert!(MAMom::new(1, 1, 0).is_ok());
    }

    #[test]
    fn test_next() {}

    #[test]
    fn test_default() {
        MAMom::default();
    }
}

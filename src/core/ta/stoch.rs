use crate::base::SimpleCross;
use serde::{Deserialize, Serialize};

use super::*;

// https://www.investopedia.com/terms/s/stochrsi.asp
// Stochastic itself

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StochRes {
    pub main_k: f64,
    pub smooth_d: f64,
    pub k_above_d: bool, // if k crossed above d -  typicaly in lower graph
    pub k_under_d: bool, // if k crossed under d typicaly in higher graph
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stoch {
    max: Maximum,
    min: Minimum,
    ema_k: EMA,
    ema_d: EMA,
    cross: SimpleCross,
}

// Note: the logic of functionality is taken from tradingview's "Stochastic" indicator.
//  use smooth_k 1 for no effect of EMA
impl Stoch {
    pub fn new(period_k: usize, smooth_k: usize, period_d: usize) -> TAResult<Self> {
        match period_k {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                max: Maximum::new(period_k)?,
                min: Minimum::new(period_k)?,
                ema_k: EMA::new(smooth_k)?,
                ema_d: EMA::new(period_d)?,
                cross: SimpleCross::new(),
            }),
        }
    }

    pub fn next(&mut self, next_val: f64) -> StochRes {
        let min = self.min.next(next_val);
        let max = self.max.next(next_val);

        let stoch = if max == min {
            50.
        } else {
            100. * (next_val - min) / (max - min)
        };

        let ema_k = self.ema_k.next(stoch);
        let ema_d = self.ema_d.next(stoch);
        let cross = self.cross.next_v2(ema_k, ema_d);

        StochRes {
            main_k: ema_k,
            smooth_d: ema_d,
            k_above_d: cross.crossed_above,
            k_under_d: cross.crossed_under,
        }
    }
}

impl Default for Stoch {
    fn default() -> Self {
        Self::new(14, 1, 3).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Stoch::new(0, 1, 3).is_err());
        assert!(Stoch::new(1, 1, 3).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _s = Stoch::new(3, 1, 3).unwrap();
    }

    #[test]
    fn test_default() {
        Stoch::default();
    }
}

use super::*;
use serde::{Deserialize, Serialize};

// https://www.investopedia.com/terms/s/stochrsi.asp

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StochRSI {
    rsi: RSI,
    stoch: Stoch,
}

impl StochRSI {
    pub fn new(period_k: usize, smooth_k: usize, period_d: usize) -> TAResult<Self> {
        match period_k {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                rsi: RSI::new(period_k)?,
                stoch: Stoch::new(period_k, smooth_k, period_d)?,
            }),
        }
    }

    pub fn next(&mut self, next_val: f64) -> StochRes {
        let rsi = self.rsi.next(next_val);
        let stoch_rsi = self.stoch.next(rsi);

        stoch_rsi
    }
}

impl Default for StochRSI {
    fn default() -> Self {
        Self::new(14, 1, 3).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(StochRSI::new(0, 1, 3).is_err());
        assert!(StochRSI::new(1, 1, 3).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _s = StochRSI::new(3, 1, 3).unwrap();
    }

    #[test]
    fn test_default() {
        StochRSI::default();
    }
}

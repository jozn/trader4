use serde::{Deserialize, Serialize};

use super::*;

// https://www.investopedia.com/terms/s/stochrsi.asp
// Stochastic itself

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stoch {
    max: Maximum,
    min: Minimum,
}

impl Stoch {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                max: Maximum::new(period)?,
                min: Minimum::new(period)?,
            }),
        }
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        let min = self.min.next(next_val);
        let max = self.max.next(next_val);

        if max == min {
            50.
        } else {
            100. * (next_val - min) / (max - min)
        }
    }
}

impl Default for Stoch {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Stoch::new(0).is_err());
        assert!(Stoch::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _s = Stoch::new(3).unwrap();
    }

    #[test]
    fn test_default() {
        Stoch::default();
    }
}

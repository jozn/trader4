use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

pub type BB = BollingerBand;
pub type BBOut = BollingerBandOut;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerBand {
    multi: f64,
    ma: SMA,
    std_div: StandardDeviation,
    is_new: bool,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct BollingerBandOut {
    pub high_band: f64,
    pub low_band: f64,
    pub ma_band: f64,
}

impl BollingerBand {
    pub fn new(period: usize, multi: f64) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                multi,
                ma: SMA::new(period).unwrap(),
                std_div: StandardDeviation::new(period).unwrap(),
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> BollingerBandOut {
        let tp = (candle.high() + candle.low() + candle.close()) / 3.0;
        self._next_(tp as f64)
    }

    pub fn _next_(&mut self, price: f64) -> BollingerBandOut {
        let ma = self.ma.next(price);
        let sd = self.std_div.next(price);

        BollingerBandOut {
            high_band: ma + self.multi * sd,
            low_band: ma - self.multi * sd,
            ma_band: ma,
        }
    }
}

impl Default for BollingerBand {
    fn default() -> Self {
        Self::new(20, 2.).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(BollingerBand::new(0).is_err());
        assert!(BollingerBand::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _cci = BollingerBand::new(3).unwrap();
    }

    #[test]
    fn test_default() {
        BollingerBand::default();
    }
}

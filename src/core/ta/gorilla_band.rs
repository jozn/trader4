use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

// This indicator sucks seems Boillnger Bands is better

pub type GB = GorillaBand;
pub type GBOut = GorillaBandOut;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GorillaBand {
    multi: f64,
    ma: SMA,
    dc: DC,
    std_div: StandardDeviation,
    is_new: bool,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct GorillaBandOut {
    pub high_band: f64,
    pub low_band: f64,
    pub ma_band: f64,
}

impl GorillaBand {
    pub fn new(period: usize, multi: f64) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                multi,
                ma: SMA::new(period).unwrap(),
                dc: DC::new(period / 2).unwrap(),
                std_div: StandardDeviation::new(period).unwrap(),
                is_new: true,
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> GorillaBandOut {
        let price_tp = (candle.high() + candle.low() + candle.close()) / 3.0;

        let ma = self.ma.next(price_tp);
        let sd = self.std_div.next(price_tp);
        let sd = self.std_div.next(price_tp);

        let dc_out = self.dc.next(&candle);

        let high_band = (ma + dc_out.high) / 2. - self.multi * sd;
        let low_band = (ma + dc_out.low) / 2. + self.multi * sd;

        // self.multi = 1.;

        // let high_band = dc_out.high - self.multi * sd;
        // let low_band = dc_out.low + self.multi * sd;

        let high_band = dc_out.high - self.multi * sd;
        let low_band = dc_out.low + self.multi * sd;

        GorillaBandOut {
            high_band,
            low_band,
            ma_band: ma,
        }
    }
}

impl Default for GorillaBand {
    fn default() -> Self {
        Self::new(20, 2.).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(GorillaBand::new(0, 2.).is_err());
        assert!(GorillaBand::new(12, 2.).is_ok());
    }

    #[test]
    fn test_next() {}

    #[test]
    fn test_default() {
        GorillaBand::default();
    }
}
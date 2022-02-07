use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

// Directional Movement MACD

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMMD {
    is_new: bool,
    dmi: DMI,
    macd: MACD,
    _cross: SimpleCross,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DMMDOutput {
    pub diff: f64,
    pub ma_fast: f64,
    pub ma_slow: f64,
    pub histogram: f64,
    pub color: f64,
}

impl DMMD {
    pub fn new(period: usize, adx_smooth: usize) -> TAResult<Self> {
        if period == 0 || adx_smooth == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                is_new: true,
                dmi: DMI::new(period, 2).unwrap(),
                macd: MACD::new(12, 26, 3).unwrap(),
                _cross: Default::default(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> DMMDOutput {
        if self.is_new {
            self.is_new = false;

            return DMMDOutput::default();
        }
        let dmi_out = self.dmi.next(&candle);
        // let diff = dmi_out.plus - dmi_out.minus;
        let diff = dmi_out.dmx;
        let macd_out = self.macd.next(diff);

        DMMDOutput {
            diff,
            ma_fast: macd_out.macd,
            ma_slow: macd_out.signal,
            histogram: macd_out.histogram,
            color: macd_out.color,
        }
    }
}

impl Default for DMMD {
    fn default() -> Self {
        Self::new(14, 14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MACDOutput;
    use super::*;
}

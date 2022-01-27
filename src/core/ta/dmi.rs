use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

// Average Directional Movement
// https://www.investopedia.com/terms/d/dmi.asp
// https://www.investopedia.com/terms/a/adx.asp

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMI {
    is_new: bool,
    pre_high: f64,
    pre_low: f64,
    tr: TrueRange,
    tr_ma: EMA,
    plus_ma: EMA,
    minus_ma: EMA,
    adx_ma: EMA,
    cross: SimpleCross,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DMIOutput {
    pub adx: f64,
    pub plus: f64,
    pub minus: f64,
    pub plus_above: bool, // true when plus crossed above the minus line - bullish
    pub plus_under: bool, // true when plus crossed under the minus line - bearish
}

impl DMI {
    pub fn new(dm_len: usize, adx_smooth: usize) -> TAResult<Self> {
        if dm_len == 0 || adx_smooth == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                is_new: true,
                pre_high: 0.0,
                pre_low: 0.0,
                tr: Default::default(),
                tr_ma: EMA::new_alpha(dm_len, 1.)?,
                plus_ma: EMA::new_alpha(dm_len, 1.)?,
                minus_ma: EMA::new_alpha(dm_len, 1.)?,
                adx_ma: EMA::new_alpha(adx_smooth, 1.)?,
                cross: Default::default(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> DMIOutput {
        if self.is_new {
            self.pre_high = candle.high();
            self.pre_low = candle.low();
            self.is_new = false;

            return DMIOutput::default();
        }

        let up = candle.high() - self.pre_high;
        let down = self.pre_low - candle.low();

        let plus_dm = if up > down && up > 0. { up } else { 0. };

        let minus_dm = if down > up && down > 0. { down } else { 0. };

        // this is ATR - todo chagne to  ATR iteslef
        let tr = self.tr.next(&candle);
        let trur = self.tr_ma.next(tr);

        let plus = 100. * self.plus_ma.next(plus_dm) / trur;
        let minus = 100. * self.minus_ma.next(minus_dm) / trur;

        let dx = 100. * (plus - minus).abs() / (plus + minus);
        let adx = self.adx_ma.next(dx);

        let cr = self.cross.next_v2(plus, minus);

        self.pre_high = candle.high();
        self.pre_low = candle.low();

        DMIOutput {
            adx,
            plus,
            minus,
            plus_above: cr.crossed_above,
            plus_under: cr.crossed_under,
        }
    }
}

impl Default for DMI {
    fn default() -> Self {
        Self::new(14, 14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MACDOutput;
    use super::*;
}

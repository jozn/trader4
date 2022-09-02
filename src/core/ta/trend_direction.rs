use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;

// Trend Direction like DMI

pub type TD = TrendDirection;
pub type TDOut = TrendDirectionOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDirection {
    is_new: bool,
    pre_high: f64, // previous
    pre_low: f64,
    atr: ATR,
    plus_ma: EMA,
    minus_ma: EMA,
    adx_ma: EMA,
    diff_ma: EMA,
    ma_mom: Momentum,
    mom_mom: Momentum,
    cross: SimpleCross,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrendDirectionOutput {
    pub plus: f64,
    pub minus: f64, // is negative always
    pub diff: f64,
    pub diff_ma: f64,
    pub ma_mom: f64,
    pub mom_mom: f64,
    pub adx: f64,
    pub dmx: f64,         // My me: Dim Move Index -- see below for formula
    pub plus_above: bool, // true when plus crossed above the minus line - bullish
    pub plus_under: bool, // true when plus crossed under the minus line - bearish
}

impl TrendDirection {
    pub fn new(period: usize, adx_smooth: usize) -> TAResult<Self> {
        if period == 0 || adx_smooth == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                is_new: true,
                pre_high: 0.0,
                pre_low: 0.0,
                atr: ATR::new(period)?,
                plus_ma: EMA::new_alpha(period, 1.)?,
                minus_ma: EMA::new_alpha(period, 1.)?,
                adx_ma: EMA::new_alpha(adx_smooth, 1.)?,
                diff_ma: EMA::new_alpha(adx_smooth, 1.)?,
                ma_mom: Momentum::new(5).unwrap(),
                mom_mom: Momentum::new(5).unwrap(),
                cross: Default::default(),
            })
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> TrendDirectionOutput {
        if self.is_new {
            self.pre_high = candle.high();
            self.pre_low = candle.low();
            self.atr.next(&candle);
            self.is_new = false;

            return TrendDirectionOutput::default();
        }

        let up = candle.high() - self.pre_high;
        let down = self.pre_low - candle.low();

        let plus_dm = if up > down && up > 0. { up } else { 0. };
        let minus_dm = if down > up && down > 0. { down } else { 0. };

        let _atr = self.atr.next(&candle);

        // let plus = 100. * self.plus_ma.next(plus_dm) / atr;
        let plus = 100. * self.plus_ma.next(plus_dm);
        // let minus = 100. * self.minus_ma.next(minus_dm) / atr;
        let minus = 100. * self.minus_ma.next(minus_dm);

        let sum = plus + minus;
        let sum = if sum != 0. { sum } else { 1. };

        // let dx = 100. * ((plus - minus).abs() / sum);
        let dx = 100. * ((plus - minus) / sum);
        let adx = self.adx_ma.next(dx);

        let dmx = 100. * (plus - minus) / sum;
        let dmx = (plus + minus) * (plus - minus) / sum;
        // let dmx =(plus - minus)/ sum;

        let diff = (plus - minus) * 1.; //2.;
        let diff_ma = self.diff_ma.next(diff);

        let cr = self.cross.next_v2(plus, minus);

        self.pre_high = candle.high();
        self.pre_low = candle.low();

        let ma_mom = self.ma_mom.next(diff_ma);
        let mom_mom = self.mom_mom.next(ma_mom);

        TrendDirectionOutput {
            adx,
            plus,
            minus: -minus,
            diff,
            dmx,
            plus_above: cr.crossed_above,
            plus_under: cr.crossed_under,
            diff_ma,
            ma_mom,
            mom_mom,
        }
    }
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::new(14, 14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;

// Donchain Channel Signal

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DCSRes {
    pub buy2: bool,
    pub sell2: bool,

    pub x_pip: f64,
    pub x_high: f64,
    pub x_low: f64,
    pub low_sig: bool, // go for bull
    pub up_sig: bool,  // go for short
    pub x_perc: f64,
    pub hh: bool,
    pub ll: bool,
    pub b_high: f64,
    pub b_low: f64,
    pub b_hh: bool,
    pub b_ll: bool,
    pub b_middle: f64,
    pub b_perc: f64,
    #[serde(skip)]
    pub price: f64,
    pub wight: f64, // todo skip
    pub sum: f64,
    pub ratio: f64,
    pub buy1: bool,
    pub sell1: bool,
    pub dir: f64,
    #[serde(skip)]
    pub vvv: VelRes,
    // #[serde(skip)]
    pub rsi: f64,
    pub rti: f64,
    pub rt_up: bool,
    pub rt_down: bool,
    #[serde(skip)]
    pub vel2: VelRes2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCS {
    period: usize,
    max: Maximum,
    max_big: Maximum,
    min: Minimum,
    min_big: Minimum,
    vel: Vel,
    vel2: Vel2,
    v_cross: SimpleCross,
    rsi: RSI,
    rti: RTI,
    cross_high: SimpleCross,
    cross_low: SimpleCross,
    past: VecDeque<DCSRes>,
}

// Note: the logic of functionality is taken from tradingview's "DCSastic" indicator.
//  use smooth_k 1 for no effect of EMA
impl DCS {
    pub fn new(period: usize) -> TAResult<Self> {
        let big_period = period * 3;
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                max: Maximum::new(period)?,
                max_big: Maximum::new(big_period)?,
                min: Minimum::new(period)?,
                min_big: Minimum::new(big_period)?,
                vel: Vel::new(5)?,
                vel2: Vel2::new(period * 2)?,
                v_cross: SimpleCross::new(),
                rsi: RSI::new(14)?,
                rti: RTI::new(big_period * 3)?,
                cross_high: SimpleCross::new(),
                cross_low: SimpleCross::new(),
                past: VecDeque::new(),
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> DCSRes {
        let high = self.max.next(candle.high());
        let low = self.min.next(candle.low());

        let big_high = self.max_big.next(candle.high());
        let big_low = self.min_big.next(candle.low());

        let price = candle.close();

        let perc = (price - low) / (high - low);
        let b_perc = (price - big_low) / (big_high - big_low);
        let cut_off = (high - low) / 5.; // 0.2
        let high_cross_line = high - cut_off;
        let low_cross_line = low + cut_off;

        let high_cross_res = self.cross_high.next_v2(price, high_cross_line);
        let low_cross_res = self.cross_low.next_v2(price, low_cross_line);

        let mut now = DCSRes {
            x_pip: (high - low) * 10_000.,
            x_high: high,
            x_low: low,
            up_sig: high_cross_res.crossed_under,
            low_sig: low_cross_res.crossed_above,
            x_perc: perc,
            b_high: big_high,
            b_low: big_low,
            b_middle: (big_high + big_low) / 2.,
            b_perc,
            price: price,
            rsi: self.rsi.next(perc),
            ..Default::default()
        };

        let mut old_dir = 0.;
        let pre_opt = self.past.front();
        match pre_opt {
            None => {}
            Some(pre) => {
                if now.x_high > pre.x_high {
                    now.hh = true;
                }
                if now.x_low < pre.x_low {
                    now.ll = true;
                }

                // big
                if now.b_high > pre.b_high {
                    now.b_hh = true;
                }
                if now.b_low < pre.b_low {
                    now.b_ll = true;
                }
                old_dir = pre.dir;
            }
        }

        if self.past.len() == self.period {
            self.past.pop_back();
        }

        let bars = self.past.len() as f64;
        let mut trend = 0.;
        let mut sum = 0.;
        for f in self.past.iter() {
            trend += f.price - f.b_low;
            sum += f.b_high - f.b_low;
        }
        now.wight = trend / bars;
        now.sum = sum / bars;
        now.ratio = now.wight / now.sum;

        let ratio = now.ratio;

        let valid_pip = if now.x_pip >= 12.0 { true } else { false };

        if ratio < 0.75 && now.up_sig && valid_pip {
            now.sell1 = true;
        }
        if ratio > 0.25 && now.low_sig && valid_pip {
            now.buy1 = true;
        }

        now.vvv = self.vel.next(now.b_middle);
        if now.vvv.avg_vel_pip > 0. {
            now.dir = 1.
        } else if now.vvv.avg_vel_pip < 0. {
            now.dir = -1.
        } else {
            // 0.
            now.dir = old_dir;
        }

        if now.dir == 0. {
            // now.dir = old_dir;
        }

        let rti = self.rti.next(big_high, big_low);
        now.rti = rti;

        now.vel2 = self.vel2.next(now.rti);
        let cr = self.v_cross.next_v2(now.rti, now.vel2.v2_ma);
        now.rt_up = cr.crossed_under;
        now.rt_down = cr.crossed_above;

        let rti_up = if rti > now.vel2.v2_ma { true } else { false };

        // buy2 sell2 signals
        if !rti_up && rti < 70. && now.up_sig && valid_pip {
            now.sell2 = true;
        }
        // if rti_up && rti > 30. && now.low_sig && valid_pip {
        if rti_up && now.low_sig && valid_pip {
            // if rti_up  && now.low_sig {
            now.buy2 = true;
        }

        self.past.push_front(now.clone());
        now
    }
}

impl Default for DCS {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(DCS::new(0).is_err());
        assert!(DCS::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut _s = DCS::new(3).unwrap();
    }

    #[test]
    fn test_default() {
        DCS::default();
    }
}

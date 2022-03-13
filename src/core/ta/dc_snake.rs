use crate::base::{SimpleCross, OHLCV};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;

// Donchain Channel Snake - a combination of DC and boilnger bands

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DCSnakeRes {
    pub x_pip: f64,
    pub x_high: f64,
    pub x_low: f64,
    pub oversold_line: f64, // high - value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCSnake {
    period: usize,
    dc: DC,
    dc_big: DC,
    atr: ATR,
    bb: BollingerBand,
    vel: Vel,
    vel2: Vel2,
    cross_high: SimpleCross,
    cross_low: SimpleCross,
    past: VecDeque<DCSnakeRes>,
}

impl DCSnake {
    pub fn new(period: usize) -> TAResult<Self> {
        let big_period = period * 3;
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                dc: DC::new(period)?,
                dc_big: DC::new(big_period)?,
                atr: ATR::new(big_period)?,
                bb: BollingerBand::new(big_period, 2.)?,
                vel: Vel::new(5)?,
                vel2: Vel2::new(period)?,
                cross_high: Default::default(),
                cross_low: Default::default(),
                past: VecDeque::new(),
            }),
        }
    }

    pub fn next(&mut self, candle: impl OHLCV) -> DCSnakeRes {
        let dcr = self.dc.next(&candle);
        let bbr = self.bb.next(&candle);
        let atr = self.atr.next(&candle);

        DCSnakeRes {
            x_pip: (dcr.high - dcr.low) * 10_000.,
            x_high: dcr.high,
            x_low: dcr.low,
            // oversold_line: dcr.high - bbr.sd * 2.,
            oversold_line: dcr.high - atr * 2.5,
        }
    }
}

impl Default for DCSnake {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

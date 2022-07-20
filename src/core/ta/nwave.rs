use super::*;
use crate::bar::Bar;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::VecDeque;

/// UNIMPLEMENTED

// NewWave: cable of embedding in methods like the rest of Indicators. But unlike
//  others it's not a frame based, aka not every Bar has a result, it's historic
//  data that matters the most.
// ATR based so no need for expelict percentage like the Wave.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NWave {
    depth: usize,
    backstep: usize,
    deviation: f64, // percent
    cnt: usize,
    bars: VecDeque<Bar>, // 0: recent bar
    last: NWaveRes,
    atr: ATR,
    pub wave_ress: Vec<NWaveRes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NWaveRes {
    pub time: i64,
    pub bar_id: u64,
    pub price: f64,
    pub pos: NPointPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NPointPos {
    Up,
    Down,
}

fn calc_dev(base_price: f64, price: f64) -> f64 {
    100. * (price - base_price) / base_price
}

impl NWave {
    pub fn new(depth: usize, backstep: usize, deviation: f64) -> TAResult<Self> {
        if depth <= backstep || depth <= 2 || backstep <= 1 {
            return Err(TAErr::WrongArgs);
        }
        Ok(Self {
            depth,
            backstep,
            deviation,
            cnt: 0,
            bars: Default::default(),
            last: Default::default(),
            atr: ATR::new(6).unwrap(),
            wave_ress: vec![],
        })
    }

    pub fn next(&mut self, bar: &Bar) {}
}

impl NPointPos {
    fn is_up(&self) -> bool {
        self == &NPointPos::Up
    }
    fn is_down(&self) -> bool {
        !self.is_up()
    }
}
impl Default for NPointPos {
    fn default() -> Self {
        NPointPos::Down
    }
}

use crate::bar::Bar;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::VecDeque;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave {
    depth: usize,
    backstep: usize,
    deviation: f64, // percent
    cnt: usize,
    bars: VecDeque<Bar>, // 0: recent bar
    last: WaveRes,
    pub wave_ress: Vec<WaveRes>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WaveRes {
    pub time: i64,
    pub bar_id: u64,
    pub price: f64,
    pub post: PointPos, // true: up price, false: down price
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PointPos {
    Up,
    Down,
}

fn calc_dev(base_price: f64, price: f64) -> f64 {
    100. * (price - base_price) / base_price
}

impl Wave {
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
            wave_ress: vec![],
        })
    }

    pub fn next(&mut self, bar: &Bar) {
        let depth = self.depth;
        let backstep = self.backstep;
        let deviation = self.deviation;

        self.bars.push_front(bar.clone());
        if self.wave_ress.is_empty() {
            let p = WaveRes {
                time: bar.open_time,
                bar_id: 0,
                price: bar.low,
                post: PointPos::Down,
            };
            self.last = p.clone();
            self.wave_ress.push(p);
            return;
        }
        // let last = self.wave_ress.last().clone().unwrap();
        let last = &self.last;

        if last.post.is_down() {
            if bar.low < last.price {
                let p = WaveRes {
                    time: bar.open_time,
                    bar_id: 0,
                    price: bar.low,
                    post: PointPos::Down,
                };
                self.last = p;
                // self.wave_ress.remove(0);
                // self.wave_ress.push_front(p);
                return;
            }

            let chang = calc_dev(last.price, bar.high);
            if chang > deviation {
                self.wave_ress.push(self.last.clone());
                let p = WaveRes {
                    time: bar.open_time,
                    bar_id: 0,
                    price: bar.high,
                    post: PointPos::Up,
                };
                self.last = p;
                return;
            }
        } else {
            // up price
            if bar.high > last.price {
                let p = WaveRes {
                    time: bar.open_time,
                    bar_id: 0,
                    price: bar.high,
                    post: PointPos::Up,
                };
                self.last = p;
                return;
            }

            let chang = calc_dev(last.price, bar.low);
            if chang < -deviation {
                self.wave_ress.push(self.last.clone());
                let p = WaveRes {
                    time: bar.open_time,
                    bar_id: 0,
                    price: bar.low,
                    post: PointPos::Down,
                };
                self.last = p;
                return;
            }
        }
    }
}

impl PointPos {
    fn is_up(&self) -> bool {
        self == &PointPos::Up
    }
    fn is_down(&self) -> bool {
        !self.is_up()
    }
}
impl Default for PointPos {
    fn default() -> Self {
        PointPos::Down
    }
}

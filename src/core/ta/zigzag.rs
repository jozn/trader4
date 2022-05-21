use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

use super::*;
use crate::bar::Bar;
use crate::base::*;
use std::collections::VecDeque;

// ZigZag should not be embeded like other indicators
// it does not works correctly use wave

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZigZag {
    depth: usize,
    backstep: usize,
    deviation: f64, // percent
    cnt: usize,
    dc1: DC,
    dc2: DC,
    bars: VecDeque<Bar>, // 0: recent bar
    last: Option<ZigZagRes>,
    pub store: Vec<ZigZagRes>,

    last_ema: f64,
    store_ema: VecDeque<f64>,
    buff: VecDeque<f64>, // A buffer to avoid allocating in each call
    is_new: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZigZagRes {
    pub time: i64,
    pub bar_id: u64,
    pub price: f64,
    pub is_up_price: bool, // true: up price, false: down price
}

fn calc_dev(base_price: f64, price: f64) -> f64 {
    100. * (price - base_price) / base_price
}

impl ZigZag {
    pub fn new(depth: usize, backstep: usize, deviation: f64) -> TAResult<Self> {
        if depth <= backstep || depth <= 2 || backstep <= 1 {
            return Err(TAErr::WrongArgs);
        }
        Ok(Self {
            depth,
            backstep,
            deviation,
            cnt: 0,
            dc1: DC::new(depth).unwrap(),
            dc2: Default::default(),
            bars: Default::default(),
            last: None,
            store: vec![],
            // ema: EMA::new(ema_period).unwrap(),
            last_ema: 0.,
            store_ema: VecDeque::with_capacity(200),
            buff: VecDeque::with_capacity(200),
            is_new: true,
        })
    }

    pub fn next_futu(&mut self, bar: &Bar) {
        let (iH, pH) = self.pivot_high(bar.high);
        let (iL, pL) = self.pivot_low(bar.low);

        let lineLast = 0.;
        // let  labelLast = na;
        let iLast = 0;
        let pLast = 0.;
        let isHighLast = true; // otherwise the last pivot is a low pivot
        let linesCount = 0;
        // let float sumVol = 0
        // let float sumVolLast = 0
    }

    // pub fn pivot_high(&mut self, high: f64) -> Option<(i64,f64)> {
    pub fn pivot_high(&mut self, high: f64) -> (i64, f64) {
        let mut found = true;
        let mut size = 0;
        for bar in self.bars.iter() {
            if size >= self.backstep {
                break;
            }
            if bar.high > high {
                found = false;
            }
            size += 1;
        }

        for bar in self.bars.iter().skip(size) {
            if size >= self.depth {
                break;
            }
            if bar.high >= high {
                found = false;
            }
            size += 1;
        }

        let mid_bar = self.bars.get(self.backstep).unwrap();
        if found && self.cnt < self.depth {
            (mid_bar.open_time, high)
        } else {
            (0, 0.)
        }
        // if found && self.cnt < self.depth {
        //     Some((mid_bar.open_time, high))
        // }else {
        //     None
        // }
    }

    // pub fn pivot_low(&mut self, low: f64) -> Option<(i64, f64)> {
    pub fn pivot_low(&mut self, low: f64) -> (i64, f64) {
        let mut found = false;
        let mut size = 0;
        for bar in self.bars.iter() {
            if size >= self.backstep {
                break;
            }
            if bar.low < low {
                found = false;
            }
            size += 1;
        }

        for bar in self.bars.iter().skip(size) {
            if size >= self.backstep {
                break;
            }
            if bar.low <= low {
                found = false;
            }
            size += 1;
        }

        let mid_bar = self.bars.get(self.backstep).unwrap();
        if found && self.cnt < self.depth {
            (mid_bar.open_time, low)
        } else {
            (0, 0.)
        }
        // let mid_bar = self.bars.get(self.backstep).unwrap();
        // if found && self.cnt < self.depth {
        //     Some((mid_bar.open_time, low))
        // }else {
        //     None
        // }
    }

    pub fn next_v2(&mut self, bar: &Bar) -> Option<ZigZagRes> {
        let dcr = self.dc1.next(&bar);
        self.cnt += 1;
        if self.is_new {
            self.is_new = false;
            let last = ZigZagRes {
                time: bar.open_time,
                bar_id: 0,
                price: bar.low,
                is_up_price: false,
            };
            self.store.push(last.clone());
            self.last = Some(last.clone());
            // return Some(last)
        }
        let mut new_price = 0.;
        let mut change = false;
        let last = self.last.clone().unwrap();
        if last.is_up_price {
            // last is hih pint
            // look for down
            let low = dcr.low;

            // update lower
            if bar.high > last.price {
                change = true;
                new_price = bar.high;
            }
            if 100. * (bar.low - last.price).abs() / last.price > self.deviation {
                let last = ZigZagRes {
                    time: bar.open_time,
                    bar_id: 0,
                    price: bar.low,
                    is_up_price: false,
                };
                self.store.push(last.clone());
            }
        } else {
            // last is low point

            // update lower
            if bar.low < last.price {
                change = true;
                new_price = bar.low;
            }
            if 100. * (bar.high - last.price).abs() / last.price > self.deviation {
                let last = ZigZagRes {
                    time: bar.open_time,
                    bar_id: 0,
                    price: bar.high,
                    is_up_price: true,
                };
                self.store.push(last.clone());
            }
        }
        if change {
            let last = ZigZagRes {
                time: bar.open_time,
                bar_id: 0,
                price: new_price,
                is_up_price: true,
            };
            self.store.pop();
            self.store.push(last.clone());
        }

        None
    }

    // pub fn next(&mut self, candle: impl OHLCV) -> ZigZagRes {
    pub fn next(&mut self, bar: &Bar) -> Option<ZigZagRes> {
        let dcr = self.dc1.next(&bar);
        if self.is_new {
            self.is_new = false;
            let last = ZigZagRes {
                time: bar.open_time,
                bar_id: 0,
                price: bar.low,
                is_up_price: false,
            };
            self.last = Some(last.clone());
            // return Some(last)
        }
        let mut new_price = 0.;
        let mut change = false;
        // let chigh = bar.h
        let last = self.last.clone().unwrap();
        if last.is_up_price {
            // look for down
            let low = dcr.low;
        } else {
            if bar.low < last.price {
                change = true;
                new_price = bar.low;
            }
            // if bar.high - last.price{
            //
            // }
        }

        // crappy impl
        self.cnt += 1;
        if self.cnt % 12 == 1 {
            let mut high = true; // looke for
            if self.store.len() > 0 {
                let l = self.store.last().unwrap();
                if l.is_up_price {
                    // swtich
                    high = false;
                }
            }
            let mut price = dcr.low;
            if high {
                price = dcr.high;
            }
            let r = ZigZagRes {
                time: bar.open_time,
                bar_id: 0,
                price,
                is_up_price: high,
            };
            self.store.push(r.clone());
            return Some(r);
        }

        None
    }
}

impl Default for ZigZag {
    fn default() -> Self {
        Self::new(12, 6, 1.).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

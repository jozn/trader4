use std::borrow::Borrow;
use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;
use std::collections::VecDeque;
use crate::bar::Bar;

// ZigZag should not be embeded like other indicators

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZigZag {
    depth: usize,
    backstep: usize,
    deviation: f64, // percent
    cnt: usize,
    dc: DC,
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

impl ZigZag {
    pub fn new( depth: usize, backstep: usize, deviation: f64,) -> TAResult<Self> {
        if depth <= backstep {
            return Err(TAErr::WrongArgs)
        }
        Ok(Self {
            depth,
            backstep,
            deviation,
            cnt:0,
            dc: DC::new(depth).unwrap(),
            last:None,
            store: vec![],
            // ema: EMA::new(ema_period).unwrap(),
            last_ema: 0.,
            store_ema: VecDeque::with_capacity(200),
            buff: VecDeque::with_capacity(200),
            is_new: true,
        })
    }

    pub fn next_v2(&mut self, bar: &Bar) -> Option<ZigZagRes> {
        let dcr = self.dc.next(&bar);
        self.cnt +=1;
        if self.is_new {
            self.is_new =false;
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
            if 100. * (bar.low - last.price).abs()/ last.price > self.deviation {
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
            if 100. * (bar.high - last.price).abs()/ last.price > self.deviation {
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
        let dcr = self.dc.next(&bar);
        if self.is_new {
            self.is_new =false;
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
        self.cnt +=1;
        if self.cnt % 12 == 1 {
            let mut high = true; // looke for
            if self.store.len() > 0 {
                let l = self.store.last().unwrap();
                if l.is_up_price { // swtich
                    high = false;
                }
            }
            let mut price = dcr.low;
            if high {
                price  = dcr.high;
            }
            let r = ZigZagRes {
                time: bar.open_time,
                bar_id: 0,
                price,
                is_up_price: high,
            };
            self.store.push(r.clone());
            return Some(r)
        }

        None
    }
}

impl Default for ZigZag {
    fn default() -> Self {
        Self::new(12,6,1.).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

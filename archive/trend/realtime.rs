// Note this moudle focus on ticking rather than only one kline timeframe type.

use crate::*;
// use super::super::loader::loader_csv;
// use super::super::loader::loader_csv::*;
use crate::base::OHLCV;
use crate::candle::{CandleSeriesTA, TimeSerVec};
use crate::offline::{to_csv_out, to_csv_out_old};
use serde::{Deserialize, Serialize};
use Dir::*;

pub fn trend_play1() {
    let arr = loader::loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    println!("{:#?}", ts.small);
}

////////////////////////////////////
pub fn trend_play2() {
    let arr = loader::loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    let d = diff(ts);

    // let out = to_csv_out(&d);
    // println!("{}", out);

    // ts.print_ticking();

    // println!("{:}", ts);
}

fn diff(arr: CandleSeriesTA) -> Vec<Trend> {
    // let klines = &arr.small.klines_ta;
    let klines = &arr.ticking;
    let first = klines.first().unwrap();

    let mut res = vec![];
    let mut last_kline = &first.medium.ta1;
    for k in klines.iter() {
        let kt = &k.medium.ta1;
        let t = Trend {
            price: k.medium.kline.hlc3().round() as u64,
            line1: to_dir(kt.t_hull1 - last_kline.t_hull1),
            line2: to_dir(kt.t_hull2 - last_kline.t_hull2),
            line3: to_dir(kt.t_hull3 - last_kline.t_hull3),
            hull1: kt.t_hull1,
            hull2: kt.t_hull2,
            hull3: kt.t_hull3,
        };
        last_kline = kt;
        res.push(t);
    }

    println!("{:#?}", res);
    res
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Dir {
    UP,
    DOWN,
    ZERO,
}
impl Default for Dir {
    fn default() -> Self {
        Dir::ZERO
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Trend {
    pub price: u64,
    pub line1: Dir,
    pub line2: Dir,
    pub line3: Dir,
    pub hull1: f64,
    pub hull2: f64,
    pub hull3: f64,
}

fn to_dir(v: f64) -> Dir {
    if v == 0. {
        ZERO
    } else if v > 0. {
        UP
    } else {
        DOWN
    }
}

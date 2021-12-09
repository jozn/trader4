pub mod realtime;

use crate::offline::loader::*;
use crate::offline::*;
use crate::*;
// use super::loader::loader_csv;
// use super::loader::loader_csv::*;
// use super::offline_old;
use crate::base::OHLCV;
use crate::candle::{CandleSeriesTA, TimeSerVec};
use crate::offline::{to_csv_out, to_csv_out_old};
use crate::trend::Dir::{DOWN, UP, ZERO};
use serde::{Deserialize, Serialize};

pub fn trend_play3_tang() {
    use crate::candle::*;
    let arr = loader_csv::_load(100_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = crate::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = crate::candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            price_raw: v.ask_price,
            price_multi: 100_000.0,
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

    let d = diff_tang(ts);

    let out = to_csv_out(&d);
    println!("{}", out);

    // ts.print_ticking();

    // println!("{:}", ts);
}

pub fn trend_play1() {
    use crate::candle::*;
    let arr = loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = crate::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = crate::candle::Tick {
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

    let out = to_csv_out(&d);
    println!("{}", out);

    // ts.print_ticking();

    // println!("{:}", ts);
}

fn diff_tang(arr: CandleSeriesTA) -> Vec<TangDiff> {
    let klines = &arr.small.klines_ta;
    let first = klines.first().unwrap();

    let mut res = vec![];
    let mut last_kline = &first.ta1;
    for (i, k) in klines.iter().enumerate() {
        let kt = &k.ta1;
        let t = TangDiff {
            price: k.kline.hlc3().round() as u64,
            ema: round2(k.ta1.ema10) as u64,
            diff: round2((k.ta1.ema10 - last_kline.ema10)),
        };
        last_kline = kt;
        if i > 50 {
            res.push(t);
        }
    }

    // println!("{:#?}", res);
    res
}

fn diff(arr: CandleSeriesTA) -> Vec<Trend> {
    let klines = &arr.small.klines_ta;
    let first = klines.first().unwrap();

    let mut res = vec![];
    let mut last_kline = &first.ta1;
    for k in klines.iter() {
        let kt = &k.ta1;
        let t = Trend {
            price: k.kline.hlc3().round() as u64,
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

    // println!("{:#?}", res);
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

////// For Tangent //////
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct TangDiff {
    pub price: u64,
    pub ema: u64,
    pub diff: f64,
}

fn round2(n: f64) -> f64 {
    ((n * 100.) as i64) as f64 / 100.
}

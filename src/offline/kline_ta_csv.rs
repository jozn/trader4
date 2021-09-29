use super::*;
use crate::candle::KlineTA;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CsvOut {
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub hlc3: f64,
    // TA
    pub ema: f64,
    pub hull: f64,
    pub roc: i64,
    pub mom: i64,
    pub cci: i64,
}

impl CsvOut {}

pub fn to_json_out(arr: &Vec<CsvOut>) -> String {
    let o = serde_json::to_string_pretty(arr).unwrap();
    o
}
pub fn to_csv_out(arr: &Vec<CsvOut>) -> String {
    let mut str_out = vec![];
    let mut wtr = csv::Writer::from_writer(&mut str_out);

    for v in arr {
        wtr.serialize(v);
    }

    let s = wtr.flush();
    drop(wtr);

    let s = String::from_utf8(str_out).unwrap();

    format!("{:}", s)
}

pub fn kline_to_csv_out(kt: &KlineTA) -> CsvOut {
    let k = &kt.kline;
    let s = kt;
    let t = &kt.ta1;
    let open_time = NaiveDateTime::from_timestamp(k.open_time as i64 / 1000, 0);
    let ots = open_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let o = CsvOut {
        time: ots,
        open: num5(k.open),
        high: num5(k.high),
        low: num5(k.low),
        close: num5(k.close),
        volume: num5(k.volume),
        hlc3: num5((k.high + k.low + k.close) / 3.0),
        ema: num5(t.ema10),
        hull: num5(t.hull),
        mom: num5(t.mom * 1.).round() as i64, // from 1 million part
        roc: num5(t.roc * 10000.).round() as i64, // from 1 million part
        // roc: t.roc ,
        cci: num5(t.cci * 10.).round() as i64,
    };

    o
}

fn round5(v: f64) -> f64 {
    let pre = 100_000.;
    ((v * pre) as i64) as f64 / pre
}

fn num5(v: f64) -> f64 {
    let n = v.abs() as u64;
    let dig = if n >= 10_000 {
        0
    } else if n >= 1000 {
        1
    } else if n >= 100 {
        2
    } else if n >= 10 {
        3
    } else if n >= 1 {
        4
    } else {
        5
    };

    let dig_per = 10_f64.powf(dig as f64);

    ((v * dig_per) as i64) as f64 / dig_per
}

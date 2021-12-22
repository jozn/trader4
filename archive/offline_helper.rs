use super::*;
use crate::candle::KlineTA;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub fn to_json_out<T: Serialize>(arr: &Vec<T>) -> String {
    let o = serde_json::to_string_pretty(arr).unwrap();
    o
}

pub fn to_csv_out<T: Serialize>(arr: &Vec<T>) -> String {
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

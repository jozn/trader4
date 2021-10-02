use super::*;
use crate::base::OHLCV;
use crate::candle::{KlineTA, KlineTATick};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Tok = Tick of realtime anlayse , but to not conflict with Forex tick we use Tok as 'Tick Tok'

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokCsvOut {
    pub time: String,
    pub high_small: f64,
    pub high_medium: f64,
    pub high_big: f64,
    pub low_small: f64,
    pub low_medium: f64,
    pub low_big: f64,
    pub hlc3_small: f64,
    pub hlc3_medium: f64,
    pub hlc3_big: f64,
    // TA
    pub ema_small: f64,
    pub ema_medium: f64,
    pub ema_big: f64,
    pub hull_small: f64,
    pub hull_medium: f64,
    pub hull_big: f64,
    pub roc_small: i64,
    pub roc_medium: i64,
    pub roc_big: i64,
    pub mom_small: i64,
    pub mom_medium: i64,
    pub mom_big: i64,
}

pub fn tok_to_json_out(arr: &Vec<TokCsvOut>) -> String {
    let o = serde_json::to_string_pretty(arr).unwrap();
    o
}

pub fn tok_to_csv_out(arr: &Vec<TokCsvOut>) -> String {
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

pub fn tok_kline_to_csv_out(kt: &KlineTATick) -> TokCsvOut {
    let sk = &kt.small.kline;
    let st = &kt.small.ta1;

    let mk = &kt.medium.kline;
    let mt = &kt.medium.ta1;

    let bk = &kt.big.kline;
    let bt = &kt.big.ta1;

    let open_time = NaiveDateTime::from_timestamp(sk.open_time as i64 / 1000, 0);
    let ots = open_time.format("%Y-%m-%d %H:%M:%S").to_string();

    TokCsvOut {
        time: ots,
        high_small: sk.high,
        high_medium: mk.high,
        high_big: bk.high,
        low_small: sk.low,
        low_medium: mk.low,
        low_big: bk.low,
        hlc3_small: sk.hlc3(),
        hlc3_medium: mk.hlc3(),
        hlc3_big: bk.hlc3(),
        // TA
        ema_small: num5(st.ema10),
        ema_medium: num5(mt.ema10),
        ema_big: num5(bt.ema10),
        hull_small: num5(st.hull),
        hull_medium: num5(mt.hull),
        hull_big: num5(bt.hull),
        roc_small: num5i64(st.roc),
        roc_medium: num5i64(mt.roc),
        roc_big: num5i64(bt.roc),
        mom_small: num5i64(st.mom),
        mom_medium: num5i64(mt.mom),
        mom_big: num5i64(bt.mom),
    }
}

use chrono::prelude::*;
use trader3;
use trader3::candle::{CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, TimeSerVec};
use trader3::collector;
use trader3::configs::assets::Pair;
use trader3::offline::num5;

pub fn main() {
    let pairs = trader3::configs::assets::get_all_symbols();

    for pair in pairs {
        for i in 1..=53 {
            let path = format!("/mnt/c/me/data/{:?}/{}.tsv", pair, i);
            if std::path::Path::new(&path).exists() {
                let cfg = CandleConfig {
                    small_tick: 40,
                    medium_tick: 10,
                    big_tick: 100,
                    vel1_period: 30,
                };
                let mut candle = CandleSeriesTA::new(&cfg);

                let ticks = trader3::collector::loader::load_rows(&path);
                let mut arr = TimeSerVec::new();

                for t in ticks {
                    arr.push(t.to_tick());
                    if arr.len() == cfg.small_tick as usize {
                        candle.add_ticks(arr.clone());
                        arr.clear();
                    }
                }

                write_output(&candle.small, &pair, i, "small");
                write_output(&candle.medium, &pair, i, "medium");
                write_output(&candle.big, &pair, i, "big");
            }
        }
    }
}

fn write_output(khf: &KlineHolderFrameTA, pair: &Pair, week_id: i64, time_frame_str: &str) {
    let mut vec_candles = vec![];
    for k in khf.klines_ta.iter() {
        vec_candles.push(kline_to_kline_out(&k.kline));
    }

    let s = trader3::core::helper::to_csv_out(&vec_candles, true);
    // let s  = trader3::offline::kline_ta_csv::to_csv_out(&vec_candles);

    // Write to file
    let dir = format!("/mnt/c/me/data_trans/{:?}", pair);
    let out_file_path = format!(
        "/mnt/c/me/data_trans/{:?}/{}_{}.tsv",
        pair, week_id, time_frame_str
    );

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, s);
    println!("{}", &out_file_path);
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct KlineOut {
    // #[serde(flatten)]
    // pub kline: Kline,
    pub open_time: u64, // in mill seconds
    // pub close_time: u64,
    pub bucket: u64,
    pub tick_count: u32,
    pub kline_num: i32, // -1: from trades sums >0 sums of klines
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pip_dif_max: f64, // high - low
    pub pip_dif_oc: f64,  // close - open
    // pub volume: f64,
    pub open_time_str: String,
    pub duration: String,
}

fn kline_to_kline_out(k: &Kline) -> KlineOut {
    let open_time = NaiveDateTime::from_timestamp(k.open_time as i64, 0);
    let ots = open_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let o = KlineOut {
        // kline: k.clone(),
        open_time: k.open_time,
        bucket: k.bucket,
        tick_count: k.tick_count,
        kline_num: k.kline_num,
        open: k.open,
        high: k.high,
        low: k.low,
        close: k.close,
        pip_dif_max: num5((k.high - k.low) * 10_000.),
        pip_dif_oc: num5((k.close - k.open) * 10_000.),
        open_time_str: ots,
        duration: trader3::offline::shared::to_duration((k.close_time - k.open_time) as i64),
    };

    o
}

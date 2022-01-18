use chrono::prelude::*;
use trader3;
use trader3::candle::{
    CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, KlineTA, TimeSerVec, TA2,
};
use trader3::collector;
use trader3::collector::row_data::BTickData;
use trader3::configs::assets::Pair;
use trader3::offline::num5_dep;
use trader3::ta::{DCRes, VelRes};
use serde::{Serialize, Deserialize};

// Note: we do not generate binary for daily now.

const OUT_FOLDER: &'static str = "/mnt/j/trader/data_fast/";
const DAILY_DATA: bool = false;

pub fn main() {
    let pairs = trader3::configs::assets::get_all_symbols();
    // let pairs = vec![trader3::configs::assets::Pair::EURUSD]; // todo: remove

    for pair in pairs {
        for week_id in 25..=53 {
            let path = format!("/mnt/j/trader/data/{}/{}.tsv", pair.folder_path(), week_id);
            if std::path::Path::new(&path).exists() {
                let ticks = trader3::collector::loader::load_rows(&path);

                // Fast weekly data
                write_week_fast(&ticks,&pair,week_id);

                if DAILY_DATA {
                    let mut day_ticks = vec![];

                    let mut start = ticks.first().unwrap().timestamp_sec;
                    let mut day_num = 1;

                    for t in ticks {
                        if t.timestamp_sec < start + 86_400 {
                            day_ticks.push(t);
                        } else {
                            write_single_daily(day_ticks.clone(), &pair, week_id, day_num);
                            day_num += 1;
                            start = t.timestamp_sec;
                            day_ticks.clear();
                            day_ticks.push(t);
                        }
                    }
                    write_single_daily(day_ticks.clone(), &pair, week_id, day_num);
                }
            }
        }
    }
}

pub fn write_single_daily(ticks: Vec<BTickData>, pair: &Pair, week_id: u64, day_num: u64) {
    let s = trader3::core::helper::to_csv_out(&ticks, true);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
    let out_file_path = format!("{}{}/{:}_{}.tsv", OUT_FOLDER, &pair.folder_path(), week_id, day_num);

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, s);
    println!("{}", &out_file_path);
}

pub fn write_week_fast(ticks: &Vec<BTickData>, pair: &Pair, week_id: u64) {
    let mut arr_fast = vec![];
    for t in ticks {
        arr_fast.push(t.to_fast_bin());
    }

    let encoded= bincode::serialize(&arr_fast).unwrap();
    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
    let out_file_path = format!("{}{}/{}.bin", OUT_FOLDER, &pair.folder_path(), week_id);

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, encoded);
    println!("{}", &out_file_path);
}


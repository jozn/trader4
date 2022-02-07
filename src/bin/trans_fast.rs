use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use trader4;
use trader4::candle::{
    CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, KlineTA, TimeSerVec, TA2,
};
use trader4::collector;
use trader4::collector::row_data::BTickData;
use trader4::configs::assets::Pair;
use trader4::offline::num5_dep;
use trader4::ta::{DCRes, VelRes};

// Note: we do not generate binary for daily now.

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_fast/";
const DAILY_DATA: bool = false;

pub fn main() {
    let pairs = trader4::configs::assets::get_all_symbols();
    // let pairs = vec![trader4::configs::assets::Pair::EURUSD]; // todo: remove

    for pair in pairs {
        for week_id in 25..=60 {
            // let cat = pair.to_category();
            let path_tsv = format!("/mnt/t/trader/data/{}/{}.tsv", pair.folder_path(), week_id);
            let path_bin = format!("{}{}/{}.bin", OUT_FOLDER, &pair.folder_path(), week_id);
            if std::path::Path::new(&path_bin).exists() {
                continue;
            }
            if std::path::Path::new(&path_tsv).exists() {
                let ticks = trader4::collector::loader::load_tsv_rows(&path_tsv, &pair);

                // Fast weekly data
                write_week_fast(&ticks, &pair, week_id);

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
    let s = trader4::core::helper::to_csv_out(&ticks, true);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
    let out_file_path = format!(
        "{}{}/{:}_{}.tsv",
        OUT_FOLDER,
        &pair.folder_path(),
        week_id,
        day_num
    );

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

    let encoded = bincode::serialize(&arr_fast).unwrap();
    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
    let out_file_path = format!("{}{}/{}.bin", OUT_FOLDER, &pair.folder_path(), week_id);

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, encoded);
    println!("{}", &out_file_path);
}

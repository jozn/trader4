use chrono::prelude::*;
use trader3;
use trader3::candle::{
    CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, KlineTA, TimeSerVec, TA2,
};
use trader3::collector;
use trader3::configs::assets::Pair;
use trader3::offline::num5;
use trader3::ta::{DCRes, VelRes};

pub fn main() {
    let pairs = trader3::configs::assets::get_all_symbols();

    for pair in pairs {
        for week_id in 1..=53 {
            let path = format!("/mnt/c/me/data/{:?}/{}.tsv", pair, week_id);
            if std::path::Path::new(&path).exists() {
                let ticks = trader3::collector::loader::load_rows(&path);
                let mut arr = TimeSerVec::new();
                let mut frames = vec![];
                let mut id = 1;
                for t in ticks {
                    arr.push(t.to_tick());
                    if arr.len() == 400 {
                        let mut fm = trader3::dc_intel::FrameMem::default();
                        fm.frame_id = id;
                        fm.add_ticks(arr.clone());
                        arr.clear();
                        frames.push(fm.to_csv());
                        id +=1;
                    }
                }

                let s = trader3::core::helper::to_csv_out(&frames, true);

                // Write to file
                const FOLDER: &'static str = "/mnt/c/me/data_dc_intel/";
                let dir = format!("{}{:?}", FOLDER, pair);
                let out_file_path = format!("{}{:?}/{}.tsv", FOLDER, pair, week_id);

                use std::fs;
                fs::create_dir_all(&dir);
                fs::write(&out_file_path, s);
                println!("{}", &out_file_path);
            }
        }
    }
}

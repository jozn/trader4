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
    let pairs = vec![trader3::configs::assets::Pair::EURUSD]; // todo: remove

    for pair in pairs {
        for week_id in 1..=53 {
            let path = format!("/mnt/c/me/data/{:?}/{}.tsv", pair, week_id);
            if std::path::Path::new(&path).exists() {
                let ticks = trader3::collector::loader::load_rows(&path);
                let mut dc_parent = trader3::dc_intel::DCParent::new();

                for t in ticks {
                    dc_parent.add_tick(&t.to_tick());
                }

                let mut frames = vec![];
                for fm in dc_parent.frames.iter() {
                    frames.push(fm.to_csv());
                }

                let s = trader3::core::helper::to_csv_out(&frames, false);

                // Write to file
                const FOLDER: &'static str = "/mnt/c/me/data_dc_intel/";
                let dir = format!("{}{:?}", FOLDER, pair);
                let out_file_path = format!("{}{:?}/{}.csv", FOLDER, pair, week_id);

                use std::fs;
                fs::create_dir_all(&dir);
                fs::write(&out_file_path, s);
                println!("{}", &out_file_path);
            }
        }
    }
}

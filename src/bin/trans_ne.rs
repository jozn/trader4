use chrono::prelude::*;
use trader3;
use trader3::candle::{
    CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, KlineTA, TimeSerVec, TA2,
};
use trader3::collector;
use trader3::collector::row_data::BTickData;
use trader3::configs::assets::Pair;
use trader3::ne2::{FrameCsv, NEFrame};
use trader3::offline::num5_dep;
use trader3::ta::{DCRes, VelRes};

const OUT_FOLDER: &'static str = "/mnt/c/me/data_ne/";

pub fn main() {
    // let pairs = trader3::configs::assets::get_all_symbols();
    let pairs = vec![trader3::configs::assets::Pair::USDCHF]; // todo: remove

    for pair in pairs {
        for week_id in 25..=53 {
            let path = format!("/mnt/c/me/data/{:?}/{}.tsv", pair, week_id);
            if std::path::Path::new(&path).exists() {
                let ticks = trader3::collector::loader::load_rows(&path);
                let mut root = trader3::ne2::NERoot::new();

                for t in ticks.clone() {
                    root.add_tick(&t.to_tick());
                }

                let frames = to_frame_csv(root.frames.clone());

                let s = trader3::core::helper::to_csv_out(&frames, false);

                // Write to file
                let dir = format!("{}{:?}", OUT_FOLDER, &pair);
                let out_file_path = format!("{}{:?}/{}.csv", OUT_FOLDER, &pair, week_id);

                use std::fs;
                fs::create_dir_all(&dir);
                fs::write(&out_file_path, s);
                println!("{}", &out_file_path);

                // Note this behaviout s
                // Write each day
                // wriet_daily(ticks, &pair, week_id);

                // Write frames for each day
                let mut day_frames = vec![];
                let mut start = root.frames.first().unwrap().ohlc.open_time;
                let mut day_num = 1;
                for frame in root.frames {
                    if frame.ohlc.open_time < start + 86_400 {
                        day_frames.push(frame);
                    } else {
                        write_single_day_frames(day_frames.clone(), &pair, week_id, day_num);
                        day_num += 1;
                        start = frame.ohlc.open_time;
                        day_frames.clear();
                        day_frames.push(frame);
                    }
                }
                // last day (5)
                write_single_day_frames(day_frames.clone(), &pair, week_id, day_num);
            }
        }
    }
}

pub fn write_single_day_frames(frames_arr: Vec<NEFrame>, pair: &Pair, week_id: u64, day_num: u64) {
    let frames = to_frame_csv(frames_arr);

    let s = trader3::core::helper::to_csv_out(&frames, false);

    // Write to file
    let dir = format!("{}{:?}", OUT_FOLDER, pair);
    let out_file_path = format!("{}{:?}/{}_{}.csv", OUT_FOLDER, pair, week_id, day_num);

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, s);
    println!("{}", &out_file_path);
}

fn to_frame_csv(frames: Vec<NEFrame>) -> Vec<FrameCsv> {
    let mut arr = vec![];
    for fm in frames.iter() {
        let mut fm = fm.clone();
        let dc_str = &fm.strength;
        /*if dc_str.dis_bull == 2 && fm.trd2 > 0. && fm.vel.avg_vel_pip > 0. {
            fm.ohlc.close = fm.ohlc.close * 1.002; // 2pip
        }
        if dc_str.dis_bear == 2 && fm.trd2 < 0. && fm.vel.avg_vel_pip < 0. {
            fm.ohlc.close = fm.ohlc.close * 1.003; // 2pip
        }*/
        // Disable now
        /*
        if dc_str.buy  {
            fm.ohlc.close = fm.ohlc.close * 1.003; // 3pip
        }
        if dc_str.sell {
            fm.ohlc.close = fm.ohlc.close * 1.0015; // .5pip
        }*/
        arr.push(fm.to_csv());
    }
    arr
}

/////////////////// Deprecated Codes - To Delete //////////////////

// Note: this function is deprecated as we do not see the previous day ticks and it causes
//  to have an start wrong day data and idicators.
pub fn wriet_daily(ticks: Vec<BTickData>, pair: &Pair, week_id: u64) {
    let mut day_ticks = vec![];
    let mut start = ticks.first().unwrap().timestamp_sec;
    let mut day_num = 1;
    for t in ticks {
        if t.timestamp_sec < start + 86_400 {
            day_ticks.push(t);
        } else {
            wriet_single_daily(day_ticks.clone(), pair, week_id, day_num);
            day_num += 1;
            start = t.timestamp_sec;
            day_ticks.clear();
            day_ticks.push(t);
        }
    }
    wriet_single_daily(day_ticks.clone(), pair, week_id, day_num);
}

// todo: change in way not to build a new NERoot but break wikly frames into days
pub fn wriet_single_daily(ticks: Vec<BTickData>, pair: &Pair, week_id: u64, day_num: u64) {
    let mut root = trader3::ne2::NERoot::new();

    for t in ticks.clone() {
        root.add_tick(&t.to_tick());
    }

    let frames = to_frame_csv(root.frames.clone());

    let s = trader3::core::helper::to_csv_out(&frames, false);

    // Write to file
    let dir = format!("{}{:?}", OUT_FOLDER, pair);
    let out_file_path = format!("{}{:?}/{}_{}.csv", OUT_FOLDER, pair, week_id, day_num);

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, s);
    println!("{}", &out_file_path);
}

fn wriet_file_dep(content: String, pair: &Pair, week_id: u64, day_num: u64) {
    // Write to file
    let dir = format!("{}{:?}", OUT_FOLDER, pair);
    let out_file_path = if day_num == 0 {
        format!("{}{:?}/{}.csv", OUT_FOLDER, &pair, week_id)
    } else {
        format!("{}{:?}/{}_{}.csv", OUT_FOLDER, pair, week_id, day_num)
    };

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, content);
    println!("{}", &out_file_path);
}

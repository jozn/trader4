use chrono::prelude::*;
use trader3;
use trader3::candle::{
    CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, KlineTA, TimeSerVec, TA2,
};
use trader3::collector;
use trader3::collector::row_data::BTickData;
use trader3::configs::assets::Pair;
use trader3::sky_eng::*;
use trader3::ta::{DCRes, VelRes};

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_sky/";

pub fn main() {
    let pairs = trader3::configs::assets::get_all_symbols();
    let pairs = vec![trader3::configs::assets::Pair::USDCHF];

    for pair in pairs {
        if pair.is_forex() {
            // continue;
        }
        for week_id in 25..=60 {
            let ticks = trader3::collector::loader::load_week(&pair, week_id);
            if ticks.len() == 0 {
                continue;
            }
            let mut sky_eng = trader3::sky_eng::SkyEng::new();

            for t in ticks.clone() {
                sky_eng.add_tick(&t.to_tick());
            }

            let frames = to_frame_csv(sky_eng.frames.clone());

            let s = trader3::core::helper::to_csv_out(&frames, false);

            // Write to file
            let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
            let out_file_path = format!("{}{}/{}.csv", OUT_FOLDER, &pair.folder_path(), week_id);

            use std::fs;
            fs::create_dir_all(&dir);
            fs::write(&out_file_path, s);
            println!("{}", &out_file_path);

            // Write frames for each day
            if sky_eng.frames.len() > 0 {
                let mut day_frames = vec![];
                let mut start = sky_eng.frames.first().unwrap().ohlc.open_time;
                let mut day_num = 1;
                for frame in sky_eng.frames {
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
            // }
        }
    }
}

pub fn write_single_day_frames(frames_arr: Vec<SFrame>, pair: &Pair, week_id: u16, day_num: u64) {
    let frames = to_frame_csv(frames_arr);

    let s = trader3::core::helper::to_csv_out(&frames, false);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
    let out_file_path = format!(
        "{}{}/{}_{}.csv",
        OUT_FOLDER,
        pair.folder_path(),
        week_id,
        day_num
    );

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, s);
    println!("{}", &out_file_path);
}

fn to_frame_csv(frames: Vec<SFrame>) -> Vec<FrameCsv> {
    let mut arr = vec![];
    for fm in frames.iter() {
        let mut fm = fm.clone();

        arr.push(fm.to_csv());
    }
    arr
}

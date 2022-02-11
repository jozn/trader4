use chrono::prelude::*;
use trader4;
use trader4::collector;
use trader4::collector::row_data::BTickData;
use trader4::configs::assets::Pair;
use trader4::sky_eng::*;
use trader4::ta::{DCRes, VelRes};

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_sky2/";

pub fn main() {
    let pairs = trader4::configs::assets::get_all_symbols();
    let pairs = vec![trader4::configs::assets::Pair::USDCHF];

    for pair in pairs {
        if pair.is_forex() {
            // continue;
        }
        for week_id in 25..=60 {
            let ticks = trader4::collector::loader::load_week(&pair, week_id);
            if ticks.len() == 0 {
                continue;
            }
            let mut sky_eng = trader4::sky_eng::SkyEng::new();

            for t in ticks.clone() {
                sky_eng.add_tick(&t);
            }

            let frames = to_frame_csv(sky_eng.frames.clone());

            let s = trader4::core::helper::to_csv_out(&frames, false);

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
                let mut start = sky_eng.frames.first().unwrap().bar.primary.open_time;
                let mut day_num = 1;
                for frame in sky_eng.frames {
                    if frame.bar.primary.open_time < start + 86_400_000 {
                        day_frames.push(frame);
                    } else {
                        write_single_day_frames(day_frames.clone(), &pair, week_id, day_num);
                        day_num += 1;
                        start = frame.bar.primary.open_time;
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

    let s = trader4::core::helper::to_csv_out(&frames, false);

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

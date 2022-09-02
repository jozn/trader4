use crate::configs::assets::Pair;
use crate::core::brain::*;
use crate::helper;
use crate::helper::*;
use crate::sig_engs::ml_eng::{FrameCsv, FrameCsvV2, MLFrame};
use crate::types::*;
use rstats::Printing;

pub const OUT_FOLDER_CSV: &'static str = "/mnt/t/trader_out/v17/data_sky_web/";

pub fn write_ml_csv(brain: &Brain) {
    for pm in brain.db.iter() {
        let pair = &pm.pair;
        let frames = &pm.ml_eng.frames;
        if frames.len() > 0 {
            // All
            write_csv2(&frames, pair, "all");

            let first = frames.first().unwrap();
            let mut week_id = timestamp_to_week(first.info.bar_medium.primary.open_time);

            let mut arr_frames = vec![];
            for f in frames.iter() {
                let wi = timestamp_to_week(f.info.bar_medium.primary.open_time);
                if week_id.week_id == wi.week_id {
                    arr_frames.push(f.clone());
                } else {
                    write_csv2(&arr_frames, &pair, &week_id.week_id.to_str());

                    arr_frames.clear();
                    week_id = wi;
                    arr_frames.push(f.clone());
                };
            }

            // Last
            write_csv2(&arr_frames, &pair, &week_id.week_id.to_str());
        }
    }
}

fn write_csv2(frames: &Vec<MLFrame>, pair: &Pair, week_id: &str) {
    let folder = format!(
        "{}/{}/ml_csv/{}_{}.csv",
        OUT_FOLDER_CSV,
        pair.folder_path(),
        week_id,
        &pair.to_string(),
    );
    helper::create_folders_for_file(&folder);

    let fff: Vec<FrameCsvV2> = frames.iter().map(|f| f.to_csv_v2()).collect();
    // let fff :Vec<FrameCsv> = frames.iter().map(|f| f.to_csv()).collect();
    let csv = to_csv_out_v3(&fff, false, true);
    std::fs::write(folder, csv);
}

use crate::configs::assets::Pair;
use crate::core::brain::*;
use crate::helper;
use crate::helper::*;
use crate::offline::Position;
use crate::sig_engs::ml_eng::*;
use crate::sim::ml_csv_writer::OUT_FOLDER_CSV;
use crate::types::*;

// const OUT_FOLDER_CSV: &'static str = "/mnt/t/trader_out/v17/data_sky_web/";

// For each Pair must be called seprately
pub fn write_ml_csv_feed(trades: Vec<Position>, pair: Pair) {
    if trades.len() == 0 {
        return;
    }
    let mut wins = vec![];
    let mut loose = vec![];
    for t in trades.clone() {
        if t.profit > 0. {
            wins.push(t);
        } else {
            loose.push(t);
        }
    }
    let folder = helper::get_time_sec();

    write_csv_feed(&trades, pair, "all", folder);
    write_csv_feed(&wins, pair, "wins", folder);
    write_csv_feed(&loose, pair, "loose", folder);
}

fn write_csv_feed(frames: &Vec<Position>, pair: Pair, file_ext_name: &str, rnd: u64) {
    let suffix = rnd % 1000;
    let folder = format!(
        "{}/{}/ml_feed/{}/{}_{}_{}.csv",
        OUT_FOLDER_CSV,
        pair.folder_path(),
        rnd,
        file_ext_name,
        &pair.to_string(),
        suffix
    );
    helper::create_folders_for_file(&folder);

    let fff: Vec<MLFrameFeedV2> = frames
        .iter()
        .map(|f| f.new_pos.new_pos.frame_ml_ref.to_ml_feed_csv_v2(f))
        .collect();
    // let fff :Vec<FrameCsv> = frames.iter().map(|f| f.to_csv()).collect();
    let csv = to_csv_out_v3(&fff, false, true);
    std::fs::write(folder, &csv);

    //  Copy in main folder
    let folder = format!(
        "{}/{}/ml_feed/{}_{}.csv",
        OUT_FOLDER_CSV,
        pair.folder_path(),
        file_ext_name,
        &pair.to_string(),
    );
    std::fs::write(folder, csv);
}

use chrono::prelude::*;
use std::os::unix::raw::off_t;
use trader3;
// use trader3::candle::{
//     CandleConfig, CandleSeriesTA, Kline, KlineHolderFrameTA, KlineTA, TimeSerVec, TA2,
// };
use trader3::collector;
use trader3::collector::row_data::BTickData;
use trader3::configs::assets::Pair;
use trader3::sky_eng2::*;
use trader3::ta::{DCRes, VelRes};

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_sky_web/";

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
            let mut sky_eng = trader3::sky_eng2::SkyEng::new();

            for t in ticks.clone() {
                sky_eng.add_tick(&t);
            }

            let frames = to_json_out(sky_eng.frames.clone());

            let json_data = serde_json::to_string_pretty(&frames).unwrap();

            let html_tmpl = std::fs::read_to_string("./src/web/tmpl/ui2.html").unwrap();
            let html = html_tmpl.replace("{{JSON_DATA}}", &json_data);

            // Write to file
            let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
            let out_file_path = format!("{}{}/{}.html", OUT_FOLDER, &pair.folder_path(), week_id);

            use std::fs;
            fs::create_dir_all(&dir);
            fs::write(&out_file_path, html);
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
    let json_data = to_json_out(frames_arr);
    let json_text = serde_json::to_string_pretty(&json_data).unwrap();
    let html_tmpl = std::fs::read_to_string("./src/web/tmpl/ui2.html").unwrap();
    let html = html_tmpl.replace("{{JSON_DATA}}", &json_text);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());
    let out_file_path = format!(
        "{}{}/{}_{}.html",
        OUT_FOLDER,
        pair.folder_path(),
        week_id,
        day_num
    );

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&out_file_path, html);
    println!("{}", &out_file_path);
}

fn to_json_out(frames: Vec<SFrame>) -> JsonOut {
    let mut out = JsonOut::default();
    for fm in frames.iter() {
        out.ohlc.push(JsonRowOHLC::new(&fm.bar.primary));
    }
    out
}

///////////////////////////////////////////////////
////////////////// Json Types /////////////////////

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JsonOut {
    pub ohlc: Vec<JsonRowOHLC>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JsonRowOHLC {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl JsonRowOHLC {
    fn new(b: &Bar) -> JsonRowOHLC {
        Self {
            time: b.open_time / 1000,
            open: b.open,
            high: b.high,
            low: b.low,
            close: b.close,
        }
    }
}

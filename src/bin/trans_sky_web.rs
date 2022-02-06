use chrono::prelude::*;
use std::os::unix::raw::off_t;
use trader3;
use trader3::base::OHLCV;
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
            let title = format!("{:?}/{}", &pair, week_id);
            let html = html_tmpl.replace("{{TITLE}}", &title);
            let html = html.replace("{{JSON_DATA}}", &json_data);

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
    let title = format!("{:?}/{}_{}", &pair, week_id, day_num);

    let json_data = to_json_out(frames_arr);
    let json_text = serde_json::to_string_pretty(&json_data).unwrap();
    let html_tmpl = std::fs::read_to_string("./src/web/tmpl/ui2.html").unwrap();
    let html = html_tmpl.replace("{{TITLE}}", &title);
    let html = html.replace("{{JSON_DATA}}", &json_text);

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

        // Set high/low lines
        let bar = &fm.bar.primary;
        let ta = &bar.ta;
        let bta = &fm.bar.big.ta;
        let time = bar.open_time / 1000;
        out.high_line.push(RowJson {
            time: bar.open_time / 1000,
            value: ta.rpi.high,
        });
        out.low_line.push(RowJson {
            time: bar.open_time / 1000,
            value: ta.rpi.low,
        });

        // Trend line
        out.bull_line.push(RowJson {
            time,
            value: bta.trend.bull_line, // green
        });
        out.bear_line.push(RowJson {
            time,
            value: bta.trend.bear_line,
        });

        // Set buy/sell markers
        if fm.buy1 {
            out.markers.push(MarkerJson {
                time,
                position: "belowBar".to_string(),
                color: "#2196F3".to_string(),
                shape: "arrowUp".to_string(),
                text: format!(""), // text: format!("Buy @")
                                   // text: format!("Buy @ {}", bar.hlc3())
            })
        }
        if fm.sell1 {
            out.markers.push(MarkerJson {
                time,
                position: "aboveBar".to_string(),
                color: "#e91e63".to_string(),
                shape: "arrowDown".to_string(),
                text: format!(""), // text: format!("Sell @")
                                   // text: format!("Sell @ {}", bar.hlc3())
            })
        }

        // Add scores
        let score = &fm.score;
        out.score_bull.push(RowJson {
            time,
            value: score.bull as f64,
        });
        out.score_bear.push(RowJson {
            time,
            value: -score.bear as f64,
        });
        out.score_diff.push(RowJson {
            time,
            value: score.diff as f64,
        });
    }
    out
}

///////////////////////////////////////////////////
////////////////// Json Types /////////////////////

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JsonOut {
    pub ohlc: Vec<JsonRowOHLC>,
    pub high_line: Vec<RowJson>,
    pub low_line: Vec<RowJson>,
    pub markers: Vec<MarkerJson>,

    pub bull_line: Vec<RowJson>,
    pub bear_line: Vec<RowJson>,

    pub score_bull: Vec<RowJson>,
    pub score_bear: Vec<RowJson>,
    pub score_diff: Vec<RowJson>,
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
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RowJson {
    pub time: i64,
    pub value: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MarkerJson {
    pub time: i64,
    pub position: String,
    pub color: String,
    pub shape: String,
    pub text: String,
}
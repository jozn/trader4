use chrono::prelude::*;
use trader4;
use trader4::base::OHLCV;
use trader4::collector;
use trader4::collector::row_data::BTickData;
use trader4::configs::assets::Pair;
use trader4::sky_eng::*;
use trader4::ta::{DCRes, VelRes};

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_sky_web/";

pub fn main() {
    let pairs = trader4::configs::assets::get_all_symbols();
    let pairs = vec![trader4::configs::assets::Pair::USDCHF];
    let pairs = vec![trader4::configs::assets::Pair::EURUSD];

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
                let mut start = sky_eng.frames.first().unwrap().bar_medium.primary.open_time;
                let mut day_num = 1;
                for frame in sky_eng.frames {
                    if frame.bar_medium.primary.open_time < start + 86_400_000 {
                        day_frames.push(frame);
                    } else {
                        write_single_day_frames(day_frames.clone(), &pair, week_id, day_num);
                        day_num += 1;
                        start = frame.bar_medium.primary.open_time;
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
        out.ohlc.push(JsonRowOHLC::new(&fm.bar_medium.primary));

        // Set high/low lines
        let bar = &fm.bar_medium.primary;
        let ta = &bar.ta;
        let pta = &fm.bar_medium.primary.ta;
        let bta = &fm.bar_medium.big.ta;
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

        // Add sample
        let dmi = &pta.dmi;
        // let dmi = &bta.dmi;
        out.sample_1.push(RowJson {
            time,
            value: dmi.plus as f64,
        });
        out.sample_2.push(RowJson {
            time,
            value: dmi.minus as f64,
        });
        out.sample_3.push(RowJson {
            time,
            value: dmi.dmx,
            // value: dmi.adx,
        });

        // DMMD
        let dmmd = &bta.dmmd;
        // let dmmd = &pta.dmmd;
        out.dmmd_1.push(RowJson {
            time,
            // value: dmmd.ma_fast,
            value: dmmd.diff,
        });
        out.dmmd_2.push(RowJson {
            time,
            // value: dmmd.ma_slow,
            value: dmmd.color,
        });

        // Sample 2 - DMMD
        // let dmmd = &bta.dmmd;
        // out.sample_1.push(RowJson {
        //     time,
        //     value: dmmd.ma_fast as f64,
        // });
        // out.sample_2.push(RowJson {
        //     time,
        //     value: dmmd.ma_slow as f64,
        // });
        // out.sample_3.push(RowJson {
        //     time,
        //     value: dmmd.histogram,
        // });
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

    // samples
    pub sample_1: Vec<RowJson>,
    pub sample_2: Vec<RowJson>,
    pub sample_3: Vec<RowJson>,

    pub dmmd_1: Vec<RowJson>,
    pub dmmd_2: Vec<RowJson>,

    // Major timeline
    pub major_ohlc: Vec<JsonRowOHLC>,

    // Primary timeline
    pub prime_ohlc: Vec<JsonRowOHLC>,

    // Small timeline
    pub small_ohlc: Vec<JsonRowOHLC>,
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

use chrono::prelude::*;
use trader4;
use trader4::base::OHLCV;
use trader4::collector;
use trader4::collector::row_data::BTickData;
use trader4::configs::assets::Pair;
use trader4::sky_eng::*;
use trader4::ta::{DCRes, VelRes};

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_sky_web_v3/";

struct WeekData {
    week_id: u16,
    start: i64,
    end: i64,
}

pub fn main() {
    let pairs = trader4::configs::assets::get_all_symbols();
    let pairs = vec![trader4::configs::assets::Pair::USDCHF];
    let pairs = vec![trader4::configs::assets::Pair::EURUSD];

    for pair in pairs {
        if pair.is_forex() {
            // continue;
        }

        let mut week_data = vec![];
        let mut all_ticks = vec![];
        for week_id in 25..=60 {
            let ticks = trader4::collector::loader::load_week(&pair, week_id);
            if ticks.len() == 0 {
                continue;
            }
            week_data.push(WeekData {
                week_id,
                start: ticks.first().unwrap().timestamp,
                end: ticks.last().unwrap().timestamp,
            });
            for t in ticks {
                all_ticks.push(t);
            }
        }
        println!("Ticks loaded.");

        let mut sky_eng = trader4::sky_eng::SkyEng::new();
        for t in all_ticks {
            sky_eng.add_tick(&t);
        }
        println!("Sky Eng Completed.");

        for wd in week_data {
            let jo = sky_eng.to_json(wd.start, wd.end);
            // println!("week m: {}", jo.major_ohlc.len());
            // println!("week s: {}", jo.small_ohlc.len());
            write_json(&jo, &pair, wd.week_id, 0);

            let mut start = wd.start;
            let mut end = start + 86_400_000;
            let mut day_num = 1;
            while end < wd.end {
                // println!("day m: {}", jo.major_ohlc.len());
                // println!("day s: {}", jo.small_ohlc.len());
                let jo = sky_eng.to_json(start, end);
                write_json(&jo, &pair, wd.week_id, day_num);
                start = end;
                end = start + 86_400_000;
                day_num += 1;
                // break; // todo remove
            }
            // last day
            let jo = sky_eng.to_json(start, end);
            write_json(&jo, &pair, wd.week_id, day_num);
        }
    }
}

// pub fn write_json(jo: &SkyJsonOutDep, pair: &Pair, week_id: u16, day_num: u64) {
pub fn write_json(jo: &SkyJsonOut, pair: &Pair, week_id: u16, day_num: u64) {
    let title = if day_num == 0 {
        format!("{:?}/{}", &pair, week_id)
    } else {
        format!("{:?}/{}_{}", &pair, week_id, day_num)
    };

    let file_path = if day_num == 0 {
        format!("{}{}/{}.html", OUT_FOLDER, &pair.folder_path(), week_id)
    } else {
        format!(
            "{}{}/{}_{}.html",
            OUT_FOLDER,
            pair.folder_path(),
            week_id,
            day_num
        )
    };

    let json_text = serde_json::to_string_pretty(&jo).unwrap();
    let html_tmpl = std::fs::read_to_string("../../web/tmpl/ui3.html").unwrap();
    let html = html_tmpl.replace("{{TITLE}}", &title);
    let html = html.replace("{{JSON_DATA}}", &json_text);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&file_path, html);
    println!("{}", &file_path);
}

use crate::bar::PrimaryHolder;
use crate::brain::PairMemory;
use crate::configs::assets::Pair;
use crate::file_output::{FilesOutput, FilesOutputConfig};
use crate::helper::to_csv_out_v2;
use crate::json_output::{bars_to_json, RowJson, SkyJsonOut, TrendAnalyseOut};
use crate::offline;
use crate::offline::{Money, Position};
use crate::sig_engs::ml_eng::{MLEng, MultiBars};
use crate::sig_engs::*;
use crate::sky_eng::SkyEng;
use crate::ta::zigzag::ZigZag;
use crate::ta::Wave;
use crate::types::{DayInfo, WeekInfo};
use serde::{Deserialize, Serialize};

const OUT_FOLDER: &'static str = "/mnt/t/trader_out/v13/data_sky_web/";
const OUT_FOLDER_TREND: &'static str = "/mnt/t/trader_out/v13/trend/";

// Note: the entire soruce file should be deleted as it only used for legacy sky_eng, for refrence only.

//////////////////////////////////////// SkyEng fns /////////////////////////////////////////
impl FilesOutput {
    // dep: delete this- entire sky_eng
    pub fn run_sky_eng_dep(
        &mut self,
        postions: &Vec<Position>,
        pair_mem: &PairMemory,
        money: &Money,
    ) {
        println!("web {:?} ...", &pair_mem.pair);
        self.write_trend_analyse_output_sky_eng(&pair_mem.sky_eng_dep, &postions);
        self.write_web_output_sky_eng(&pair_mem.sky_eng_dep, &postions, self.cfg.days_out);

        if self.cfg.print {
            // println!("{:#?}", x);
            println!("{:#?}", money.balance);
        }

        // todo - get report by date range
        if self.cfg.report {
            // back_ref.report_to_folder(&self.week_data, &self.pair);
        }
    }
    // code copy of trans_wky_web3.rs
    pub fn write_web_output_sky_eng(&self, sky_eng: &SkyEng, pos: &Vec<Position>, days_out: bool) {
        let pair = &self.cfg.pair;
        for wd in &self.week_data {
            let poss = get_postions_range(&pos, wd.start, wd.end);
            ///////// Hack: ma trend anlyse
            sky_eng_to_trend_analyse(sky_eng, wd.start, wd.end, &poss);
            /////////

            let jo = sky_eng_to_json(sky_eng, wd.start, wd.end, &poss);
            // println!("week m: {}", jo.major_ohlc.len());
            // println!("week s: {}", jo.small_ohlc.len());
            write_json(&jo, &poss, &pair, wd.week_id, 0);

            let mut start = wd.start;
            let mut end = start + 86_400_000;
            let mut day_num = 1;
            // while end < wd.end {
            if days_out {
                'days: loop {
                    if day_num == 8 {
                        break 'days;
                    }
                    // println!("day m: {}", jo.major_ohlc.len());
                    // println!("day s: {}", jo.small_ohlc.len());
                    let poss = get_postions_range(&pos, start, end);
                    let jo = sky_eng_to_json(sky_eng, start, end, &poss);
                    if jo.medium.ohlc.len() == 0 {
                        break 'days;
                    }
                    write_json(&jo, &poss, &pair, wd.week_id, day_num);
                    start = end;
                    end = start + 86_400_000;
                    day_num += 1;
                    // break; // todo remove
                }
            }
            // last day
            // let jo = sky_eng.to_json(start, end);
            // let poss = get_postions_range(&pos, start, end);
            // write_json(&jo, &poss, &pair, wd.week_id, day_num);
        }
    }

    // code copy of write_web_output
    pub(crate) fn write_trend_analyse_output_sky_eng(&self, sky_eng: &SkyEng, pos: &Vec<Position>) {
        let pair = &self.cfg.pair;
        for wd in &self.week_data {
            let poss = get_postions_range(&pos, wd.start, wd.end);
            let jo = sky_eng_to_trend_analyse(sky_eng, wd.start, wd.end, &poss);
            write_trend_anlyse(&jo, &poss, &pair, wd.week_id, 0);

            let mut start = wd.start;
            let mut end = start + 86_400_000;
            let mut day_num = 1;
            'days: loop {
                if day_num == 8 {
                    break 'days;
                }
                let poss = get_postions_range(&pos, start, end);
                let jo = sky_eng_to_trend_analyse(sky_eng, start, end, &poss);
                if jo.tt.len() == 0 {
                    // break 'days;
                }
                write_trend_anlyse(&jo, &poss, &pair, wd.week_id, day_num);
                start = end;
                end = start + 86_400_000;
                day_num += 1;
            }
        }
    }
}

struct _WriteParam {
    jo: SkyJsonOut,
    pair: Pair,
    week_id: u16,
    day_num: u64,
}

// code copy of trans_wky_web3.rs
pub fn write_json(jo: &SkyJsonOut, pos: &Vec<Position>, pair: &Pair, week_id: i32, day_num: u64) {
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

    // let trade_markers = offline::position_html::to_json_marker(&pos);
    // let mut jo = jo.clone();
    // jo.markers.
    let json_text = serde_json::to_string_pretty(&jo).unwrap();
    let trades_text = offline::position_html::to_html_table(&pos);

    let html_tmpl = std::fs::read_to_string("./src/web/html/ui6.html").unwrap();
    let js_script = std::fs::read_to_string("./src/web/dist/bundle.js").unwrap();
    // JS libs (jQuery and lightweight)
    let jquery = std::fs::read_to_string("./src/web/libs/jquery.min.js").unwrap();
    let lighweight =
        std::fs::read_to_string("./src/web/libs/lightweight-charts.standalone.development.js")
            .unwrap();
    let libs = format!("{} \n {}", jquery, lighweight);

    // let js_script = std::fs::read_to_string("./src/web/ts/lib.js").unwrap();
    let html = html_tmpl.replace("{{TITLE}}", &title);
    let html = html.replace("{{JSON_DATA}}", &json_text);
    let html = html.replace("{{TRADE_DATA}}", &trades_text);
    let html = html.replace("{{JS_LIBS}}", &libs);
    let html = html.replace("{{JS_SCRIPT}}", &js_script);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&file_path, html);
    println!("{}", &file_path);
}

fn get_postions_range(pos: &Vec<Position>, time_start: i64, time_end: i64) -> Vec<Position> {
    let time_start = time_start as u64 / 1000;
    let time_end = time_end as u64 / 1000;
    let mut out = vec![];
    for p in pos {
        if p.open_time >= time_start && p.close_time <= time_end {
            out.push(p.clone())
        }
    }
    out
}

pub fn write_trend_anlyse(
    jo: &TrendAnalyseOut,
    pos: &Vec<Position>,
    pair: &Pair,
    week_id: i32,
    day_num: u64,
) {
    let title = if day_num == 0 {
        format!("{:?}/{}", &pair, week_id)
    } else {
        format!("{:?}/{}_{}", &pair, week_id, day_num)
    };

    let file_path = if day_num == 0 {
        format!(
            "{}{}/{}.csv",
            OUT_FOLDER_TREND,
            &pair.folder_path(),
            week_id
        )
    } else {
        format!(
            "{}{}/{}_{}.csv",
            OUT_FOLDER_TREND,
            pair.folder_path(),
            week_id,
            day_num
        )
    };
    let html = to_csv_out_v2(&jo.tt, true, true);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER_TREND, pair.folder_path());

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&file_path, html);
    println!("{}", &file_path);
}

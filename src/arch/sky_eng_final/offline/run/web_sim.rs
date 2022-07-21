use crate::brain::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::helper::to_csv_out_v2;
use crate::json_output::{SkyJsonOut, TrendAnalyseOut};
use crate::offline::*;
use crate::sky_eng::{sky_eng_to_json, sky_eng_to_trend_analyse, SkyEng};
use crate::types::WeekData;
use crate::{collector, offline};
use std::ops::Range;
use std::sync::Arc;

// todo: migrate this
const OUT_FOLDER: &'static str = "/mnt/t/trader_out/v12/data_sky_web/";
const OUT_FOLDER_TREND: &'static str = "/mnt/t/trader_out/v12/trend/";

pub struct WebBackRunConfig {
    pub balance: f64,
    pub pairs_conf: Vec<PairBarCfg>,
    pub ticks: Vec<BTickData>,
    pub week_data: Vec<WeekData>,
    pub pair: Pair,
    pub week_id: u16,
    pub print: bool,
    pub report: bool,
    pub web: bool,
    pub report_cfg: BackReportConf,
}

pub struct WebBackRunRes {
    pub free_usd: f64,
}

impl WebBackRunConfig {
    fn load_weeks_data(&mut self, week_rng: Range<u16>) {
        // code copy of trans_wky_web3.rs
        let mut week_data = vec![];
        let mut all_ticks = vec![];
        for week_id in week_rng {
            let ticks = collector::loader::load_week(&self.pair, week_id);
            if ticks.len() == 0 {
                println!("Empty ticks {}", week_id);
                continue;
            }
            week_data.push(WeekData {
                week_id,
                start_ms: ticks.first().unwrap().timestamp,
                end_ms: ticks.last().unwrap().timestamp,
            });
            for t in ticks {
                all_ticks.push(t);
            }
        }
        println!("Ticks loaded.");

        self.ticks = all_ticks;
        self.week_data = week_data;
    }

    pub fn run_web_sim(&mut self, week_rng: Range<u16>, days_out: bool) -> WebBackRunRes {
        self.load_weeks_data(week_rng.clone());
        let backend = BackendEngineOuter::new(self.balance, &self.report_cfg);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain::new(back_arc.clone(), self.pairs_conf.first().unwrap().clone());
        let pair = self.pair.clone();
        for (i, t) in self.ticks.iter().enumerate() {
            if i % 10000 == 0 {
                // println!("{}", i);
            }
            back_arc.next_tick(t.clone());
            brain.on_price_tick(&pair, t.clone());
            let notifys = back_arc.take_notify();
            for not in notifys {
                brain.on_notify_position(not);
            }
        }
        let mut back_ref = back_arc.engine.borrow_mut();
        back_ref.close_all_positions();

        println!("Completed Brain.");

        // ML Statics

        // Print Sky_Eng outputs
        if self.web {
            let mut postions = vec![];
            for (_, p) in back_ref.closed.iter() {
                postions.push(p.clone());
            }

            for (_, pair_mem) in brain.db.iter() {
                println!("web {:?} ...", &pair_mem.pair);
                self.write_trend_analyse_output(&pair_mem.sky_eng_dep, &postions);
                self.write_web_output(&pair_mem.sky_eng_dep, &postions, days_out);
            }
        }

        let money = back_ref.get_money();
        if self.print {
            // println!("{:#?}", x);
            println!("{:#?}", money.balance);
        }

        // todo - get report by date range
        if self.report {
            back_ref.report_to_folder(&self.week_data, &self.pair);
            // back_ref.report_to_folder_dep(&format!(
            //     "_v2_week_{}_{}",
            //     self.week_id,
            //     self.pair.to_string()
            // ));
        }
        WebBackRunRes {
            free_usd: back_ref.balance,
        }
    }

    // code copy of trans_wky_web3.rs
    fn write_web_output(&self, sky_eng: &SkyEng, pos: &Vec<Position>, days_out: bool) {
        let pair = &self.pair;
        for wd in &self.week_data {
            let poss = get_postions_range(&pos, wd.start_ms, wd.end_ms);
            ///////// Hack: ma trend anlyse
            sky_eng_to_trend_analyse(sky_eng, wd.start_ms, wd.end_ms, &poss);
            /////////

            let jo = sky_eng_to_json(sky_eng, wd.start_ms, wd.end_ms, &poss);
            // println!("week m: {}", jo.major_ohlc.len());
            // println!("week s: {}", jo.small_ohlc.len());
            write_json(&jo, &poss, &pair, wd.week_id, 0);

            let mut start = wd.start_ms;
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
    fn write_trend_analyse_output(&self, sky_eng: &SkyEng, pos: &Vec<Position>) {
        let pair = &self.pair;
        for wd in &self.week_data {
            let poss = get_postions_range(&pos, wd.start_ms, wd.end_ms);
            let jo = sky_eng_to_trend_analyse(sky_eng, wd.start_ms, wd.end_ms, &poss);
            write_trend_anlyse(&jo, &poss, &pair, wd.week_id, 0);

            let mut start = wd.start_ms;
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
pub fn write_json(jo: &SkyJsonOut, pos: &Vec<Position>, pair: &Pair, week_id: u16, day_num: u64) {
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
    week_id: u16,
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
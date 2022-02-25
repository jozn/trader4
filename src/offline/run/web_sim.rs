use crate::brain::*;
use crate::collector;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::*;
use crate::sky_eng::{SkyEng, SkyJsonOut};
use crate::types::WeekData;
use std::ops::Range;
use std::sync::Arc;

const OUT_FOLDER: &'static str = "/mnt/t/trader/data_sky_web_v4/";

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

        self.ticks = all_ticks;
        self.week_data = week_data;
    }

    pub fn run_web_sim(&mut self, week_rng: Range<u16>) -> WebBackRunRes {
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

        // Print Sky_Eng outputs
        for (_, pair_mem) in brain.db.iter() {
            println!("web {:?} ...", &pair_mem.pair);
            self.write_web_output(&pair_mem.sky_eng);
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
    fn write_web_output(&self, sky_eng: &SkyEng) {
        let pair = &self.pair;
        for wd in &self.week_data {
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

// code copy of trans_wky_web3.rs
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
    let html_tmpl = std::fs::read_to_string("./src/web/tmpl/ui3.html").unwrap();
    let html = html_tmpl.replace("{{TITLE}}", &title);
    let html = html.replace("{{JSON_DATA}}", &json_text);

    // Write to file
    let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());

    use std::fs;
    fs::create_dir_all(&dir);
    fs::write(&file_path, html);
    println!("{}", &file_path);
}

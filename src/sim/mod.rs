pub mod ml_csv_writer;
pub mod sim_play;
use ml_csv_writer::*;

use crate::core::brain::*;
// use crate::brain::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::file_output::{FilesOutput, FilesOutputConfig};
use crate::gate_api::GateWay;
use crate::helper::to_csv_out_v2;
use crate::json_output::{SkyJsonOut, TrendAnalyseOut};
use crate::offline::*;
use crate::types::{WeekDataDep, WeekInfo};
use crate::{app, collector, offline, types};
use std::ops::Range;
use std::sync::Arc;

// Sim is the simiulater for offline testing with web output and backtest.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct Setting {
    pub pairs: Vec<Pair>,
    pub week_start: u16,
    pub week_end: u16,
    pub print: bool,
    pub report: bool,
    pub days_out: bool,
    pub web: bool,
}

pub struct SimConfig {
    pub balance: f64,
    pub pairs_conf: Vec<PairBarCfg>,
    pub ticks: Vec<BTickData>,
    pub week_data: Vec<WeekInfo>,
    pub pair: Pair,
    pub out: FilesOutputConfig,
    // pub week_id: u16,
    // pub print: bool,
    // pub report: bool,
    // pub web: bool,
    pub report_cfg: BackReportConf,
}

impl SimConfig {
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
            let start_tick = ticks.first().unwrap().timestamp;
            let end_tick = ticks.last().unwrap().timestamp;
            let wi = types::timestamp_to_week(start_tick);
            let wi_e = types::timestamp_to_week(end_tick);
            assert_eq!(wi.week_id, wi_e.week_id);
            week_data.push(wi);
            //
            // week_data.push(WeekData {
            //     week_id,
            //     start: ticks.first().unwrap().timestamp,
            //     end: ticks.last().unwrap().timestamp,
            // });
            for t in ticks {
                all_ticks.push(t);
            }
        }
        println!(
            "Ticks loaded. Weeks: {}  Size: {}",
            week_data.len(),
            all_ticks.len()
        );

        self.ticks = all_ticks;
        self.week_data = week_data.clone();
        self.out.week_data = week_data;
    }

    pub fn run_web_sim(&mut self, week_rng: Range<u16>, days_out: bool) {
        make_output_dirs();
        let start_time = app::core::helper::get_time_ms();
        self.load_weeks_data(week_rng.clone());
        let backend = BackendEngineOuter::new(self.balance, &self.report_cfg);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain::new(back_arc.clone(), self.pairs_conf.first().unwrap().clone());
        let pair = self.pair.clone();
        for (i, t) in self.ticks.iter().enumerate() {
            if i % 1_000_000 == 0 {
                println!("{}", i);
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
        // End tear down
        brain.on_end();

        let end_time = app::core::helper::get_time_ms();
        let run_time = (end_time - start_time) as f64;
        println!("Completed Brain. Runtime: {} sec", run_time / 1000.);

        // Print Sky_Eng outputs
        if self.out.web {
            let mut postions = vec![];
            for (_, p) in back_ref.closed.iter() {
                postions.push(p.clone());
            }

            for pair_mem in brain.db.iter() {
                println!("web {:?} ...", &pair_mem.pair);
                let mut file_out = FilesOutput {
                    cfg: self.out.clone(),
                    week_data: self.week_data.clone(),
                };
                // file_out.run_sky_eng(&postions, &pair_mem, &back_ref.get_money());
                file_out.run_sig_eng(&postions, &pair_mem, &back_ref.get_money());
                // todo below
                // self.write_trend_analyse_output(&pair_mem.sky_eng, &postions);
                // self.write_web_output(&pair_mem.sky_eng, &postions, days_out);
            }
        }

        if self.out.report {
            back_ref.report_to_folder(&self.week_data, &self.pair);
        }

        // CSV of ML
        write_ml_csv(&brain);

        let end_time = app::core::helper::get_time_ms();
        let run_time = (end_time - start_time) as f64;
        println!(
            "Fully end {:?} Runtime: {} sec",
            &self.pair,
            run_time / 1000.
        );
    }
}

fn make_output_dirs() {
    std::fs::create_dir_all("./debug/runtime/");
}

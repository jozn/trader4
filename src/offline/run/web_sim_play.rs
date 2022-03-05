use std::sync::Arc;

use crate::bar::BarConfig;
use crate::collector;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::helper::get_time_sec;
use crate::offline::*;

use super::web_sim::*;
use super::*;

pub fn run1() {
    // run_pair(&Pair::EURUSD);
    run_pair(&Pair::USDCHF);
    // run_pair(&Pair::USDCAD);
    // run_pair(&Pair::NZDUSD);
    // run_pair(&Pair::GBPUSD);
    // run_pair(&Pair::ADAUSD);
}

pub fn run2() {
    let pairs = assets::get_all_usd_forex_symbols();
    let pairs = assets::get_all_symbols();
    for p in &pairs {
        if !p.is_forex() {
            run_pair(p);
        }
    }
}

pub fn run_pair(pair: &Pair) {
    let primary_ticks = 150;
    let pair_cfg = (
        // Pair::EURUSD,
        pair.clone(),
        BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        },
    );

    let mut run_cfg = WebBackRunConfig {
        balance: 100_000.,
        pairs_conf: vec![pair_cfg],
        ticks: vec![],
        week_data: vec![],
        pair: pair.clone(),
        week_id: 1,
        print: true,
        report: true,
        web: true,
        report_cfg: BackReportConf {
            report_folder: "".to_string(),
            report_sub_folder: "".to_string(),
        },
    };

    // run_cfg.run_web_sim(25..60);
    run_cfg.run_web_sim(25..60);
}

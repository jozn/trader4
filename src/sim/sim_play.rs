use rand::Rng;
use std::sync::Arc;

use crate::bar::BarConfig;
use crate::collector;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::helper::get_time_sec;
use crate::offline::*;

// use super::web_sim::*;
use super::*;

pub fn run_setting() {
    let args: Args = Args::parse();
    println!("Hello {:#?}!", args);

    let set_path = format!("./{}.json", args.setting);
    // let js = std::fs::read_to_string("./settings.json").unwrap();
    let js = std::fs::read_to_string(set_path).unwrap();
    let set: app::sim::Setting = serde_json::from_str(&js).unwrap();
    // println!("{:#?}", set);

    for p in &set.pairs {
        run_pair_setting(p, &set);
    }
}
fn run_pair_setting(pair: &Pair, setting: &Setting) {
    let primary_ticks = 150;
    let pair_cfg = (
        // Pair::EURUSD,
        pair.clone(), // todo: not used this field - go to sky_eng in there we have this
        BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        },
    );

    let mut run_cfg = SimConfig {
        balance: 100_000.,
        pairs_conf: vec![pair_cfg],
        ticks: vec![],
        week_data: vec![],
        pair: pair.clone(),
        out: FilesOutputConfig {
            week_data: vec![],
            pair: pair.clone(),
            print: setting.print,
            report: setting.report,
            // days_out: false,
            days_out: setting.days_out,
            web: setting.web,
        },
        report_cfg: BackReportConf {
            report_folder: "".to_string(),
            report_sub_folder: "".to_string(),
        },
    };
    let rng = Range {
        start: setting.week_start,
        end: setting.week_end,
    };
    run_cfg.run_web_sim(rng, false);
}

/*

/////// Deprecated - not Setting based /////////////
//// todo del

pub fn run1() {
    // rstats::MinMax{}
    // run_pair(&Pair::EURUSD);
    // run_pair(&Pair::USDCHF);
    // run_pair(&Pair::USDCAD);
    // run_pair(&Pair::NZDUSD);
    // run_pair(&Pair::GBPUSD);
    // run_pair(&Pair::ADAUSD);
    // run_pair(&Pair::Oracle);
    // run_pair(&Pair::XAGUSD);
    // run_pair(&Pair::BTCUSD);
    // run_pair(&Pair::US30);
    // run_pair(&Pair::UK100);
    // run_pair(&Pair::CN50);
    // run_pair(&Pair::Apple);
    // run_pair(&Pair::IBM);
    // run_pair(&Pair::Gasoline);

    // run_pair(&Pair::EURUSD);
    // run_pair(&Pair::USDJPY);

    run_pair(&Pair::XAUUSD);
    // run_pair(&Pair::SpotCrude);
    // run_pair(&Pair::USDCHF);
}
pub fn run2() {
    // run_pair(&Pair::USDCNH);

    let pairs = assets::get_all_usd_forex_symbols();
    let pairs = assets::get_all_symbols();
    let pairs = assets::get_symbols_samples();
    // let pairs = assets::get_symbols_trnd();
    for p in &pairs {
        run_pair(p);
        // if p.is_us_stocks() || p.is_index(){
        let r: f64 = rand::thread_rng().gen();
        if !p.is_forex() && r > 0.7 {
            // run_pair(p);
        }
    }
}

pub fn run_pair(pair: &Pair) {
    let primary_ticks = 150;
    // match pair { }
    // let primary_ticks = if pair.is_forex() {
    //     950
    // } else if pair.is_us_stocks() {
    //     900
    // } else if pair.is_crypto() {
    //     900
    // } else {
    //     900
    // };
    let pair_cfg = (
        // Pair::EURUSD,
        pair.clone(), // todo: not used this field - go to sky_eng in there we have this
        BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        },
    );

    let mut run_cfg = SimConfig {
        balance: 100_000.,
        pairs_conf: vec![pair_cfg],
        ticks: vec![],
        week_data: vec![],
        pair: pair.clone(),
        out: FilesOutputConfig {
            week_data: vec![],
            pair: pair.clone(),
            print: true,
            report: true,
            // days_out: false,
            days_out: true,
            web: true,
        },
        report_cfg: BackReportConf {
            report_folder: "".to_string(),
            report_sub_folder: "".to_string(),
        },
    };

    // run_cfg.run_web_sim(25..60);
    // run_cfg.run_web_sim(25..32);
    // run_cfg.run_web_sim(45..60);
    // run_cfg.run_web_sim(45..47);
    // run_cfg.run_web_sim(50..60, false);
    // run_cfg.run_web_sim(25..60, false);
    // run_cfg.run_web_sim(45..60, false);
    // run_cfg.run_web_sim(52..60, false);
    // run_cfg.run_web_sim(45..55, false);
    run_cfg.run_web_sim(50..55, false);

    // one
    // run_cfg.run_web_sim(54..60, false);
}
*/

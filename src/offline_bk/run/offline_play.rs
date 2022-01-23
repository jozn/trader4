use std::sync::Arc;

use crate::candle::CandleConfig;
use crate::collector;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::helper::get_time_sec;
use crate::offline::{BackReportConf, BackendEngine, BackendEngineOuter};

use super::*;

pub fn run1() {
    run_pair(&Pair::USDCHF);
}

pub fn run12() {
    let pairs = assets::get_all_symbols();
    for p in &pairs {
        if p.is_forex() {
            run_pair(p);
        }
    }
}

pub fn run_pair(pair: &Pair) {
    let pair_cfg = (
        // Pair::EURUSD,
        pair.clone(),
        CandleConfig {
            // small_tick: 30,
            // medium_tick: 10,
            // big_tick: 50,
            // vel1_period: 20,
            // vel2_period: 50,
            small_tick: 30,
            medium_tick: 10,
            big_tick: 120,
            vel1_period: 20,
            vel2_period: 50,
        },
    );
    let week_id = 25;
    let week_id = 49;
    // let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/1.tsv");
    // let ticks = collector::loader::load_all_pair(&Pair::USDCAD, 25..50);
    let ticks = collector::loader::load_all_pair(&pair, 25..50);
    // let ticks = collector::loader::load_all_pair(&Pair::NZDUSD, 25..50);
    // let ticks = collector::loader::load_all_pair(&Pair::USDCHF, 25..50);
    // let ticks = collector::loader::load_week(&Pair::USDCHF, 25);
    // let ticks = collector::loader::load_all_pair(&Pair::EURUSD, 25..50);
    // let ticks = collector::loader::load_week(&Pair::EURUSD, 49);
    // let ticks = collector::loader::load_week(&Pair::EURUSD, week_id);
    // let ticks = collector::loader::load_day(&Pair::EURUSD, week_id, 3);
    // let ticks = collector::loader::load_all_pair(&Pair::EURUSD, 44..45);
    println!("loaded... {:?}", &pair);
    let mut run_cfg = BackRunConfig {
        balance: 100_000,
        pairs_conf: vec![pair_cfg],
        ticks,
        pair: pair.clone(),
        week_id: week_id,
        print: true,
        report: true,
        report_cfg: BackReportConf {
            report_folder: "../trader5_out/".to_string(),
            report_sub_folder: "".to_string(),
        },
    };

    run_cfg.run_brain4();
}

pub fn run_optimized() {
    let mut bal = vec![];
    let mut sum = 0.;
    let mut sum_abs = 0.;
    let mut weeks_up = 0;
    let mut weeks_down = 0;

    let mut sub_folder_time = get_time_sec();
    for i in 25..=53 {
        // let tsv = format!("{:?}/{}.tsv", Pair::EURUSD, i);
        let tsv = format!("{:?}/{}.tsv", Pair::USDCHF, i);
        let path = format!("/mnt/c/me/data/{}", tsv);
        if std::path::Path::new(&path).exists() {
            let pair_cfg = (
                Pair::EURUSD,
                CandleConfig {
                    small_tick: 30,
                    medium_tick: 10,
                    big_tick: 120,
                    vel1_period: 20,
                    vel2_period: 200,
                },
            );
            let ticks = collector::loader::load_rows(&path);

            let mut run_cfg = BackRunConfig {
                balance: 100_000,
                pairs_conf: vec![pair_cfg],
                ticks,
                pair: Pair::EURUSD,
                week_id: i,
                print: false,
                report: true,
                report_cfg: BackReportConf {
                    report_folder: "../trader5_out/".to_string(),
                    report_sub_folder: format!("{}", sub_folder_time),
                },
            };
            let x = run_cfg.run_brain4();

            // collect balance
            bal.push(x.free_usd);

            // Print as we go
            {
                let p = x.free_usd - 100_000.;
                if p > 0. {
                    weeks_up += 1;
                } else {
                    weeks_down += 1;
                }
                sum += p;
                sum_abs += p.abs();
                println!(
                    "{}   {:.1}  {:.1}%    Sum: ({:.0}/{:.0})    {:.1}%     weeks(up/down) ({}/{})",
                    tsv,
                    p,
                    p / 10.,
                    sum,
                    sum_abs,
                    sum * 100. / sum_abs,
                    weeks_up,
                    weeks_down
                );
            }
        }
    }
    println!("{:#?}", bal);
    println!(
        "Sum: {:}            weeks(up/down) ({}/{}) ",
        sum, weeks_up, weeks_down
    );
}
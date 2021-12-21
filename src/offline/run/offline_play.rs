use super::*;

use crate::brain::Brain;
use crate::candle::CandleConfig;
use crate::collector;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::helper::get_time_sec;
use crate::offline::{BackReportConf, BackendEngine, BackendEngineOuter};
use std::sync::Arc;

pub fn run1() {
    let pair_cfg = (
        Pair::EURUSD,
        CandleConfig {
            small_tick: 30,
            medium_tick: 10,
            big_tick: 120,
            vel_period: 25,
        },
    );
    // let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/1.tsv");
    // let ticks = collector::loader::load_all_pair(&Pair::EURUSD, 44..50);
    let ticks = collector::loader::load_week(&Pair::EURUSD, 49);
    // let ticks = collector::loader::load_all_pair(&Pair::EURUSD, 44..45);

    let mut run_cfg = BackRunConfig {
        balance: 100_000,
        pairs_conf: vec![pair_cfg],
        ticks,
        week_id: 0,
        print: true,
        report: true,
        report_cfg: BackReportConf {
            report_folder: "../trader4_out/".to_string(),
            report_sub_folder: "".to_string(),
        },
    };

    run_cfg.run();
}

pub fn run_optimized() {
    let mut bal = vec![];
    let mut sum = 0.;
    let mut sum_abs = 0.;

    let mut sub_folder_time = get_time_sec();
    for i in 25..=53 {
        let tsv = format!("{:?}/{}.tsv", Pair::EURUSD, i);
        let path = format!("/mnt/c/me/data/{}", tsv);
        if std::path::Path::new(&path).exists() {
            let pair_cfg = (
                Pair::EURUSD,
                CandleConfig {
                    small_tick: 30,
                    medium_tick: 10,
                    big_tick: 120,
                    vel_period: 25,
                },
            );
            let ticks = collector::loader::load_rows(&path);

            let mut run_cfg = BackRunConfig {
                balance: 100_000,
                pairs_conf: vec![pair_cfg],
                ticks,
                week_id: i,
                print: false,
                report: true,
                report_cfg: BackReportConf {
                    report_folder: "../trader4_out/".to_string(),
                    report_sub_folder: format!("{}", sub_folder_time),
                },
            };
            let x = run_cfg.run();

            // collect balance
            bal.push(x.free_usd);

            // Print as we go
            {
                let p = x.free_usd - 100_000.;
                sum += p;
                sum_abs += p.abs();
                println!("{}   {:.1}  {:.1}%    Sum: ({:.0}/{:.0})    {:.1}%", tsv, p, p / 10., sum, sum_abs, sum * 100./ sum_abs);
            }
        }
    }
    println!("{:#?}", bal);
    println!("Sum: {:}", sum);
}

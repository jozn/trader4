use super::*;

use crate::brain::Brain;
use crate::candle::CandleConfig;
use crate::collector;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::{BackendEngine, BackendEngineOuter};
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
    let ticks = collector::loader::load_all_pair(&Pair::EURUSD, 40..45);

    let mut run_cfg = BackRunConfig {
        balance: 100_000,
        pairs_conf: vec![pair_cfg],
        ticks,
        report: true,
    };

    run_cfg.run();
}

pub fn run_optimized() {
    let mut bal = vec![];
    let mut sum = 0.;

    for i in 1..=53 {
        let path = format!("/mnt/c/me/data/{:?}/{}.tsv", Pair::EURUSD, i);
        if std::path::Path::new(&path).exists() {
            let pair_cfg = (
                Pair::EURUSD,
                CandleConfig {
                    small_tick: 30,
                    medium_tick: 10,
                    big_tick: 30,
                    vel_period: 25,
                },
            );
            let ticks = collector::loader::load_rows(&path);

            let mut run_cfg = BackRunConfig {
                balance: 100_000,
                pairs_conf: vec![pair_cfg],
                ticks,
                report: true,
            };
            let x = run_cfg.run();

            // collect balance
            bal.push(x.free_usd);

            // Print as we go
            {
                let p = x.free_usd - 100_000.;
                sum += p;
                println!(
                    ">>>          {:.1}             {:.1}%       Sum:{:.0}",
                    p,
                    p / 10.,
                    sum
                );
            }
        }
    }
    println!("{:#?}", bal);
    println!("Sum: {:}", sum);
}

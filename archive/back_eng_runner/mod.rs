pub mod back_runner;

use crate::brain::Brain;
use crate::candle::CandleConfig;
use crate::collector;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::{BackendEngine, BackendEngineOuter};
use std::sync::Arc;

pub fn run1() {
    let backend = BackendEngineOuter::new(100_000);
    let mut back_arc = Arc::new(backend);
    let pair_cfg = (
        Pair::EURUSD,
        CandleConfig {
            small_tick: 30,
            medium_tick: 10,
            big_tick: 120,
            vel_period: 25,
        },
    );
    let mut brain = Brain::new(back_arc.clone(), vec![pair_cfg]);

    // let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/1.tsv");
    let ticks = collector::loader::load_all_pair(&Pair::EURUSD, 40..45);

    for (i, t) in ticks.iter().enumerate() {
        if i % 10000 == 0 {
            // println!("{}", i);
        }
        back_arc.next_tick(1, t.clone());
        brain.on_price_tick(1, t.to_tick());
        let notifys = back_arc.take_notify();
        for not in notifys {
            brain.on_notify_position(not);
        }
    }
    let mut x = back_arc.engine.borrow_mut();
    x.close_all_positions();
    println!("{:#?}", x);
    println!("{:#?}", x.free_usd);

    x.report_to_folder("vdsd");
}

pub fn run2() {
    let mut bal = vec![];

    for i in 1..=53 {
        let path = format!("/mnt/c/me/data/{:?}/{}.tsv", Pair::EURUSD, i);
        if std::path::Path::new(&path).exists() {
            let backend = BackendEngineOuter::new(100_000);
            let mut back_arc = Arc::new(backend);
            let mut brain = Brain {
                con: Box::new(back_arc.clone()),
                db: vec![],
                last_trade_time: 0,
                acted: Default::default(),
            };

            let ticks = collector::loader::load_rows(&path);

            for (i, t) in ticks.iter().enumerate() {
                back_arc.next_tick(1, t.clone());
                brain.on_price_tick(1, t.to_tick())
            }
            let mut x = back_arc.engine.borrow_mut();
            x.close_all_positions();
            bal.push(x.free_usd)
        }
    }
    println!("{:#?}", bal);
    let mut sum = 0.;

    for b in bal {
        let p = b - 100_000.;
        sum += p;
        println!(">>>          {:}                     {}%", p, p / 10.);
    }
    println!("Sum: {:}", sum);
}

pub fn run_optimized() {
    let mut bal = vec![];
    let mut sum = 0.;

    for i in 1..=53 {
        let path = format!("/mnt/c/me/data/{:?}/{}.tsv", Pair::EURUSD, i);
        if std::path::Path::new(&path).exists() {
            let backend = BackendEngineOuter::new(100_000);
            let mut back_arc = Arc::new(backend);

            let pair_cfg = (
                Pair::EURUSD,
                CandleConfig {
                    // small_tick: 10,
                    // medium_tick: 24,
                    // big_tick: 80,
                    // vel_period: 37,
                    // small_tick: 24,
                    // medium_tick: 10,
                    // big_tick: 30,
                    // vel_period: 47,
                    small_tick: 30,
                    medium_tick: 10,
                    big_tick: 30,
                    vel_period: 25,
                },
            );
            let mut brain = Brain::new(back_arc.clone(), vec![pair_cfg]);

            let ticks = collector::loader::load_rows(&path);

            for (i, t) in ticks.iter().enumerate() {
                back_arc.next_tick(1, t.clone());
                brain.on_price_tick(1, t.to_tick())
            }
            let mut x = back_arc.engine.borrow_mut();
            x.close_all_positions();
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

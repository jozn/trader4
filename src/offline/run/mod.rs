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
            // good
            // small_tick: 8,
            // medium_tick: 16,
            // big_tick: 80,
            // small_tick: 16,
            // medium_tick: 8,
            // big_tick: 35,
            // good
            // small_tick: 24,
            // medium_tick: 6,
            // big_tick: 30,
            // small_tick: 44,
            // medium_tick: 4,
            // big_tick: 20,
            small_tick: 19,
            medium_tick: 23,
            big_tick: 85,
        },
    );
    let mut brain = Brain::new(back_arc.clone(), vec![pair_cfg]);

    let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/40.tsv");

    for (i, t) in ticks.iter().enumerate() {
        if i % 10000 == 0 {
            // println!("{}", i);
        }
        back_arc.next_tick(1, t.clone());
        brain.on_price_tick(1, t.to_tick())
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

    for i in 40..=53 {
        let path = format!("/mnt/c/me/data/{:?}/{}.tsv", Pair::EURUSD, i);
        if std::path::Path::new(&path).exists() {
            let backend = BackendEngineOuter::new(100_000);
            let mut back_arc = Arc::new(backend);

            let pair_cfg = (
                Pair::EURUSD,
                CandleConfig {
                    // small_tick: 49,
                    // medium_tick: 9,
                    // big_tick: 25,
                    // small_tick: 31,
                    // medium_tick: 7,
                    // big_tick: 17,
                    // small_tick: 46,
                    // medium_tick: 9,
                    // big_tick: 65,
                    // small_tick: 16,
                    // medium_tick: 11,
                    // big_tick: 37,
                    // small_tick: 49,
                    // medium_tick: 9,
                    // big_tick: 65,
                    // small_tick: 10,
                    // medium_tick: 13,
                    // big_tick: 49,
                    // small_tick: 16,
                    // medium_tick: 30,
                    // big_tick: 60,
                    // small_tick: 20,
                    // medium_tick: 22,
                    // big_tick: 30,
                    // good
                    // small_tick: 8,
                    // medium_tick: 16,
                    // big_tick: 80,
                    // small_tick: 16,
                    // medium_tick: 8,
                    // big_tick: 35,
                    // good
                    // small_tick: 24,
                    // medium_tick: 6,
                    // big_tick: 30,
                    small_tick: 44,
                    medium_tick: 4,
                    big_tick: 20,
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

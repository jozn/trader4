use crate::brain::Brain;
use crate::collector;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::{BackendEngine, BackendEngineOuter};
use std::sync::Arc;

pub fn run1() {
    let backend = BackendEngineOuter::new(100_000);
    let mut back_arc = Arc::new(backend);
    let mut brain = Brain {
        con: Box::new(back_arc.clone()),
        db: vec![],
        acted: Default::default(),
    };

    let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/45.tsv");

    for (i, t) in ticks.iter().enumerate() {
        if i % 10000 == 0 {
            // println!("{}", i);
        }
        back_arc.next_tick(1, t.clone());
        brain.on_price_tick_dep(1, t.to_tick())
    }
    let x = back_arc.engine.borrow();
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
                brain.on_price_tick_dep(1, t.to_tick())
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

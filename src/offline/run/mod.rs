use crate::brain::Brain;
use crate::collector;
use crate::gate_api::GateWay;
use crate::offline::{BackendEngine, BackendEngineOuter};
use std::sync::Arc;

pub fn run1() {
    let backend = BackendEngineOuter::new(100_000);
    let back_arc = Arc::new(backend);
    let brain = Brain {
        con: Box::new(back_arc.clone()),
        db: vec![],
        acted: Default::default(),
    };

    let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/16.tsv");

    for m in 1..100_000 {
        if m % 100 == 0 {
            println!("{}", m);
        }
        for (i, t) in ticks.iter().enumerate() {
            if i % 10000 == 0 {
                // println!("{}", i);
            }
            back_arc.next_tick(1, t.clone());
        }
    }
}

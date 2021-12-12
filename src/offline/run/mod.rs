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

    for i in 1..50 {
        let path = format!("/mnt/c/me/data/EURUSD/{}.tsv", i);
        let ticks = collector::loader::load_rows(&path);
        println!("{} - {}", i, ticks.len())
    }

    let ticks = collector::loader::load_rows("/mnt/c/me/data/EURUSD/16.tsv");

    for t in ticks {
        back_arc.next_tick(1, t);
    }
}

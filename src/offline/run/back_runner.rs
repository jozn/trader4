use crate::brain::{Brain, PairCandleCfg};
use crate::candle::CandleConfig;
use crate::collector;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::{BackendEngine, BackendEngineOuter};
use std::sync::Arc;

pub struct BackRunConfig {
    pub balance: i64,
    pub pairs_conf: Vec<PairCandleCfg>,
    pub ticks: Vec<BTickData>,
    pub report: bool,
}

pub struct BackRunRes {
    pub free_usd: f64,
}

impl BackRunConfig {
    pub fn run(mut self) -> BackRunRes {
        let backend = BackendEngineOuter::new(self.balance);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain::new(back_arc.clone(), self.pairs_conf);

        for (i, t) in self.ticks.iter().enumerate() {
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

        if self.report {
            x.report_to_folder("vdsd");
        }
        BackRunRes {
            free_usd: x.free_usd,
        }
    }
}

use crate::brain5::*;
use crate::candle::CandleConfig;
use crate::collector;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline2::*;
use std::sync::Arc;

pub struct BackRunConfig {
    pub balance: f64,
    pub pairs_conf: Vec<PairCandleCfg>,
    pub ticks: Vec<BTickData>,
    pub pair: Pair,
    pub week_id: u16,
    pub print: bool,
    pub report: bool,
    pub report_cfg: BackReportConf,
}

pub struct BackRunRes {
    pub free_usd: f64,
}

impl BackRunConfig {
    pub fn run_brain4(mut self) -> BackRunRes {
        let backend = BackendEngineOuter::new(self.balance, &self.report_cfg);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain5::new(back_arc.clone(), self.pairs_conf.first().unwrap().clone());
        let pair = self.pair.clone();
        for (i, t) in self.ticks.iter().enumerate() {
            if i % 10000 == 0 {
                // println!("{}", i);
            }
            back_arc.next_tick(t.clone());
            // brain.on_price_tick_NE(1, t.to_tick());
            // brain.on_price_tick(1, t.to_tick());
            // brain.on_price_tick_ne_dc_v3(1, t.to_tick());
            brain.on_price_tick_ne_dc_v4(&pair, t.to_tick());
            let notifys = back_arc.take_notify();
            for not in notifys {
                brain.on_notify_position(not);
            }
        }
        let mut x = back_arc.engine.borrow_mut();
        x.close_all_positions();

        let money = x.get_money();
        if self.print {
            // println!("{:#?}", x);
            println!("{:#?}", money.balance);
        }

        if self.report {
            x.report_to_folder(&format!("_week_{}_{}", self.week_id, self.pair.to_string()));
        }
        BackRunRes {
            free_usd: x.balance,
        }
    }
}

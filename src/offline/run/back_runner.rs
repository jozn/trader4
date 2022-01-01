use crate::brain1::{Brain1, PairCandleCfg};
use crate::brain2::Brain2;
use crate::brain3::Brain3;
use crate::candle::CandleConfig;
use crate::collector;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::{BackReportConf, BackendEngine, BackendEngineOuter};
use std::sync::Arc;

pub struct BackRunConfig {
    pub balance: i64,
    pub pairs_conf: Vec<PairCandleCfg>,
    pub ticks: Vec<BTickData>,
    pub week_id: u16,
    pub print: bool,
    pub report: bool,
    pub report_cfg: BackReportConf,
}

pub struct BackRunRes {
    pub free_usd: f64,
}

impl BackRunConfig {
    pub fn run_brain1(mut self) -> BackRunRes {
        let backend = BackendEngineOuter::new(self.balance, &self.report_cfg);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain1::new(back_arc.clone(), self.pairs_conf);

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

        if self.print {
            println!("{:#?}", x);
            println!("{:#?}", x.free_usd);
        }

        if self.report {
            x.report_to_folder(&format!("_week_{}", self.week_id));
        }
        BackRunRes {
            free_usd: x.free_usd,
        }
    }

    // TEMP
    pub fn run_brain2(mut self) -> BackRunRes {
        let backend = BackendEngineOuter::new(self.balance, &self.report_cfg);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain2::new(back_arc.clone(), self.pairs_conf.first().unwrap().clone());

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

        if self.print {
            println!("{:#?}", x);
            println!("{:#?}", x.free_usd);
        }

        if self.report {
            x.report_to_folder(&format!("_week_{}", self.week_id));
        }
        BackRunRes {
            free_usd: x.free_usd,
        }
    }

    // TEMP
    pub fn run_brain3(mut self) -> BackRunRes {
        let backend = BackendEngineOuter::new(self.balance, &self.report_cfg);
        let mut back_arc = Arc::new(backend);
        let mut brain = Brain3::new(back_arc.clone(), self.pairs_conf.first().unwrap().clone());

        for (i, t) in self.ticks.iter().enumerate() {
            if i % 10000 == 0 {
                // println!("{}", i);
            }
            back_arc.next_tick(1, t.clone());
            // brain.on_price_tick_NE(1, t.to_tick());
            // brain.on_price_tick(1, t.to_tick());
            brain.on_price_tick_ne_dc_v2(1, t.to_tick());
            let notifys = back_arc.take_notify();
            for not in notifys {
                brain.on_notify_position(not);
            }
        }
        let mut x = back_arc.engine.borrow_mut();
        x.close_all_positions();

        if self.print {
            // println!("{:#?}", x);
            println!("{:#?}", x.free_usd);
        }

        if self.report {
            x.report_to_folder(&format!("_week_{}", self.week_id));
        }
        BackRunRes {
            free_usd: x.free_usd,
        }
    }
}

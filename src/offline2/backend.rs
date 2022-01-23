use chrono::Weekday::Mon;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::report::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::core::gate_api::*;
use crate::gate_api::{GateWay, PosResDep};

use super::*;

#[derive(Debug)]
pub struct Money {
    pub balance: f64,
    pub equity: f64,
    // pub margin: f64,
    // pub free_margin: f64,
    pub locked: f64,
    pub free_balance: f64,
    pub net_pl: f64,
}

// todo add a fn to return an struc of availabe money, fee margin,...

// type PricePair = (Pair, BTickData);
#[derive(Debug)]
pub struct BackendEngine {
    pub deposit: f64,
    pub leverage: i64,
    pub balance: f64,
    pub symbols: Vec<Pair>,
    pub ticks: BTreeMap<i64, BTickData>,
    pub las_time_ms: u64,
    pub pos_id: u64,
    // pub free_usd: f64,
    pub opens: BTreeMap<u64, Position>,
    pub closed: BTreeMap<u64, Position>,
    pub events: Vec<EventPosition>,
    pub report: Report,
}

impl BackendEngine {
    pub fn new(fund: f64, report_cfg: &BackReportConf) -> Self {
        Self {
            deposit: fund,
            leverage: 100,
            balance: fund,
            symbols: vec![],
            ticks: Default::default(),
            las_time_ms: 0,
            pos_id: 0,
            // free_usd: 0.0,
            opens: Default::default(),
            closed: Default::default(),
            events: vec![],
            report: Report::new(report_cfg),
        }
    }

    fn subscribe_pairs_req(&mut self, symbols: Vec<Pair>) {
        self.symbols = symbols;
    }

    fn open_position_req_new(&mut self, param: &NewPos) {
        let money = self.get_money();
        // self.report.collect_balance(&money);
        self.report_balance();
        self.report.on_new_trade(param, &money);

        // let quote_asset =
        // if !self.has_enough_balance(param.base_asset_size) {
        //     return;
        // }
        let sid = param.pair.to_symbol_id();
        // self.pos_id += 1;
        // println!("buy long long");
        let tick = self.get_symbol_tick(sid).unwrap();
        let new_p = NewPosInter {
            new_pos: param.clone(),
            tick,
            locked: self.get_money().locked,
            time: self.las_time_ms,
            pos_id: self.pos_id,
        };

        let mut pos = Position::new(new_p);
        if pos.is_long() {
            if !self.has_enough_balance(pos.quote_asset_size) {
                return;
            }
            self.pos_id += 1;
            pos.pos_id = self.pos_id;
        }
        self.events.push(pos.to_event());
        self.opens.insert(pos.pos_id, pos.clone());
    }

    pub fn next_tick(&mut self, btick: BTickData) {
        let symbol_id = btick.pair.to_symbol_id();
        if self.las_time_ms < btick.timestamp as u64 {
            self.las_time_ms = btick.timestamp as u64;
        }
        self.ticks.insert(symbol_id, btick.clone());

        // update touch prices
        for (_, pos) in self.opens.iter_mut() {
            if pos.symbol_id == symbol_id {
                pos.update_touch_prices(&btick);
            }
        }
        self.close_stasfied_poss(symbol_id, false);
    }

    fn update_position(&mut self, req: &UpdatePos) {
        // todo migrate from
    }

    fn close_stasfied_poss(&mut self, symob_id: i64, force: bool) {
        let btick = self.get_symbol_tick(symob_id);
        if btick.is_none() {
            return;
        }
        let btick = btick.unwrap();
        let mut remove_pos_ids = vec![];
        let mut closed_some = false;

        for (_, pos) in self.opens.iter_mut() {
            if pos.symbol_id == symob_id {
                if pos.should_close(&btick) || force {
                    let cp = CloseParm {
                        pair: btick.pair.clone(),
                        tick: btick.clone(),
                        locked: 0.,
                        time_sec: self.las_time_ms / 1000,
                    };
                    pos.close_pos(&cp);

                    closed_some = true;
                    remove_pos_ids.push(pos.pos_id);

                    if pos.is_long() {
                        self.balance += pos.profit;
                    }

                    self.closed.insert(pos.pos_id, pos.clone());
                    self.events.push(pos.to_event());
                }
            }
        }

        for pid in remove_pos_ids {
            self.opens.remove(&pid);
        }
        if closed_some {
            self.report_balance();
        }
    }

    // Privates
    fn get_symbol_tick(&self, symbol_id: i64) -> Option<BTickData> {
        let res = self.ticks.get(&symbol_id);
        match res {
            None => None,
            Some(t) => Some(t.clone()),
        }
    }

    // Utils
    fn has_enough_balance(&self, usd_vol: f64) -> bool {
        let money = self.get_money();
        if money.free_balance > usd_vol {
            true
        } else {
            false
        }
    }

    pub fn close_all_positions(&mut self) {
        self.report_balance();
        let ids = assets::get_all_symbols_ids();
        for id in ids {
            self.close_stasfied_poss(id, true);
        }
        self.report_balance();
    }

    fn get_money(&self) -> Money {
        let mut long_debt = 0.0;
        let mut long_pl = 0.0;
        let mut _short_debt = 0.0;

        for (_, pos) in self.opens.iter() {
            if pos.is_long() {
                long_debt = pos.quote_asset_size;
                let tick = self.get_symbol_tick(pos.symbol_id).unwrap();

                // Close a copy for profit calcualtion
                let mut pos_cp = pos.clone();
                let cp = CloseParm {
                    pair: tick.pair.clone(),
                    tick: tick.clone(),
                    locked: 0.,
                    time_sec: self.las_time_ms / 1000,
                };
                pos_cp.close_pos(&cp);
                long_pl += pos_cp.profit;
            }
        }

        // self.free_usd - short_debt;

        let balance = self.balance;
        let equity = self.balance + long_pl;
        let net_pl = long_pl;
        let locked = long_debt;
        let free_balance = self.balance - locked;

        Money {
            balance,
            equity,
            locked,
            free_balance,
            net_pl,
        }
    }

    // Reports
    fn report_balance(&mut self) {
        self.report.collect_balance(&self.get_money());
    }

    pub fn get_report_summery(&self) -> ReportSummery {
        self.report.get_report_summery(&self)
    }

    pub fn report_to_folder(&mut self, suffix: &str) {
        self.report.write_to_folder(&self, suffix);
    }
}

#[derive(Debug)]
pub struct BackendEngineOuter {
    pub engine: RefCell<BackendEngine>, // Could be Mutex too
}

impl BackendEngineOuter {
    pub fn new(fund: f64, cfg: &BackReportConf) -> Self {
        Self {
            engine: RefCell::new(BackendEngine::new(fund, cfg)),
        }
    }

    pub fn next_tick(&self, btick: BTickData) {
        let mut eng = self.engine.borrow_mut();
        eng.next_tick(btick);
    }

    pub fn take_notify(&self) -> Vec<EventPosition> {
        let mut eng = self.engine.borrow_mut();
        let res = eng.events.clone();
        eng.events.clear();
        res
    }
}

impl GateWay for BackendEngineOuter {
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>) {
        let mut x = self.engine.borrow_mut();
        x.subscribe_pairs_req(symbols);
    }

    fn open_position_req_new(&self, new_pos: &NewPos) {
        let mut x = self.engine.borrow_mut();
        x.open_position_req_new(new_pos);
    }

    fn update_position(&self, update: &UpdatePos) {
        let mut x = self.engine.borrow_mut();
        x.update_position(update);
    }

    fn get_time_ms(&self) -> u64 {
        let mut x = self.engine.borrow_mut();
        x.las_time_ms
    }
}

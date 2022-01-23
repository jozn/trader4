use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::collector::row_data::BTickData;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::core::gate_api::*;
use crate::gate_api::{GateWay, PosResDep};
use crate::offline::report::{Report, ReportSummery};

use super::*;

// todo add a fn to return an struc of availabe money, fee margin,...

// type PricePair = (Pair, BTickData);
#[derive(Debug)]
pub struct BackendEngine {
    pub balance: i64,
    pub symbols: Vec<Pair>,
    pub ticks: BTreeMap<i64, BTickData>,
    pub las_time_ms: u64,
    pub pos_id: u64,
    pub free_usd: f64,
    pub opens: BTreeMap<u64, Position>,
    pub closed: BTreeMap<u64, Position>,
    pub events: Vec<EventPosition>,
    // pub report: Report,
}

impl BackendEngine {
    pub fn new(fund: i64, report_cfg: &BackReportConf) -> Self {
        Self {
            balance: 0,
            symbols: vec![],
            ticks: Default::default(),
            las_time_ms: 0,
            pos_id: 0,
            free_usd: 0.0,
            opens: Default::default(),
            closed: Default::default(),
            events: vec![],
        }
    }

    fn subscribe_pairs_req(&mut self, symbols: Vec<Pair>) {
        self.symbols = symbols;
    }

    fn open_position_req_new(&mut self, param: &NewPos) {
        // self.report_balance();
        // self.report
        //     .on_new_trade(new_pos, self.get_total_balance(), self.get_locked_money());

        if !self.has_enough_balance(param.base_asset_size) {
            return;
        }
        let sid = param.pair.to_symbol_id();
        self.pos_id += 1;
        // println!("buy long long");
        let tick = self.get_symbol_tick(sid).unwrap();
        let new_p = NewPosInter {
            new_pos: param.clone(),
            tick,
            locked: self.get_locked_money(),
            time: self.las_time_ms,
            pos_id: self.pos_id,
        };

        let mut pos = Position::new(new_p);
        if pos.is_long() {
            self.free_usd -= pos.quote_asset_size as f64;
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
                if pos.should_close(&btick) {
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
                        self.free_usd += pos.quote_asset_size;
                        self.free_usd += pos.profit;
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
            // self.report_balance();
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
        let free = self.get_free_balance();
        if free > usd_vol {
            true
        } else {
            false
        }
    }

    fn get_free_balance(&self) -> f64 {
        let mut short_debt = 0.0;
        // for p in self.opens.iter() {
        //     if p.is_short() {
        //         short_debt += p.pos_size_usd;
        //     }
        // }
        self.free_usd - short_debt
    }

    fn get_locked_money(&self) -> f64 {
        // let mut locked_money = 0.0;
        // for p in self.opens.iter() {
        //     locked_money += p.pos_size_usd as f64
        // }
        // locked_money
        0.
    }
}

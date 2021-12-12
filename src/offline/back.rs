use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::core::gate_api::NewPos;
use crate::gate_api::GateWay;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

type PricePair = (Pair, BTickData);
// need: loader, report from offline as it is
#[derive(Debug)]
pub struct BackendEngine {
    pub balance: i64,
    pub symbols: Vec<Pair>,
    pub price: Vec<PricePair>,
    pub las_time_ms: u64,
    pub pos_id: u64,
    pub free_usd: f64,
    pub opens: Vec<Position>,
    pub closed: Vec<Position>,
}

impl BackendEngine {
    // Direct GateWay api calls
    fn subscribe_pairs_req(&mut self, symbols: Vec<Pair>) {
        self.symbols = symbols;
    }

    fn open_position_req_new(&mut self, new_pos: &NewPos) {
        if new_pos.is_short {
            self.sell_short(new_pos)
        } else {
            self.buy_long(new_pos)
        }
    }

    fn update_position(&self) {
        todo!()
    }

    fn get_time_ms(&self) -> u64 {
        self.las_time_ms
    }

    // From script/bot calls
    // todo: optimize with multi bticks per call
    pub fn next_tick(&self, symbol_id: u64, btick: BTickData) {
        // set last time, symobl price, close postions
    }

    // Privates
    fn buy_long(&mut self, param: &NewPos) {
        let usd_vol = param.get_usd();
        if !self.has_enough_balance(usd_vol) {
            return;
        }
        // println!("buy long long");

        let mut pos = Position::new_long(param);

        // self.report.on_new_trade(&pos, self.get_total_balance(param.price));

        // self.free_usd -= usd as f64 * 1000.;
        self.free_usd -= param.get_usd();

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    fn close_long(&mut self, param: &NewPos) {
        let pos = self.opens.iter().find(|p| p.pos_id == param.pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                // p.close_pos(param.price, param.time);
                p.close_pos(param);

                // self.report.on_close_trade(&p, self.get_total_balance(param.price));

                let got_usd = p.final_balance;
                self.free_usd += got_usd;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    fn sell_short(&mut self, param: &NewPos) {
        let usd_vol = param.get_usd();
        if !self.has_enough_balance(usd_vol) {
            return;
        }

        let mut pos = Position::new_short(param);

        // self.report.on_new_trade(&pos, self.get_total_balance(param.price));

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    fn close_short(&mut self, param: &NewPos) {
        let pos = self.opens.iter().find(|p| p.pos_id == param.pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                // p.close_pos(param.price, param.time);
                p.close_pos(param);

                // self.report.on_close_trade(&p, self.get_total_balance(param.price));

                let got_coin = p.final_balance;
                self.free_usd += p.profit;

                // self.free_asset += got_coin;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    // Close
    pub fn try_close_satasfied_postions(&mut self, param: &NewPos) -> bool {
        let mut done = false;
        for p in self.opens.clone().iter_mut() {
            let mut param2 = param.clone();
            param2.pos_id = p.pos_id;

            p.update_ailing(param.price);

            if p.should_close_bk_simple(param.price) {
                match p.direction {
                    PosDir::Long => {
                        done = true;
                        self.close_long(&param2);
                    }
                    PosDir::Short => {
                        done = true;
                        self.close_short(&param2);
                    }
                }
            }
        }
        done
    }

    // Close
    pub fn close_all_positions(&mut self, param: &NewPos) {
        for p in self.opens.clone().iter() {
            let mut param2 = param.clone();
            param2.pos_id = p.pos_id;
            match p.direction {
                PosDir::Long => {
                    self.close_long(&param2);
                }
                PosDir::Short => {
                    self.close_short(&param2);
                }
            }
        }
    }
    // Utils
    fn has_enough_balance(&self, usd_vol: f64) -> bool {
        let b = self.get_free_balance();
        let res = if b > usd_vol { true } else { false };
        res
    }

    fn get_free_balance(&self) -> f64 {
        let mut short_debt = 0.0;
        for (i, p) in self.opens.iter().enumerate() {
            if p.direction == PosDir::Short {
                short_debt += p.pos_size_usd;
            }
        }
        self.free_usd - short_debt
    }

    pub fn update_pos(&mut self, pos: &mut Position) {
        self._remove_pos(pos.pos_id);
        self.opens.push(pos.clone());
    }

    fn _close_pos(&mut self, pos: &mut Position) {
        self._remove_pos(pos.pos_id);
        self.closed.push(pos.clone());
    }

    fn next_pos_id(&mut self) -> u64 {
        self.pos_id += 1;
        self.pos_id
    }

    // Remove from both open and closed position vector.
    fn _remove_pos(&mut self, pos_id: u64) {
        let mut idx = -1_i32;
        for (i, p) in self.opens.iter().enumerate() {
            if p.pos_id == pos_id {
                idx = i as i32;
            }
        }
        if idx >= 0 {
            self.opens.remove(idx as usize);
        }

        let mut idx = -1_i32;
        for (i, p) in self.closed.iter().enumerate() {
            if p.pos_id == pos_id {
                idx = i as i32;
            }
        }
        if idx >= 0 {
            self.opens.remove(idx as usize);
        }
    }
}

#[derive(Debug)]
pub struct BackendEngineOuter {
    pub engine: Mutex<BackendEngine>,
}

impl BackendEngineOuter {
    pub fn new(fund: i64) -> Self {
        Self {
            engine: Mutex::new(BackendEngine {
                balance: fund,
                symbols: vec![],
                price: vec![],
                las_time_ms: 0,
                pos_id: 0,
                free_usd: fund as f64,
                opens: vec![],
                closed: vec![],
            }),
        }
    }

    pub fn next_tick(&self, symbol_id: u64, btick: BTickData) {
        let mut locked_eng = self.engine.lock().unwrap();
        locked_eng.next_tick(symbol_id, btick);
    }
}
impl GateWay for BackendEngineOuter {
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>) {
        let mut locked_eng = self.engine.lock().unwrap();
        // symbols.iter().for_each(|p| locked_eng.borrow_mut().symbols.push(p.clone()));
        locked_eng.symbols = symbols;
    }

    fn open_position_req_new(&self, new_pos: &NewPos) {
        todo!()
    }

    fn update_position(&self) {
        todo!()
    }

    fn get_time_ms(&self) -> u64 {
        let mut locked_eng = self.engine.lock().unwrap();
        locked_eng.las_time_ms
    }
}

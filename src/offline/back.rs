use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::core::gate_api::NewPos;
use crate::gate_api::GateWay;
use std::borrow::BorrowMut;
use std::cell::RefCell;
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
    pub fn next_tick(&mut self, symbol_id: i64, btick: BTickData) {
        // set last time, symobl price, close postions
        self.las_time_ms = btick.timestamp as u64;
        let mut idx = -1;

        for mut r in self.price.iter().enumerate() {
            if r.1 .0.to_symbol_id() == symbol_id {
                idx = r.0 as i64;
            }
        }
        if idx >= 0 && self.price.len() > 0 {
            self.price.remove(idx as usize);
        }
        self.price.push((Pair::id_to_symbol(symbol_id), btick));
        self.close_stasfied_poss(symbol_id, false);
    }

    fn get_symbol_tick(&self, symbol_id: i64) -> Option<BTickData> {
        for r in self.price.iter() {
            if r.0.to_symbol_id() == symbol_id {
                return Some(r.1.clone());
            }
        }
        None
    }

    fn close_stasfied_poss(&mut self, symob_id: i64, force: bool) {
        let btick = self.get_symbol_tick(symob_id);
        if btick.is_none() {
            return;
        }
        let btick = btick.unwrap();
        let mut remove = vec![];

        for mut pos in self.opens.iter_mut() {
            if pos.symbol_id != symob_id {
                continue;
            }

            let price = btick.ask_price;
            // println!("+++++++++++++++++++ >> : {:#?}, {:?}", pos, btick);
            if price > pos.high_exit_price || price < pos.low_exit_price || force {
                let p = CloseParm {
                    at_price: price,
                    time: btick.timestamp_sec as u64,
                    ta: Default::default(),
                };
                pos.close_pos(&p);

                // println!("+++++++++++++++++++ closding pos : {:#?}", pos);

                remove.push(pos.pos_id);

                if pos.is_short() {
                    self.free_usd += pos.profit;
                } else {
                    self.free_usd += pos.pos_size_usd;
                    self.free_usd += pos.profit;
                }

                // self._remove_pos(pos.pos_id);
                self.closed.push(pos.clone());
            }
        }

        for pid in remove {
            self._remove_open_pos(pid);
        }
    }

    // Privates
    fn buy_long(&mut self, param: &NewPos) {
        if !self.has_enough_balance(param.size_usd) {
            return;
        }
        // println!("buy long long");

        let mut pos = Position::new(param);

        // self.report.on_new_trade(&pos, self.get_total_balance(param.price));

        // self.free_usd -= usd as f64 * 1000.;
        self.free_usd -= param.size_usd as f64;

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    fn sell_short(&mut self, param: &NewPos) {
        if !self.has_enough_balance(param.size_usd) {
            return;
        }

        let mut pos = Position::new(param);

        // self.report.on_new_trade(&pos, self.get_total_balance(param.price));

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    // Close
    pub fn close_all_positions(&mut self) {
        //todo
        let ids = assets::get_all_symbols_ids();
        for id in ids {
            self.close_stasfied_poss(id, true);
        }
    }
    // Utils
    fn has_enough_balance(&self, usd_vol: i64) -> bool {
        let b = self.get_free_balance() as i64;
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
    fn _remove_open_pos(&mut self, pos_id: u64) {
        let mut idx = -1_i32;
        for (i, p) in self.opens.iter().enumerate() {
            if p.pos_id == pos_id {
                idx = i as i32;
            }
        }
        if idx >= 0 {
            self.opens.remove(idx as usize);
        }
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
            self.closed.remove(idx as usize);
        }
    }
}

#[derive(Debug)]
pub struct BackendEngineOuter {
    pub engine: RefCell<BackendEngine>, // Could be Mutex too
}

impl BackendEngineOuter {
    pub fn new(fund: i64) -> Self {
        Self {
            engine: RefCell::new(BackendEngine {
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

    pub fn next_tick(&self, symbol_id: i64, btick: BTickData) {
        // let mut locked_eng = self.engine.lock().unwrap();
        // locked_eng.next_tick(symbol_id, btick);
        let mut eng = self.engine.borrow_mut();
        // println!("{:?}", btick.ask_price);
        eng.next_tick(symbol_id, btick);
    }
}
impl GateWay for BackendEngineOuter {
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>) {
        let mut x = self.engine.borrow_mut();
        x.subscribe_pairs_req(symbols);
        // let mut locked_eng = self.engine.lock().unwrap();
        // // symbols.iter().for_each(|p| locked_eng.borrow_mut().symbols.push(p.clone()));
        // locked_eng.symbols = symbols;
    }

    fn open_position_req_new(&self, new_pos: &NewPos) {
        // todo!()
        let mut x = self.engine.borrow_mut();
        // x.pos_id += 1;
        x.open_position_req_new(new_pos);
        // println!(">>>>>>>>>>>>>>>>>>>>>>> {}", x.pos_id)
    }

    fn update_position(&self) {
        todo!()
    }

    fn get_time_ms(&self) -> u64 {
        let mut x = self.engine.borrow_mut();
        x.las_time_ms
        // let mut locked_eng = self.engine.lock().unwrap();
        // locked_eng.las_time_ms
    }
}

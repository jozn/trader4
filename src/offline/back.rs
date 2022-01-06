use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::core::gate_api::{NewPos, UpdatePos};
use crate::gate_api::{GateWay, PosRes};
use crate::offline::report::{Report, ReportSummery};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

type PricePair = (Pair, BTickData);
#[derive(Debug)]
pub struct BackendEngine {
    pub balance: i64,
    pub symbols: Vec<Pair>,
    pub price: Vec<PricePair>, // todo replace with a Set/Map
    pub las_time_ms: u64,
    pub pos_id: u64,
    pub free_usd: f64,
    pub opens: Vec<Position>,
    pub closed: Vec<Position>,
    pub notify: Vec<PosRes>,
    pub report: Report,
}

impl BackendEngine {
    pub fn new(fund: i64, cfg: &BackReportConf) -> Self {
        Self {
            balance: fund,
            symbols: vec![],
            price: vec![],
            las_time_ms: 0,
            pos_id: 0,
            free_usd: fund as f64,
            opens: vec![],
            closed: vec![],
            notify: vec![],
            report: Report::new(cfg),
        }
    }
    // Direct GateWay api calls
    fn subscribe_pairs_req(&mut self, symbols: Vec<Pair>) {
        self.symbols = symbols;
    }

    fn open_position_req_new(&mut self, new_pos: &NewPos) {
        self.report_balance();
        self.report
            .on_new_trade(new_pos, self.get_total_balance(), self.get_locked_money());

        if new_pos.is_short {
            self.sell_short(new_pos)
        } else {
            self.buy_long(new_pos)
        }
    }

    fn update_position(&mut self, req: &UpdatePos) {
        // println!("update: {:?}", &req);
        let pos_opt = self._get_open_pos(req.pos_id as u64);
        match pos_opt {
            None => {}
            Some(mut pos) => {
                // println!("=======> update {:?}", pos);
                pos.updates += 1;
                if req.close {
                    let tick = self.get_symbol_tick(pos.symbol_id).unwrap();
                    let close_par = CloseParm {
                        at_price: tick.ask_price,
                        time: self.las_time_ms,
                        ta: Default::default(),
                    };
                    pos.close_pos(&close_par);
                    self._remove_open_pos(pos.pos_id as u64);
                    self.opens.push(pos);
                } else {
                    let (high, low) = if pos.is_short() {
                        (req.stop_loose_price, req.take_profit_price)
                    } else {
                        (req.take_profit_price, req.stop_loose_price)
                    };
                    if high > 0. {
                        pos.high_exit_price = high;
                    }
                    if low > 0. {
                        pos.low_exit_price = low;
                    }
                    self._remove_open_pos(pos.pos_id);
                    // commented to avoid forever loops
                    // self.notify.push(pos.to_notify());
                    self.opens.push(pos);
                }
            }
        }
    }

    fn get_time_ms(&self) -> u64 {
        self.las_time_ms
    }

    // From script/bot calls
    // todo: optimize with multi bticks per call
    pub fn next_tick(&mut self, symbol_id: i64, btick: BTickData) {
        // set last time, symobl price, close postions
        if self.las_time_ms < btick.timestamp as u64 {
            self.las_time_ms = btick.timestamp as u64;
        }
        let mut idx = -1;

        // Update symbol ticks data
        for mut r in self.price.iter().enumerate() {
            if r.1 .0.to_symbol_id() == symbol_id {
                idx = r.0 as i64;
            }
        }
        if idx >= 0 && self.price.len() > 0 {
            self.price.remove(idx as usize);
        }
        self.price
            .push((Pair::id_to_symbol(symbol_id), btick.clone()));

        self.update_touch_prices(symbol_id, &btick);
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

    fn update_touch_prices(&mut self, symob_id: i64, btick: &BTickData) {
        for mut pos in self.opens.iter_mut() {
            if pos.symbol_id != symob_id {
                continue;
            }

            let high = (btick.ask_price - pos.open_price) * 10_000.;
            if pos.touch_high_pip < high {
                pos.touch_high_pip = high;
            }

            let low = (btick.bid_price - pos.open_price) * 10_000.;
            if pos.touch_low_pip > low {
                pos.touch_low_pip = low;
            }
        }
    }

    fn close_stasfied_poss(&mut self, symob_id: i64, force: bool) {
        let btick = self.get_symbol_tick(symob_id);
        if btick.is_none() {
            return;
        }
        let btick = btick.unwrap();
        let mut remove_pos_ids = vec![];
        let mut closed_some = false;

        for mut pos in self.opens.iter_mut() {
            if pos.symbol_id != symob_id {
                continue;
            }

            let price = btick.ask_price;
            // println!("+++++++++++++++++++ >> : {:#?}, {:?}", pos, btick);
            if price >= pos.high_exit_price || price <= pos.low_exit_price || force {
                let p = CloseParm {
                    at_price: price,
                    time: btick.timestamp_sec as u64,
                    ta: Default::default(),
                };
                pos.close_pos(&p);
                closed_some = true;

                // println!("+++++++++++++++++++ closding pos : {:#?}", pos);

                remove_pos_ids.push(pos.pos_id);

                if pos.is_short() {
                    self.free_usd += pos.profit;
                } else {
                    self.free_usd += pos.pos_size_usd;
                    self.free_usd += pos.profit;
                }

                // self.report.on_close_trade(&pos,self.get_free_balance());

                // self._remove_pos(pos.pos_id);
                self.closed.push(pos.clone());
                self.notify.push(pos.to_notify());
            }
        }

        for pid in remove_pos_ids {
            self._remove_open_pos(pid);
        }

        if closed_some {
            self.report_balance();
        }
    }

    // Privates
    fn buy_long(&mut self, param: &NewPos) {
        if !self.has_enough_balance(param.size_usd) {
            return;
        }
        // println!("buy long long");
        let mut pos = Position::new(param, self.get_locked_money());
        self.free_usd -= param.size_usd as f64;
        pos.pos_id = self._next_pos_id();
        self.notify.push(pos.to_notify());
        self.opens.push(pos);
    }

    fn sell_short(&mut self, param: &NewPos) {
        if !self.has_enough_balance(param.size_usd) {
            return;
        }
        let mut pos = Position::new(param, self.get_locked_money());
        pos.pos_id = self._next_pos_id();
        self.notify.push(pos.to_notify());
        self.opens.push(pos);
    }

    pub fn close_all_positions(&mut self) {
        self.report_balance();
        let ids = assets::get_all_symbols_ids();
        for id in ids {
            self.close_stasfied_poss(id, true);
        }
        self.report_balance();
    }

    // Utils
    fn has_enough_balance(&self, usd_vol: i64) -> bool {
        let b = self.get_free_balance() as i64;
        let res = if b > usd_vol { true } else { false };
        res
    }

    fn get_total_balance(&self) -> f64 {
        let mut locked_usd = 0.;
        for pos in self.opens.iter() {
            let btick = self.get_symbol_tick(pos.symbol_id).unwrap();
            let mut pos_cp = pos.clone();
            let p = CloseParm {
                at_price: btick.bid_price,
                time: btick.timestamp_sec as u64,
                ta: Default::default(),
            };
            pos_cp.close_pos(&p);

            if pos_cp.is_short() {
                locked_usd += pos_cp.profit;
            } else {
                locked_usd += pos_cp.pos_size_usd;
                locked_usd += pos_cp.profit;
            }
        }

        self.free_usd + locked_usd
    }

    fn get_free_balance(&self) -> f64 {
        let mut short_debt = 0.0;
        for p in self.opens.iter() {
            if p.is_short() {
                short_debt += p.pos_size_usd;
            }
        }
        self.free_usd - short_debt
    }

    fn get_locked_money(&self) -> f64 {
        let mut locked_money = 0.0;
        for p in self.opens.iter() {
            locked_money += p.pos_size_usd as f64
        }
        locked_money
    }

    fn _next_pos_id(&mut self) -> u64 {
        self.pos_id += 1;
        self.pos_id
    }

    fn _get_open_pos(&self, pos_id: u64) -> Option<Position> {
        let mut res = None;
        for (i, p) in self.opens.iter().enumerate() {
            if p.pos_id == pos_id {
                res = Some(p.clone());
                break;
            }
        }
        res
    }

    // Remove from open position vector.
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
    fn _remove_all_pos(&mut self, pos_id: u64) {
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

    // Reports
    fn report_balance(&mut self) {
        self.report.collect_balance(self.get_total_balance());
    }

    pub fn get_report_summery(&self) -> ReportSummery {
        self.report.report_summery(&self)
    }

    pub fn report_to_folder(&mut self, suffix: &str) {
        self.closed.sort_by(|a, b| a.pos_id.cmp(&b.pos_id));
        self.report.write_to_folder(&self, suffix);
    }
}

#[derive(Debug)]
pub struct BackendEngineOuter {
    pub engine: RefCell<BackendEngine>, // Could be Mutex too
}

impl BackendEngineOuter {
    pub fn new(fund: i64, cfg: &BackReportConf) -> Self {
        Self {
            engine: RefCell::new(BackendEngine::new(fund, cfg)),
        }
    }

    pub fn next_tick(&self, symbol_id: i64, btick: BTickData) {
        let mut eng = self.engine.borrow_mut();
        eng.next_tick(symbol_id, btick);
    }

    pub fn take_notify(&self) -> Vec<PosRes> {
        let mut eng = self.engine.borrow_mut();
        let res = eng.notify.clone();
        eng.notify.clear();
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
        x.get_time_ms()
    }
}

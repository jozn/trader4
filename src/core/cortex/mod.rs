pub mod flags;
pub mod pair_mem;
pub mod sim_virtual;

pub use flags::*;
pub use pair_mem::*;
pub use sim_virtual::*;

use std::cell::{Cell, RefCell};

use crate::app;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::*;
use crate::sig_engs::ml_eng::MLFrameTradeInsight;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

// In order to get mut of this use:
//   let mut m = self.cortex.as_ref().borrow_mut();
pub type CortexRef = Rc<RefCell<Cortex>>;

pub fn new_cortex_ref() -> CortexRef {
    Rc::new(RefCell::new(Cortex::new()))
}

// Specs:
//  no gateway in cortex - only brain

#[derive(Debug)]
pub struct Cortex {
    pub flags: FlagsDB,
    pub pairs_mem: Vec<PairMemory>,
    pub sim_virtual: SimVirtual,
    pub open_pos: HashMap<u64, PosHolder>,
    pub closed_pos: HashMap<u64, PosHolder>,
    pub orders: f64,
    pub v_orders: f64,
    pub wallet: f64,
    pub policy: f64,
    pub time_table: f64,

    pub new_positions: Vec<NewPosReq>,
    pub update_positions: Vec<UpdatePosReq>,
}

impl Cortex {
    pub fn new() -> Self {
        Self {
            flags: Default::default(),
            pairs_mem: vec![],
            sim_virtual: SimVirtual::new(),
            open_pos: Default::default(),
            closed_pos: Default::default(),
            orders: 0.0,
            v_orders: 0.0,
            wallet: 0.0,
            policy: 0.0,
            time_table: 0.0,
            new_positions: vec![],
            update_positions: vec![],
        }
    }

    fn init_pair(&mut self, pair: &Pair) {
        if !self.pairs_mem.iter().any(|ps| &ps.pair == pair) {
            self.pairs_mem.push(PairMemory {
                pair: pair.clone(),
                last_tick: None,
            })
        }
    }

    pub fn on_price_tick(&mut self, pair: &Pair, tick: BTickData) {
        self.init_pair(pair);

        for pm in self.pairs_mem.iter_mut() {
            if &pm.pair == pair {
                pm.last_tick = Some(tick.clone());
            }
        }

        self.sim_virtual.run_next_tick(tick.clone());
    }

    pub fn run_on_tick_end(&mut self) {
        for np in self.new_positions.iter() {
            self.sim_virtual.open_position(&np, "sky_1");
        }
        for up in &self.update_positions {
            // self.sim_virtual.update_position(up);
        }
    }

    pub fn on_notify_position(&mut self, pos: EventPosition) {
        self._change_position(pos.clone());
    }

    fn _change_position(&mut self, pos: EventPosition) {
        let pos_id = pos.pos_id;
        if pos.is_closed {
            let pos_end = self.open_pos.remove(&pos.pos_id);
            match pos_end {
                None => {}
                Some(mut ph) => {
                    // println!("cortex: closing cortex postion {:?}", &ph);
                    /*
                      println!(
                        "cortex: closing postion {:?} -- open/closed cnt {}/{}",
                        pos_id,
                        self.open_pos.len(),
                        self.closed_pos.len()
                    );
                    */
                    ph.pos_res = pos; // update
                    self.closed_pos.insert(pos_id, ph.clone());
                }
            }
        } else {
            let mut res_opt = self.open_pos.get_mut(&pos.pos_id);
            match res_opt {
                None => {
                    let ph = PosHolder {
                        pos_res: pos.clone(),
                        profit_level: 0,
                    };
                    // println!("cortex: inserting cortex postion {:?}", &ph);
                    // println!("cortex: inserting postion {:?}", pos_id);
                    self.open_pos.insert(pos.pos_id, ph);
                }
                Some(ph) => {
                    ph.pos_res = pos; // update
                }
            }
        }
    }

    pub fn on_end(&mut self) {
        // Ignore big runtime dump
        let MAX = 300;
        let s = if self.closed_pos.len() > MAX || self.sim_virtual._closed.len() > MAX {
            let s = format!(
                "too big for print \n closed orders: {:#?}",
                self.closed_pos.len()
            );
            // return;
            s
        } else {
            let s = format!("{:#?}", self);
            s
        };
        // println!("{}",s);
        std::fs::write("./debug/runtime/cortex_dump.txt", s);
    }

    pub fn get_last_trade(&self, pair: Pair) -> LastTradeRes {
        let open = self._get_open_trade(pair);
        let closed = self._get_closed_trade(pair);
        let c = match open {
            None => {
                match closed {
                    None => LastTradeRes {
                        trade_cnt: 0,
                        is_closed: false,
                        is_won: false,
                        is_short: false,
                        open_time: 0,
                        time_elapsed: 0,
                    },
                    Some(t) => {
                        let x = &t.pos_res;
                        LastTradeRes {
                            trade_cnt: 3,
                            is_closed: true,
                            is_won: x.profit > 0.,
                            is_short: t.pos_res.is_short,
                            open_time: x.open_time as i64,
                            // fixme: from closed time
                            time_elapsed: app::clock::get_clock_time_sec() - x.close_time as i64,
                        }
                    }
                }
            }
            Some(t) => {
                let x = &t.pos_res;
                LastTradeRes {
                    trade_cnt: 1,
                    is_closed: false,
                    is_won: false,
                    is_short: t.pos_res.is_short,
                    open_time: x.open_time as i64,
                    // time_elapsed: 0
                    // from open time
                    time_elapsed: app::clock::get_clock_time_sec() - x.open_time as i64,
                }
            }
        };
        c
    }

    fn _get_open_trade(&self, pair: Pair) -> Option<PosHolder> {
        let mut trade = None;
        let mut last_time = 0;
        for (k, v) in self.open_pos.iter() {
            if v.pos_res.pair == pair && last_time < v.pos_res.open_time {
                last_time = v.pos_res.open_time;
                trade = Some(v.clone());
            }
        }
        trade
    }

    fn _get_closed_trade(&self, pair: Pair) -> Option<PosHolder> {
        let mut trade = None;
        let mut last_time = 0;
        for (k, v) in self.closed_pos.iter() {
            if v.pos_res.pair == pair && last_time < v.pos_res.open_time {
                last_time = v.pos_res.open_time;
                trade = Some(v.clone());
            }
        }
        trade
    }
}

// todo add closed time
#[derive(Debug, Clone, Default)]
pub struct LastTradeRes {
    pub trade_cnt: i32,
    pub is_closed: bool,
    pub is_won: bool,
    pub is_short: bool,
    pub open_time: i64,    // sec
    pub time_elapsed: i64, // sec
}

#[derive(Debug, Clone, Default)]
pub struct PosHolder {
    pub pos_res: EventPosition,
    pub profit_level: i32,
}

// todo: update/redesign this class fields
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ActionSignal {
    pub small_kid: i32,
    pub consumed: bool,
    pub long: bool,
    pub profit: f64,
    pub loss: f64,
    pub time_sec: i64,
    pub frame_insight: MLFrameTradeInsight,
}

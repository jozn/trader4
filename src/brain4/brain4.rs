use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
// use crate::dc_intel::{DCParent, FrameMem};
use crate::gate_api::{GateWay, NewPosDep, PosResDep, UpdatePos};
// use crate::ne::{NEFrame, NERoot};
use crate::ne3::NERoot;

use super::*;

pub type PairCandleCfg = (Pair, CandleConfig);

#[derive(Debug)]
pub struct Brain4 {
    pub con: Box<Arc<dyn GateWay>>,
    pub acted: HashSet<String>,
    pub db: Vec<PairMemory>,
    // pub open_pos: HashMap<u64, PosRes>,
    pub open_pos: HashMap<u64, PosHolder>,
    // From PairMemo
    // pub pair: Pair,
    // pub last_tick: Option<Tick>,
    pub last_trade_time: u64, // used in Acted filter
                              // pub ticks_arr: TimeSerVec<Tick>,
                              // pub ne: NERoot,
}

impl Brain4 {
    pub fn new(backend: Arc<impl GateWay + 'static>, pair_conf: (Pair, CandleConfig)) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            last_trade_time: 0,
            // ticks_arr: Default::default(),
            acted: Default::default(),
            db: vec![],
            open_pos: Default::default(),
            // pair: Pair::EURUSD,
            // last_tick: None,
            // dc_intl: DCParent::new(),
            // ne: NERoot::new(),
        };

        brain
    }

    // Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        println!("on_connect Brain2");
    }

    pub fn on_notify_position(&mut self, pos: PosResDep) {
        // println!(">>> {:?}", pos);
        if pos.is_closed {
            self.open_pos.remove(&pos.pos_id);
        } else {
            let mut res_opt = self.open_pos.get_mut(&pos.pos_id);
            match res_opt {
                None => {
                    let ph = PosHolder {
                        pos_res: pos.clone(),
                        profit_level: 0,
                    };
                    self.open_pos.insert(pos.pos_id, ph);
                }
                Some(ph) => {
                    ph.pos_res = pos; // update
                }
            }
        }
    }

    pub fn already_acted(&mut self, symbol_id: i64, kline_id: u64) -> bool {
        let time_sec = self.con.get_time_ms() / 1000;
        // println!("lat: {}", time_sec);
        if time_sec < self.last_trade_time + 1800 {
            return true;
        }

        let kids = format!("{}_{}", symbol_id, kline_id);
        if self.acted.contains(&kids) {
            return true;
        }
        self.last_trade_time = time_sec;
        self.acted.insert(kids);
        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct PosHolder {
    pub pos_res: PosResDep,
    pub profit_level: i32,
}

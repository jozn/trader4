use crate::bar::*;
use crate::base::CrossRes;
use crate::brain::sim_virtual::SimVirtual;
use crate::configs::assets;
use crate::configs::assets::*;
use crate::cortex;
use crate::cortex::{new_cortex_ref, CortexRef};
use crate::gate_api::*;
use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use super::*;

pub type PairBarCfg = (Pair, BarConfig);

#[derive(Debug)]
pub struct BrainLegacy {
    pub con: Box<Arc<dyn GateWay>>,
    pub cortex: CortexRef,
    pub acted: HashSet<String>,
    pub sim_virtual: SimVirtual,
    pub db: BTreeMap<i64, PairMemory>,
    pub open_pos: HashMap<u64, PosHolder>,
    pub last_trade_time: u64, // used in Acted filter
}

impl BrainLegacy {
    pub fn new(backend: Arc<impl GateWay + 'static>, pair_conf: PairBarCfg) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            cortex: cortex::new_cortex_ref(),
            last_trade_time: 0,
            acted: Default::default(),
            sim_virtual: SimVirtual::new(),
            db: Default::default(),
            open_pos: Default::default(),
        };
        brain
    }

    // Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        println!("on_connect Brain2");
    }

    pub fn on_notify_position(&mut self, pos: EventPosition) {
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

    pub fn already_acted(&mut self, symbol_id: i64, kline_id: i32) -> bool {
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
    pub pos_res: EventPosition,
    pub profit_level: i32,
}

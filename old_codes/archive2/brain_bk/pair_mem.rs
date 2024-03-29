use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::GateWay;
use crate::sig_engs::ml_eng::MLEng;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PairMemory {
    pub pair: Pair,
    pub last_tick: Option<BTickData>,
    pub last_trade_time: u64,
    pub dep_signals_db: SignalsDB, // dep
    pub ml_eng: MLEng,
}

impl PairMemory {
    pub fn new(p: Pair) -> PairMemory {
        Self {
            pair: p.clone(),
            last_tick: None,
            last_trade_time: 0,
            dep_signals_db: SignalsDB::new(),
            ml_eng: MLEng::new(&p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignalsDB {
    pub signals_db: BTreeMap<String, PairSignalsMemory>,
}

impl SignalsDB {
    pub fn new() -> Self {
        Self {
            signals_db: Default::default(),
        }
    }
    pub fn insert_signal(&mut self, sig: &PairSignalsMemory) {
        self.signals_db.insert(sig.key.clone(), sig.clone());
    }

    pub fn get_signal(&self, key: &str) -> Option<PairSignalsMemory> {
        let res = self.signals_db.get(key);
        match res {
            None => None,
            Some(sig) => Some(sig.clone()),
        }
    }
    pub fn remove_signal(&mut self, key: &str) {
        let res = self.signals_db.remove(key);
    }
}
#[derive(Debug, Clone)]
pub struct PairSignalsMemory {
    pub key: String,
    pub primary_signal: bool,
    pub ps_small_bar_id: i32,
    pub final_buy: bool,
    pub fb_small_bar_id: i32,
}

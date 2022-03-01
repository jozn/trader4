use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// A memory engine to be embeded in each SkyEng
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CortexMem {
    pub signal_mem: Option<SignalMem>,
    pub signals_db: BTreeMap<String, PairSignalsMemory>,
}

impl CortexMem {
    pub fn new() -> Self {
        Self {
            signal_mem: None,
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

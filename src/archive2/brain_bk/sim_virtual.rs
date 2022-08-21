use super::*;
use crate::collector::row_data::BTickData;
use crate::gate_api::{EventPosition, NewPosReq};
use crate::offline::{BackReportConf, BackendEngine, Position};
use crate::ta;
use crate::ta::EMA;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct SimVirtual {
    pub backend: BackendEngine,
    pub virtual_id: u64,
    pub _vid_keys: BTreeMap<u64, String>,
    pub _tails: BTreeMap<String, TailingWinRate>,
    pub _opens: BTreeMap<u64, Position>,
    pub _closed: BTreeMap<u64, Position>,
}

impl SimVirtual {
    pub fn new() -> Self {
        Self {
            backend: BackendEngine::new(100_000., &BackReportConf::default()),
            virtual_id: 0,
            _vid_keys: Default::default(),
            _tails: Default::default(),
            _opens: Default::default(),
            _closed: Default::default(),
        }
    }

    pub fn run_next_tick(&mut self, btick: BTickData) {
        self.backend.next_tick(btick);
        let nots = self.take_notify();
        for n in nots {
            // if
        }
    }

    pub fn take_notify(&mut self) -> Vec<EventPosition> {
        let res = self.backend.events.clone();
        self.backend.events.clear();
        res
    }

    pub fn open_position(&mut self, param: &NewPosReq, signal_key: &str) {
        self._vid_keys
            .insert(param.virtual_id, signal_key.to_string());
        self.backend.open_position_req_new(param);
    }

    pub fn next_virtual_id(&mut self) -> u64 {
        self.virtual_id += 1;
        self.virtual_id
    }
}

#[derive(Debug)]
pub struct TailingWinRate {
    pub ema: ta::EMA,
    pub last_val: f64,
    pub pair: u64,
}

impl TailingWinRate {
    fn new() -> Self {
        TailingWinRate {
            ema: EMA::new(5).unwrap(),
            last_val: 0.,
            pair: 0,
        }
    }
    fn next(&mut self, val: f64) -> f64 {
        let ma = self.ema.next(val);
        self.last_val = ma;
        ma
    }
}

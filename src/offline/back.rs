use super::*;
use crate::collector::row_data::BTickData;
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

#[derive(Debug)]
pub struct BackendEngineOuter {
    engine: Mutex<BackendEngine>,
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

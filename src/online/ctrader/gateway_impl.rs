use crate::configs::assets::Pair;
use crate::core::gate_api::{NewPosDep, UpdatePos};
use crate::gate_api::GateWay;
use crate::helper;
use crate::online::CTrader;

impl GateWay for CTrader {
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>) {
        let ids = symbols.iter().map(|p| p.to_symbol_id()).collect();
        self.subscribe_spots_req(ids);
    }

    fn open_position_req_new(&self, new_pos: &NewPosDep) {
        self.open_postion_req_new(&new_pos);
    }

    fn update_position(&self, update: &UpdatePos) {
        todo!()
    }

    fn get_time_ms(&self) -> u64 {
        helper::get_time_ms()
    }
}

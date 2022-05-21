use crate::configs::assets::Pair;
use crate::core::gate_api::*;
use crate::gate_api::GateWay;
use crate::helper;
use crate::online::CTrader;

impl GateWay for CTrader {
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>) {
        let ids = symbols.iter().map(|p| p.to_symbol_id()).collect();
        self.subscribe_spots_req(ids);
    }

    fn update_position(&self, update: &UpdatePosReq) {
        todo!()
    }

    fn get_time_ms(&self) -> u64 {
        helper::get_time_ms()
    }
}

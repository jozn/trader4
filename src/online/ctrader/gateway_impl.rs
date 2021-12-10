use crate::candle::Tick;
use crate::core::gate_api::NewPos;
use crate::gate_api::GateWay;
use crate::helper;
use crate::online::{Actor, CTrader};

impl GateWay for CTrader {
    fn subscribe_pairs_req(&self, symbols: Vec<i64>) {
        self.subscribe_spots_req(symbols);
    }

    fn open_postion_req_new(&self, new_pos: &NewPos) {
        self.open_postion_req_new(&new_pos);
    }

    fn update_postion(&self) {
        todo!()
    }

    fn on_price_tick(&self, symbol_id: i64, tick: Tick, bot: &mut Actor) {
        todo!()
    }

    fn on_postion_event(&self, symbol_id: i64, tick: Tick, bot: &mut Actor) {
        todo!()
    }

    fn get_time_ms(&self) -> u64 {
        helper::get_time_ms()
    }

    fn on_connected(&self) {
        todo!()
    }
}

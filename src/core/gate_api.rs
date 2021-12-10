use crate::candle::Tick;
use crate::online::Actor;
use std::fmt::Debug;

pub trait GateWay: Debug {
    // Calls from brain
    fn subscribe_pairs_req(&self, symbols: Vec<i64>);
    // fn go_long(&self, symbol_id: i64, tick: &Tick);
    // fn go_short(&self, symbol_id: i64, tick: &Tick);
    fn open_postion_req_new(&self, new_pos: &NewPos);
    fn update_postion(&self);

    // fn go_long(&mut self, symbol_id: i64, tick: &Tick);
    // fn go_short(&mut self, symbol_id: i64, tick: &Tick);
    // fn update_postion(&mut self);

    // From data source to Brain
    fn on_price_tick(&self, symbol_id: i64, tick: Tick, bot: &mut Actor);
    fn on_postion_event(&self, symbol_id: i64, tick: Tick, bot: &mut Actor);

    // Others
    fn get_time_ms(&self) -> u64;

    // Remove?
    fn on_connected(&self); // Can be called repeadly during connection distruption
}

#[derive(Debug)]
pub struct NewPos {
    pub symbol_id: i64,
    pub is_short: bool,
    pub size_usd: i64,
    pub take_profit_price: f64,
    pub stop_loose_price: f64,
}

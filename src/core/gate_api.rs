// use crate::candle::Tick;
// use crate::online::Actor;
use crate::candle::TA1;
use crate::configs::assets::Pair;
use crate::offline::{XLot, XPrice};
use std::fmt::Debug;

pub trait GateWay: Debug {
    // Calls from brain
    fn subscribe_pairs_req(&self, symbols: Vec<Pair>);
    // fn go_long(&self, symbol_id: i64, tick: &Tick);
    // fn go_short(&self, symbol_id: i64, tick: &Tick);
    fn open_position_req_new(&self, new_pos: &NewPos);
    fn update_position(&self);

    // fn go_long(&mut self, symbol_id: i64, tick: &Tick);
    // fn go_short(&mut self, symbol_id: i64, tick: &Tick);
    // fn update_postion(&mut self);

    // From data source to Brain
    // fn on_price_tick(&self, symbol_id: i64, tick: Tick, bot: &mut Actor);
    // fn on_postion_event(&self, symbol_id: i64, tick: Tick, bot: &mut Actor);

    // Others
    fn get_time_ms(&self) -> u64;

    // Remove?
    //fn on_connected(&self); // Can be called repeadly during connection distruption
}

#[derive(Debug, Clone, Default)]
pub struct NewPos3 {
    pub symbol_id: i64,
    pub is_short: bool,
    pub size_usd: i64,
    pub take_profit_price: f64,
    pub stop_loose_price: f64,

    // From old offline - feel free to delete unneeded
    pub price: XPrice,    // todo change to at_price
    pub price_multi: f64, // delete
    pub pos_size: XLot,
    pub pos_id: u64, // from Brain internal
    pub time: u64,   // Brain time
    pub ta: TA1,
    // pub pair: Pair,
}

#[derive(Debug, Clone, Default)]
pub struct NewPos {
    pub symbol_id: i64,
    pub is_short: bool,
    pub size_usd: i64, // Could be other currency -- as the cTrader works this way we do not use XLot
    pub take_profit_price: f64,
    pub stop_loose_price: f64,

    // Informative
    pub at_price: f64,
    pub time: u64, // Brain time
    pub ta: TA1,
    // add comment, label too
}

// impl NewPos {
//     pub fn get_usd(&self) -> f64 {
//         self.pos_size as f64 * 1000.
//     }
// }

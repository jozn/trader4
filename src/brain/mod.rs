pub mod brain;
pub mod handler;

pub use brain::*;
pub use handler::*;

// How to use Brain:
//  From online cTrader or offline scrip we call this methods:
//      Brain.on_connect()
//      Brain.on_price_tick(&mut self, symbol_id: i64, tick: Tick)
//  Then Brain must decide all logic when to buy,...
//  Brain calls GateWay api trait to communicate witt cTrader or offline simulator

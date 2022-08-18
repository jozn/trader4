use crate::collector::import_all::BTickData;
use crate::configs::assets::Pair;

#[derive(Debug)]
pub struct PairMemory {
    pub pair: Pair,
    pub last_tick: Option<BTickData>,
    // pub last_trade: Option<u64>,
    // pub last_trade_time: u64,
}

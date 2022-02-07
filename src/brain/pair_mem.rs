use super::*;
use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::GateWay;
use crate::sky_eng::*;

#[derive(Debug)]
pub struct PairMemory {
    pub pair: Pair,
    pub last_tick: Option<BTickData>,
    pub last_trade_time: u64,
    pub sky_eng: SkyEng,
}

impl PairMemory {
    pub fn new(p: Pair) -> PairMemory {
        Self {
            pair: p,
            last_tick: None,
            last_trade_time: 0,
            sky_eng: SkyEng::new(),
        }
    }
}

use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::GateWay;
use crate::ne4::NERoot;
use crate::{candle, helper};

#[derive(Debug)]
pub struct PairMemory {
    pub pair: Pair,
    pub last_tick: Option<Tick>,
    pub last_trade_time: u64,
    pub ticks_arr: TimeSerVec<Tick>,
    pub candles: CandleSeriesTA,
    pub ne4: NERoot,
}

impl PairMemory {
    pub fn new(p: Pair, conf: &CandleConfig) -> PairMemory {
        Self {
            pair: p,
            last_tick: None,
            last_trade_time: 0,
            ticks_arr: Default::default(),
            candles: CandleSeriesTA::new(conf),
            ne4: NERoot::new(),
        }
    }
}

use super::*;
use crate::collector::import_all::BTickData;
use crate::cortex::types::{ActionSignal, SignalMem};
use crate::helper;
use crate::types::*;

impl SkyEng {
    pub fn set_signals_v4(&mut self, tick: &BTickData, sf: &mut SFrame) -> Option<ActionSignalDep> {
        // let sf = &self;
        let kid = sf.bar_medium.primary.seq;
        let kid_small = sf.bar_small_tip.primary.seq;
        let bigb = &sf.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.primary.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;
        let low_price = sf.bar_small_tip.primary.low;
        // let low_price = sf.bar_medium.primary.low;

        let small_bar_big = &sf.bar_small_tip.big.ta;

        let act = ActionSignalDep::default();
        // if bigta.ma_mom > 0. {
        //     if medpta.ma_mom > 0. {
        // if snake.low_band > low_price && bigta.ma_mom > 0.{
        if snake.low_band > low_price {
            self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
            self.cortex_mem.set_action(&ActionSignal {
                small_kid: kid_small,
                consumed: false,
                long: true,
                profit: 8.0,
                loss: -4.0,
                time_sec: tick.timestamp_sec,
            });
        }
        // }
        // }

        None
    }
}

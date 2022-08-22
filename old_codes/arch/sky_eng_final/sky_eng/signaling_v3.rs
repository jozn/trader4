use super::*;
use crate::collector::import_all::BTickData;
use crate::cortex::types::{ActionSignal, SignalMem};
use crate::helper;
use crate::types::*;

impl SkyEng {
    // This is for random AI
    pub fn set_signals_v6(&mut self, tick: &BTickData, sf: &mut SFrame) -> Option<ActionSignalDep> {
        // let sf = &self;
        let kid = sf.bar_medium.primary.seq;
        let kid_small = sf.bar_small_tip.primary.seq;
        let bigb = &sf.bar_major.big;
        let bigta = &sf.bar_major.primary.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.primary.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;
        let low_price = sf.bar_small_tip.primary.low;
        // let low_price = sf.bar_medium.primary.low;

        let small_bar_big = &sf.bar_small_tip.big.ta;

        let dc_snake = &medpta.dc_snake;
        let macd = &bigta.macd;
        let macd_valid = bigta.macd.macd > bigta.macd.signal;
        // if dc_snake.oversold_line > low_price && bigta.ma_mom > 0. {
        let pro = medpta.atr * 10_000.;
        // if dc_snake.oversold_line > low_price  && macd_valid {
        // if dc_snake.oversold_line > low_price && bigta.ma_mom > 0. && macd_valid {
        // println!("art: {}", pro);
        if kid % 10 == 0 {
            self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
            self.cortex_mem.set_action(&ActionSignal {
                small_kid: kid_small,
                consumed: false,
                long: true,
                profit: pro * 3.,
                loss: -pro * 1.5,
                time_sec: tick.timestamp_sec,
            });
        }

        if snake.low_band > low_price {}
        // }
        // }

        None
    }
}

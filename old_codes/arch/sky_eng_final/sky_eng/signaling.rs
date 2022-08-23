use super::*;
use crate::collector::import_all::BTickData;
use crate::cortex::types::{ActionSignal, SignalMem};
use crate::helper;
use crate::types::*;

impl SkyEng {
    pub fn set_signals_v1(&mut self, tick: &BTickData, sf: &mut SFrame) -> Option<ActionSignalDep> {
        // let sf = &self;
        let kid = sf.bar_medium.primary.seq;
        let kid_small = sf.bar_small_tip.primary.seq;
        let bigb = &sf.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.big.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;
        let low_price = sf.bar_small_tip.primary.low;

        let small_bar_big = &sf.bar_small_tip.big.ta;

        let act = ActionSignalDep::default();
        if bigta.ma_mom > 0. {
            if medpta.ma_mom > 0. {
                if snake.low_band > low_price {
                    self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
                    self.cortex_mem.set_action(&ActionSignal {
                        small_kid: kid_small,
                        consumed: false,
                        long: true,
                        profit: 8.0,
                        loss: -8.0,
                        time_sec: tick.timestamp_sec,
                    });
                }
            }
        }

        None
    }

    pub fn set_signals_random(
        &mut self,
        tick: &BTickData,
        sf: &mut SFrame,
    ) -> Option<ActionSignalDep> {
        let kid_small = sf.bar_small_tip.primary.seq;
        let kid = sf.bar_medium.primary.seq;
        let bigb = &sf.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.big.ta;

        if helper::get_rand(1000) < 11 && bigta.trend.is_bearish() {
            self.cortex_mem.mark_long_early(kid, tick.timestamp_sec);
            /*            self.cortex_mem.mark_long_final(kid_small, tick.timestamp_sec);
            self.cortex_mem.set_action(&ActionSignal{
                small_kid: kid_small,
                consumed: false,
                long: true,
                profit: 8.0,
                loss: -8.0,
                time_sec: tick.timestamp_sec
            });*/
            // if helper::get_rand(1000) < 11 && bigta.trend.is_bullish() &&medbta.trend.is_bullish() {
            // let act = ActionSignalDep {
            //     small_kid: kid,
            //     long: true,
            //     profit: 8.0,
            //     loss: -8.0,
            // };
            // self.signal_mem = None;
            // return Some(act);
        }
        match self.cortex_mem.get_snapshot(0) {
            None => {}
            Some(m) => {
                // let mo = helper::get_rand(20) as i32 + 3;
                if m.ps_buy && kid > m.ps_medium_bar_id + 1 {
                    self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
                    self.cortex_mem.set_action(&ActionSignal {
                        small_kid: kid_small,
                        consumed: false,
                        long: true,
                        profit: 8.0,
                        loss: -8.0,
                        time_sec: tick.timestamp_sec,
                    });
                }
                if m.ps_buy && kid > m.ps_medium_bar_id + 10 {
                    self.cortex_mem.cancel_long_mark();
                }
            }
        }
        None
    }

    pub fn set_signals_v3(&mut self, tick: &BTickData, sf: &mut SFrame) -> Option<ActionSignalDep> {
        // let sf = &self;
        let kid = sf.bar_medium.primary.seq;
        let kid_small = sf.bar_small_tip.primary.seq;
        let bigb = &sf.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.big.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;
        let low_price = sf.bar_small_tip.primary.low;
        // let low_price = sf.bar_medium.primary.low;

        let small_bar_big = &sf.bar_small_tip.big.ta;

        let act = ActionSignalDep::default();
        // if bigta.ma_mom > 0. {
        //     if medpta.ma_mom > 0. {
        if snake.low_band > low_price && bigta.ma_mom > 0. {
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

    pub fn set_signals_v2(&mut self, tick: &BTickData, sf: &mut SFrame) -> Option<ActionSignalDep> {
        // let sf = &self;
        let kid = sf.bar_medium.primary.seq;
        let kid_small = sf.bar_small_tip.primary.seq;
        let bigb = &sf.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.big.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;
        let low_price = sf.bar_small_tip.primary.low;

        let small_bar_big = &sf.bar_small_tip.big.ta;

        let act = ActionSignalDep::default();
        if bigta.ma_mom > 0. {
            if medpta.ma_mom > 0. {
                if snake.low_band > low_price {
                    let mut sm = match self.signal_mem.clone() {
                        None => SignalMemDep::default(),
                        Some(i) => i,
                    };

                    if !sm.ps_buy {
                        sm.ps_buy = true;
                        sm.ps_small_bar_id = kid_small;
                    }
                    self.signal_mem = Some(sm);
                }
            }
        }

        match self.signal_mem.clone() {
            None => {}
            Some(sm) => {
                if sm.ps_buy {
                    if small_bar_big.trend.is_bullish() {
                        let act = ActionSignalDep {
                            small_kid: kid,
                            long: true,
                            profit: 6.0,
                            loss: -6.0,
                        };
                        self.signal_mem = None;
                        return Some(act);
                    }
                }
            }
        };

        None
    }
}
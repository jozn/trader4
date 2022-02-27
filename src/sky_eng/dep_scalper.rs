use super::*;
use crate::brain::{PairSignalsMemory, SignalsDB};
use crate::collector::row_data::BTickData;
use crate::types::ActionSignal;

pub struct ScalpConf {
    pub slow_ma_min_abs: f64,
}

impl SFrame {
    pub fn set_scalper_dep(&mut self, tick: &BTickData, mem: &mut SignalsDB) -> Option<ActionSignal> {
        let kid = self.bar_medium.primary.seq;
        // let sf = &self;
        let bigb = &self.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &self.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &self.bar_medium.big.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;

        //// to del
        let mut force = false;
        if bigta.trend.is_bullish() || false {
            // self.sign_sell = false;

            let pb = &self.bar_medium.primary;
            let mpta = &pb.ta;

            // set early signals
            for spb in &self.bars_small {
                let sb = &spb.primary;
                let tas = &spb.primary.ta;
                let m = &tas.macd;
                // if m.macd_above && m.macd < 0. {
                if m.macd_above {
                    self.buys_dep.push(sb.open_time);
                }
            }

            if mpta.rpi.buy_low {
                // self.sign_buy = true;
                self.buy2_dep = true;
                force = true;
                // self.buys.push(pb.open_time);
            }
        }
        ///////////////////////////

        let small_bar_big = &self.bar_small_tip.big.ta;
        // let act = ActionSignal::default();
        if bigta.ma_mom > 0. || force {
            if medpta.ma_mom > 0. || force {
                if snake.low_band > price || force {
                    // self.buy2 = true;
                    self.buy2_dep = true;
                    self.sign_buy_dep = true;
                    let act = ActionSignal {
                        small_kid: kid,
                        long: true,
                        profit: 6.0,
                        loss: -3.0,
                    };
                    let sm = PairSignalsMemory {
                        key: "sky1".to_string(),
                        primary_signal: false,
                        ps_small_bar_id: 0,
                        final_buy: true,
                        fb_small_bar_id: kid
                    };
                    mem.insert_signal(&sm);
                    // println!("{}",kid);
                    return Some(act);
                }
            }
        }
        None
    }
}

use super::*;
use crate::brain::SignalsDB;
use crate::collector::row_data::BTickData;
use crate::types::ActionSignal;

pub struct ScalpConf {
    pub slow_ma_min_abs: f64,
}

impl SFrame {
    pub fn set_scalper_bk(&mut self, tick: &BTickData) {
        set_scalpe_signals_v1(self, tick);
    }
    pub fn set_scalper(&mut self, tick: &BTickData, mem: &mut SignalsDB) -> Option<ActionSignal> {
        let kid = self.bar_medium.primary.seq;
        let sf = &self;
        let bigb = &sf.bar_major.big;
        let bigta = &bigb.ta;

        let medbb = &sf.bar_medium.big;
        let medbta = &medbb.ta;
        let medpta = &sf.bar_medium.big.ta;

        let snake = &medpta.sb;
        let price = tick.bid_price;

        let small_bar_big = &sf.bar_small_tip.big.ta;
        let act = ActionSignal::default();
        if bigta.ma_mom > 0. {
            if medpta.ma_mom > 0. {
                if snake.low_band > price {
                    // sf.buy2 = true;
                    self.buy2 = true;
                    self.sign_buy = true;
                    let act = ActionSignal {
                        small_kid: kid,
                        long: true,
                        profit: 6.0,
                        loss: -4.0,
                    };
                    return Some(act);
                }
            }
        }
        None
    }
}

pub fn set_scalpe_signals_v1(sf: &mut SFrame, tick: &BTickData) {
    let bigb = &sf.bar_major.big;
    let bigta = &bigb.ta;

    let medbb = &sf.bar_medium.big;
    let medbta = &medbb.ta;
    let medpta = &sf.bar_medium.big.ta;

    let snake = &medpta.sb;
    let price = tick.bid_price;

    let small_bar_big = &sf.bar_small_tip.big.ta;

    if bigta.ma_mom > 0. {
        if medpta.ma_mom > 0. {
            if snake.low_band > price {
                // sf.buy2 = true;
                sf.sign_buy = true;
            }
        }
    }

    if sf.sign_buy {}
    if small_bar_big.trend.is_bullish() {}

    if bigta.trend.is_bullish() || false {
        sf.sign_sell = false;

        let pb = &sf.bar_medium.primary;
        let mpta = &pb.ta;

        // set early signals
        for spb in &sf.bars_small {
            let sb = &spb.primary;
            let tas = &spb.primary.ta;
            let m = &tas.macd;
            // if m.macd_above && m.macd < 0. {
            if m.macd_above {
                sf.buys.push(sb.open_time);
            }
        }

        if mpta.rpi.buy_low {
            sf.sign_buy = true;
            sf.buy2 = true;
            // sf.buys.push(pb.open_time);
        }
    }
}

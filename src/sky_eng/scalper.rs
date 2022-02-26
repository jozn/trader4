use super::*;
use crate::collector::row_data::BTickData;

pub struct ScalpConf {
    pub slow_ma_min_abs: f64,
}

impl SFrame {
    pub fn set_scalper(&mut self) {
        set_scalpe_signals_v1(self);
    }
}

// pub fn set_scalpe_signals_v1(sf: &mut SFrame, tick: &BTickData) {
pub fn set_scalpe_signals_v1(sf: &mut SFrame) {
    let bigb = &sf.bar_major.big;
    let bigta = &bigb.ta;

    let medbb = &sf.bar_medium.big;
    let medbta = &medbb.ta;
    let medpta = &&sf.bar_medium.big.ta;

    if bigta.ma_mom > 3. {
        if medbta.ma_mom > 0. {}
    }

    if bigta.trend.is_bullish() {
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

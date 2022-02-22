use super::*;

pub struct ScalpConf {
    pub slow_ma_min_abs: f64,
}

impl SFrame {
    pub fn set_scalper(&mut self) {
        set_scalpe_signals_v1(self);
    }
}

pub fn set_scalpe_signals_v1(sf: &mut SFrame) {
    let bigb = &sf.bar_major.big;
    let bigta = &bigb.ta;

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

    // if sf.tscore.diff <= -1. {
    if bigta.trend.is_bearish() {
        sf.sign_buy = false;

        let pb = &sf.bar_medium.primary;
        let ta = &pb.ta;
        let m = &ta.macd;

        for spb in &sf.bars_small {
            let sb = &spb.primary;
            let tas = &spb.primary.ta;
            let m = &tas.macd;
            // if m.macd_under && m.macd > 0. {
            if m.macd_under {
                sf.sells.push(sb.open_time);
            }
        }

        if ta.rpi.buy_high {
            sf.sign_sell = true;
            sf.sell2 = true;
            // sf.sells.push(pb.open_time);
        }
    }
}

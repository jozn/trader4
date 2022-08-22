use super::*;

impl SFrame {
    pub fn set_signals_dep(&mut self) {
        set_signals_v5(self);
    }
}

pub fn set_signals_v5(sf: &mut SFrame) {
    let bigb = &sf.bar_medium.big;
    let bigta = &bigb.ta;

    // if sf.tscore.diff >= 1. {
    if bigta.trend.is_bullish() {
        sf.sign_sell = false;

        let pb = &sf.bar_medium.primary;
        let ta = &pb.ta;

        for spb in &sf.bars_small {
            let sb = &spb.primary;
            let tas = &spb.primary.ta;
            let m = &tas.macd;
            // if m.macd_above && m.macd < 0. {
            if m.macd_above {
                sf.buys.push(sb.open_time);
            }
        }

        if ta.rpi.buy_low {
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
pub fn set_signals_v4(sf: &mut SFrame) {
    if sf.tscore.diff >= 1. {
        let pb = &sf.bar_medium.primary;
        let ta = &pb.ta;
        let m = &ta.macd;
        if m.macd_above && m.macd < 0. {
            let mta = &sf.bar_major.primary.ta;
            sf.buy2 = true;
            sf.buys.push(pb.open_time);

            if mta.ma_mom > 0. {}
        }
    }

    if sf.tscore.diff <= -1. {
        let pb = &sf.bar_medium.primary;
        let ta = &pb.ta;
        let m = &ta.macd;
        if m.macd_under && m.macd > 0. {
            let mta = &sf.bar_major.primary.ta;
            sf.sell2 = true;
            sf.sells.push(pb.open_time);
            if mta.ma_mom < -0. {}
        }
    }
}

pub fn set_signals_v3(sf: &mut SFrame) {
    if sf.tscore.diff >= 1. {
        for ph in &sf.bars_small {
            let pb = &ph.primary;
            let ta = &pb.ta;
            let m = &ta.macd;
            if m.macd_above && m.macd < 0. {
                let mta = &sf.bar_major.primary.ta;
                if mta.ma_mom > 0. {
                    sf.buy2 = true;
                    sf.buys.push(pb.open_time);
                }
            }
        }
    }

    if sf.tscore.diff <= -1. {
        for ph in &sf.bars_small {
            let pb = &ph.primary;
            let ta = &pb.ta;
            let m = &ta.macd;
            if m.macd_under && m.macd > 0. {
                let mta = &sf.bar_major.primary.ta;
                sf.sell2 = true;
                sf.sells.push(pb.open_time);
                if mta.ma_mom < -0. {}
            }
        }
    }
}

pub fn set_signals_v2(sf: &mut SFrame) {
    // let s = self;
    if sf.tscore.diff > 1. {
        for ph in &sf.bars_small {
            let pb = &ph.primary;
            let ta = &pb.ta;
            let m = &ta.macd;
            if m.macd_above && m.macd < 0. {
                sf.buy2 = true;
                sf.buys.push(pb.open_time);
            }
        }
    }

    if sf.tscore.diff < -1. {
        for ph in &sf.bars_small {
            let pb = &ph.primary;
            let ta = &pb.ta;
            let m = &ta.macd;
            if m.macd_under && m.macd > 0. {
                sf.sell2 = true;
                sf.sells.push(pb.open_time);
            }
        }
    }
}

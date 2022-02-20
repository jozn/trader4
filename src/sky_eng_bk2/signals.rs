use super::*;

impl SFrame {
    pub fn set_signals(&mut self) {
        set_signals_v3(self);
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

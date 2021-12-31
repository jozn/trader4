use super::*;

#[derive(Debug)]
pub struct FramesRels<'a> {
    pub frames2: &'a Vec<NEFrame>,
    pub last: &'a NEFrame,
    pub period: usize,
}

impl FramesRels<'_> {
    pub fn trends_detection(&self) -> f64 {
        assert!(self.frames2.len() > 2);

        let mut up = 0.;
        let mut down = 0.;
        let mut zero = 0.;

        let mut midle = self.last.med_mid;
        for f in self.frames2.iter().rev().take(self.period) {
            let fstr = &f.strength;
            if f.trd4 > 0. {
                up +=1.;
            }
            if f.trd4 < 0. {
                down +=1.;
            }
            if f.trd4 == 0. {
                zero +=1.;
            }
        }
        let sum = self.period as f64;
        let mut trend = 0.;
        if (up/sum) >= 0.2 && (down / sum ) < 0.20 {
            trend = 1.;
        }
        if (up/sum) < 0.2 && (down / sum ) > 0.20 {
            trend = -1.;
        }
        // todo remove chuby from it

        trend
    }
    pub fn touched_lower_low(&self) -> bool {
        assert!(self.frames2.len() > 2);

        let mut postive = 0.;
        let mut negative = 0.;

        let mut midle = self.last.med_mid;
        for f in self.frames2.iter().rev().take(self.period) {
            let fstr = &f.strength;
            if fstr.l_low {
                postive += 1.;
            }
            if fstr.h_high {
                negative += 1.;
            }
        }

        if postive >= 2. && (negative / postive) < 0.20 {
            true
        } else {
            false
        }
    }

    pub fn touched_higher_high(&self) -> bool {
        assert!(self.frames2.len() > 2);

        let mut postive = 0.;
        let mut negative = 0.;

        let mut midle = self.last.med_mid;
        for f in self.frames2.iter().rev().take(self.period) {
            let fstr = &f.strength;
            if fstr.h_high {
                postive += 1.;
            }
            if fstr.l_low {
                negative += 1.;
            }
        }

        if postive >= 2. && (negative / postive) < 0.20 {
            true
        } else {
            false
        }
    }
    pub fn middle_going_high(&self) -> bool {
        let (postive, negative) = self.middle_rel();

        if postive >= 2. && (negative / postive) < 0.15 {
            true
        } else {
            false
        }
    }

    pub fn middle_going_down(&self) -> bool {
        let (postive, negative) = self.middle_rel();

        if negative >= 2. && (postive / negative) < 0.15 {
            true
        } else {
            false
        }
    }

    pub fn middle_rel(&self) -> (f64, f64) {
        assert!(self.frames2.len() > 2);

        let mut postive = 0.;
        let mut negative = 0.;

        let mut midle = self.last.med_mid;
        for f in self.frames2.iter().rev().take(self.period) {
            if midle > f.med_mid {
                postive += 1.;
            }
            if midle < f.med_mid {
                negative += 1.;
            }
        }
        (postive, negative)
    }
}

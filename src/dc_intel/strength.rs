use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DCStrength {
    pub trend: f64,
    pub h_high: bool, // higher high
    pub buy2: bool,
    pub l_low: bool, // lower low
    pub strength: f64,
    pub dir: f64,  // 1 up _ 0 neturalze _ -1 down
    pub dir2: f64, // 1 up _ 0 neturalze _ -1 down
    pub hh: f64,   // 1 up _ 0 neturalze _ -1 down
    pub ll: f64,   // 1 up _ 0 neturalze _ -1 down
}

pub fn get_strength(last: &FrameMem, frames: &Vec<FrameMem>) -> DCStrength {
    if frames.len() < 3 {
        return DCStrength::default();
    }
    let pre = frames.last().unwrap();

    let mut dc_str = DCStrength {
        trend: last.trd2,
        h_high: false,
        buy2: false,
        l_low: false,
        strength: 0.0,
        dir: 0.0,
        ..Default::default()
    };

    // going up?
    if last.med_high > pre.med_high {
        dc_str.h_high = true;
        if last.trd2 > 0. {
            dc_str.buy2 = true;
        }
    }

    // going up?
    if last.med_low < pre.med_low {
        dc_str.l_low = true;
    }

    if dc_str.h_high == true && dc_str.l_low == true {
        dc_str.h_high = false;
        dc_str.l_low = false;
    }

    let rel = FramesRels {
        // frames: vec![],
        frames2: frames,
        last,
        period: 10,
    };

    // set direction
    let mut dir = 0.;
    //bk
    if last.trd2 > 0. && rel.middle_going_high() {
        dir = 1.;
    }
    if last.trd2 < 0. && rel.middle_going_down() {
        dir = -1.;
    }
    dc_str.dir2 = dir;

    if last.trd2 > 0. {
        dir = 1.;
    }
    if last.trd2 < 0. {
        dir = -1.;
    }

    dc_str.dir = dir;
    dc_str.hh = rel.middle_going_high() as i32 as f64;
    dc_str.ll = rel.middle_going_down() as i32 as f64;

    dc_str
}

#[derive(Debug)]
pub struct FramesRels<'a> {
    // frames:  Vec<&'a FrameMem>,
    frames2: &'a Vec<FrameMem>,
    last: &'a FrameMem,
    period: usize,
}

impl FramesRels<'_> {
    fn middle_going_high(&self) -> bool {
        let (postive, negative) = self.middle_rel();

        if postive >= 2. && (negative / postive) < 0.15 {
            true
        } else {
            false
        }
    }

    fn middle_going_down(&self) -> bool {
        let (postive, negative) = self.middle_rel();

        if negative >= 2. && (postive / negative) < 0.15 {
            true
        } else {
            false
        }
    }

    fn middle_rel(&self) -> (f64, f64) {
        assert!(self.frames2.len() > 2);

        let mut postive = 0.;
        let mut negative = 0.;

        let mut midle = self.last.get_med_middle();
        for f in self.frames2.iter().rev().take(self.period) {
            if midle > f.get_med_middle() {
                postive += 1.;
            }
            if midle < f.get_med_middle() {
                negative += 1.;
            }
        }
        (postive, negative)
    }
}

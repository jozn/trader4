use super::*;
use crate::candle::Tick;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NEStrength {
    pub rsi_st_bull: bool,
    pub rsi_st_bear: bool,
    pub trend: f64,
    pub h_high: bool, // higher high
    pub l_low: bool,  // lower low
    pub strength: f64,
    pub dir: f64,  // 1 up _ 0 neturalze _ -1 down
    pub dir2: f64, // 1 up _ 0 neturalze _ -1 down
    pub hh: f64,   // 1 up _ 0 neturalze _ -1 down
    pub ll: f64,   // 1 up _ 0 neturalze _ -1 down
    pub dis_bull: u64,
    pub dis_bear: u64,
    pub buy: bool,
    pub sell: bool,
}

impl NEStrength {
    pub fn new(frame: &NEFrame) -> Self {
        let st_rsi = &frame.rsi_sth;
        let mut str = NEStrength {
            rsi_st_bull: (st_rsi.k_above_d && st_rsi.main_k < 35.),
            rsi_st_bear: (st_rsi.k_under_d && st_rsi.main_k > 65.),
            trend: 0.0,
            h_high: false,
            l_low: false,
            strength: 0.0,
            dir: 0.0,
            dir2: 0.0,
            hh: 0.0,
            ll: 0.0,
            dis_bull: 0,
            dis_bear: 0,
            ..Default::default()
        };
        if frame.trd_ad > 0. && st_rsi.k_above_d {
            // if frame.trd_ad > 0. {
            // println!("set");
            // if  st_rsi.k_above_d {
            str.buy = true;
        }
        if frame.trd_ad < 0. && st_rsi.k_under_d {
            // if frame.trd_ad < 0. {
            // if st_rsi.k_under_d {
            str.sell = true;
        }

        str
    }
}

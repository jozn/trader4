use super::*;
use crate::candle::Tick;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NEStrength {
    pub trend: f64,
    pub h_high: bool, // higher high
    pub buy2: bool,
    pub l_low: bool, // lower low
    pub strength: f64,
    pub dir: f64,  // 1 up _ 0 neturalze _ -1 down
    pub dir2: f64, // 1 up _ 0 neturalze _ -1 down
    pub hh: f64,   // 1 up _ 0 neturalze _ -1 down
    pub ll: f64,   // 1 up _ 0 neturalze _ -1 down
    pub dis_bull: u64,
    pub dis_bear: u64,
}

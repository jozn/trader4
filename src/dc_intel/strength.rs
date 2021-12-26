use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DCStrength {
    pub trend: f64,
    pub h_high: bool, // higher high
    pub buy2: bool,
    pub l_low: bool, // lower low
    pub strength: f64,
    pub dir: f64, // 1 up _ 0 neturalze _ -1 down
}

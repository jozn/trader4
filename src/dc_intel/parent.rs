use super::*;
use crate::base::*;
use crate::candle::{Tick, TimeSerVec};
use crate::ta::*;
use serde::{Deserialize, Serialize};
use crate::helper;


pub struct DCParent {
    pub frame_id: u64, // For next frame id
    pub frames: Vec<FrameMem>,

    // TA holders
    pub med_high: Maximum,
    pub med_low: Minimum,
    pub big_high: Maximum,
    pub big_low: Minimum,
    pub dc_med: DC,
    pub dc_big: DC,

    pub ma: EMA,
}


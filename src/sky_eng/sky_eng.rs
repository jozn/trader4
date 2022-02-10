use super::*;
use crate::base::*;
use crate::collector::import_all::BTickData;
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

// Sky Engine
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkyEng {
    pub frame_id: u64, // For next frame id
    pub frames: Vec<SFrame>,
    pub major_cfg: BarConfig,
    pub major_bars: BarSeries,
    pub medium_cfg: BarConfig,
    pub medium_bars: BarSeries,
    pub small_cfg: BarConfig, // Price hunter
    pub small_bars: BarSeries,
}

impl SkyEng {
    pub fn new() -> Self {
        let major_ticks = 1500;
        let major_cfg = BarConfig {
            primary_ticks: major_ticks,
            big_ticks: major_ticks * 3,
        };

        let primary_ticks = 150;
        let medium_cfg = BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        };

        let small_ticks = 15;
        let small_cfg = BarConfig {
            primary_ticks: small_ticks,
            big_ticks: small_ticks * 3,
        };

        Self {
            frame_id: 0,
            frames: vec![],
            major_cfg: major_cfg.clone(),
            major_bars: BarSeries::new(&major_cfg),
            medium_cfg: medium_cfg.clone(),
            medium_bars: BarSeries::new(&medium_cfg),
            small_cfg: small_cfg.clone(),
            small_bars: BarSeries::new(&small_cfg),
        }
    }

    pub fn add_tick(&mut self, tick: &BTickData) -> Option<SFrame> {
        let ph_big = self.major_bars.add_tick_mut(tick);
        let ph_medium = self.medium_bars.add_tick_mut(tick);
        let ph_small = self.small_bars.add_tick_mut(tick);
        match ph_medium {
            None => None,
            Some(r) => {
                let mut frame = new_frame(&r);
                self.frames.push(frame.clone());
                Some(frame)
            }
        }
    }
}

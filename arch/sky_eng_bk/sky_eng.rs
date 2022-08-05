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
    pub cfg: BarConfig,
    pub frames: Vec<SFrame>,
    pub bars: BarSeries,
}

impl SkyEng {
    pub fn new() -> Self {
        let primary_ticks = 200;
        let cfg = BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        };

        Self {
            frame_id: 0,
            cfg: cfg.clone(),
            frames: vec![],
            bars: BarSeries::new(&cfg),
        }
    }

    pub fn add_tick(&mut self, tick: &BTickData) -> Option<SFrame> {
        let ph = self.bars.add_tick_mut(tick);
        match ph {
            None => None,
            Some(r) => {
                let mut frame = new_frame(&r);
                self.frames.push(frame.clone());
                Some(frame)
            }
        }
    }
}

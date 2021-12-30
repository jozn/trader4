use super::*;
use crate::base::*;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};
// NE: New Engine
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NERoot {
    //
    pub frame_id: u64,     // For next frame id
    pub cfg: CandleConfig, // For next frame id
    pub frames: Vec<NEFrame>,
    pub candles: CandleSeriesTA, // todo: build a candle ver 2 like canlde but better
    pub ticks_buff: TimeSerVec<Tick>,
    pub ticks_buff2: TimeSerVec<Tick>, // for internal frame process
}

impl NERoot {
    pub fn new() -> Self {
        let cfg = CandleConfig {
            small_tick: 50,
            medium_tick: 3,
            big_tick: 9,
            vel1_period: 10,
            vel2_period: 100,
        };

        Self {
            frame_id: 0,
            cfg: cfg.clone(),
            frames: vec![],
            candles: CandleSeriesTA::new(&cfg),
            ticks_buff: Default::default(),
            ticks_buff2: Default::default(),
        }
    }

    pub fn add_tick(&mut self, tick: &Tick) -> Option<NEFrame> {
        self.ticks_buff.push(tick.clone());
        self.ticks_buff2.push(tick.clone());
        if self.ticks_buff.len() == 50 {
            // number should always be dived to meidum tikc size
            let frame = self.build_next_frame();
            let last = self.frames.last();
            match last {
                None => {
                    // self.frames.push(frame);
                }
                Some(f) => {
                    if f.fid == frame.fid {
                        self.frames.pop();
                    }
                }
            }
            self.frames.push(frame.clone());
            self.ticks_buff.clear();
            Some(frame)
        } else {
            None
        }
    }

    fn build_next_frame(&mut self) -> NEFrame {
        let mut frame = NEFrame::default();
        let tick = self.ticks_buff.last().unwrap();
        self.candles.add_ticks(self.ticks_buff.clone());
        self.ticks_buff.clear();

        let k_med = self.candles.medium.kline_ta_tip.clone().unwrap();
        let k_big = self.candles.big.kline_ta_tip.clone().unwrap();

        let mut frame = new_frame(&k_med, &k_big);
        frame.set_spread(&self.ticks_buff2);
        // be aware not go out of sync - could be a bug source
        if self.ticks_buff2.len() as u64 == self.cfg.small_tick * self.cfg.medium_tick {
            self.ticks_buff2.clear();
        }
        frame
    }
}

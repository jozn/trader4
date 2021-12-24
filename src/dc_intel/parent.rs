use super::*;
use crate::base::*;
use crate::candle::{Tick, TimeSerVec};
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DCParent {
    pub frame_id: u64, // For next frame id
    pub frames: Vec<FrameMem>,

    // TA holders
    pub dc_med: DC,
    pub dc_big: DC,
    pub ma1: EMA,
    pub vel: Vel,

    // Others
    pub ticks_buff: TimeSerVec<Tick>,
}

impl DCParent {
    pub fn new() -> Self {
        Self {
            frame_id: 0,
            frames: vec![],
            dc_med: DC::new(20).unwrap(),
            dc_big: DC::new(60).unwrap(),
            ma1: EMA::new(50).unwrap(),
            vel: Vel::new(200).unwrap(),
            ..Default::default()
        }
    }
    pub fn add_tick(&mut self, tick: &Tick) {
        self.ticks_buff.push(tick.clone());
        if self.ticks_buff.len() == 200 {
            self.run_if_next_frame();
        }
    }

    // todo remove
    pub fn add_ticks(&mut self, ticks: &TimeSerVec<Tick>) {
        if ticks.len() == 0 {
            // println!(">> Trades are empty.");
            return;
        }
        for t in ticks.iter() {
            self.ticks_buff.push(t.clone());
            self.run_if_next_frame();
        }
    }

    fn run_if_next_frame(&mut self) {
        if self.ticks_buff.len() == 200 {
            let mut frame = FrameMem::default();
            frame.add_ticks(&self.ticks_buff);
            self.frame_id += 1;
            frame.frame_id = self.frame_id;
            let dc_res = self.dc_med.next(&frame.ohlc);
            frame.med_high = dc_res.high;
            frame.med_low = dc_res.low;
            let dc_res = self.dc_big.next(&frame.ohlc);
            frame.big_high = dc_res.high;
            frame.big_low = dc_res.low;
            frame.ma1 = self.ma1.next(frame.ohlc.hlc3());
            frame.vel = self.vel.next(frame.ohlc.hlc3());

            frame.finished = true;
            self.frames.push(frame);
            self.ticks_buff.clear();
        }
    }
}

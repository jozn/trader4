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

    // todo extract to a struct
    // TA holders
    pub dc_med: DC,
    pub dc_big: DC,
    pub ma1: EMA,
    pub ma2: WMA,
    pub vel: Vel,
    pub vel2: Vel2,
    pub atr: ATR,

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
            ma2: WMA::new(50).unwrap(),
            vel: Vel::new(1000).unwrap(),
            vel2: Vel2::new(20).unwrap(),
            atr: ATR::new(14).unwrap(),
            ..Default::default()
        }
    }
    pub fn add_tick(&mut self, tick: &Tick) -> Option<FrameMem> {
        self.ticks_buff.push(tick.clone());
        if self.ticks_buff.len() == 150 {
            let frame = self.build_next_frame();
            self.frames.push(frame.clone());
            self.ticks_buff.clear();
            Some(frame)
        } else {
            None
        }
    }

    fn build_next_frame(&mut self) -> FrameMem {
        let mut frame = FrameMem::default();
        let tick = self.ticks_buff.last().unwrap();
        frame.add_ticks(&self.ticks_buff);

        // Counter
        self.frame_id += 1;
        frame.fid = self.frame_id;

        // Add TA to frame
        let dc_res = self.dc_med.next(&frame.ohlc);
        frame.med_high = dc_res.high;
        frame.med_low = dc_res.low;
        frame.med_mid = frame.get_med_middle();
        frame.med_dc_hl_pip = (dc_res.high - dc_res.low) * 10_000.;
        let dc_res = self.dc_big.next(&frame.ohlc);
        frame.big_low = dc_res.low;
        frame.big_high = dc_res.high;
        frame.big_mid = (frame.big_high + frame.big_low) / 2.;
        frame.big_dc_hl_pip = (dc_res.high - dc_res.low) * 10_000.;
        frame.ma1 = self.ma1.next(frame.ohlc.hlc3());
        frame.ma2 = self.ma2.next(frame.ohlc.hlc3());
        frame.vel = self.vel.next(frame.ohlc.hlc3());
        frame.vel2 = self.vel2.next(frame.ohlc.hlc3());
        // frame.vel2 = self.vel2.next(frame.big_mid);
        frame.atr_p = self.atr.next(&frame.ohlc) * 10_000.;

        frame.set_trend();

        let dc_str = get_strength(&frame, &self.frames, tick);
        frame.finished = true;
        frame.dc_strength = dc_str;

        frame
    }
}

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
    pub frame_id: u64, // For next frame id
    pub frames: Vec<FrameMem>,
    pub candles: CandleSeriesTA,
    pub ticks_buff: TimeSerVec<Tick>,
}

impl NERoot {
    pub fn new() -> Self {
        Self {
            frame_id: 0,
            frames: vec![],
            candles: CandleSeriesTA::new(&CandleConfig {
                small_tick: 50,
                medium_tick: 3,
                big_tick: 9,
                vel1_period: 10,
                vel2_period: 100,
            }),
            ticks_buff: Default::default(),
        }
    }
    pub fn add_tick(&mut self, tick: &Tick) -> Option<FrameMem> {
        self.ticks_buff.push(tick.clone());
        if self.ticks_buff.len() == 50 {
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
        self.candles.add_ticks(self.ticks_buff.clone());
        self.ticks_buff.clear();

        let k_med = self.candles.medium.kline_ta_tip.clone().unwrap();
        let k_big = self.candles.big.kline_ta_tip.clone().unwrap();

        frame
    }

    /*fn build_next_frame_bk(&mut self) -> FrameMem {
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
        frame.rsi = self.rsi.next(frame.ohlc.hlc3());
        frame.rsi_sth = self.rsi_stoch.next(frame.ohlc.hlc3());

        frame.set_trend();

        let dc_str = get_strength(&frame, &self.frames, tick);
        frame.finished = true;
        frame.dc_strength = dc_str;

        frame
    }*/
}

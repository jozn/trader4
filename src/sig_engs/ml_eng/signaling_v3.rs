use super::*;
use crate::app;
use crate::bar::MultiBarRes;
use crate::collector::row_data::BTickData;
use crate::cortex::*;
use rand::Rng;

impl MLEng {
    pub fn set_signals_for_ml_v1(
        &mut self,
        tick: &BTickData,
        sf: &mut MLFrame,
        mbr: &MultiBarRes,
    ) -> Option<ActionSignal> {
        let pair = tick.pair;
        let kid = self.mutli_bars.medium_bars.primary_seq;
        let kid_small = self.mutli_bars.small_bars.primary_seq;
        let time = app::clock::get_clock_time_sec();
        let pro = mbr.medium.primary.ta.dc.height_pop();

        let rp = mbr.medium.primary.ta.rel_price.clone();
        let rp = mbr.medium.big.ta.rel_price.clone();
        let mut cortex = self.get_cortex_mut();

        let small_ta = &mbr.small.big.ta;
        let med_ta = &mbr.medium.primary.ta;
        let major_ta = &mbr.major.primary.ta;
        let small_trend_ch = &small_ta.trend;
        let mut open_pos = false;

        if small_trend_ch.bull_above && kid_small > 30 {
            // if med_ta.rel_price.os_stoch_main < 20. {
            if major_ta.td.diff > 0. && med_ta.rel_price.os_index < 0.20 {
                let act = Some(ActionSignal {
                    small_kid: kid_small,
                    consumed: false,
                    long: true,
                    profit: pro * 0.80,
                    loss: -pro * 0.40,
                    time_sec: tick.timestamp_sec,
                    frame_insight: sf.insight.clone(),
                });
                return act;
            }
        }

        None
    }
}

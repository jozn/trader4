use super::*;
use crate::bar::MultiBarRes;
use crate::collector::row_data::BTickData;
use crate::cortex::types::ActionSignal;
use rand::Rng;

impl MLEng {
    pub fn set_signals_v1(
        &mut self,
        tick: &BTickData,
        sf: &mut MLFrame,
        mbr: &MultiBarRes,
    ) -> Option<ActionSignal> {
        let kid = self.mutli_bars.medium_bars.primary_seq;
        let kid_small = self.mutli_bars.small_bars.primary_seq;
        let pro = 0.001;
        let pro = mbr.medium.primary.ta.dc.height_pop() * 1.;
        let r: f32 = rand::thread_rng().gen();

        // seems big.primary returns the best resutls
        // let mva = &mbr.major.primary.ta.vel;
        let mva_med = &mbr.medium.primary.ta.vel_mom;
        let mva_big = &mbr.major.primary.ta.vel_mom;
        // let mva_big = &mbr.major.big.ta.vel_mom;
        let rdc = &mbr.medium.primary.ta.rdc;

        // if mva_big.count > 5 && mva_big.ma_mom > 0. && mva_med.count > 4 && mva_med.ma_mom >0.{
        if mva_big.count > 5 && mva_big.ma_mom > 0. {
            if rdc.perc_med < 0.2 {
                self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
                self.cortex_mem.set_action(&ActionSignal {
                    small_kid: kid_small,
                    consumed: false,
                    long: true,
                    profit: pro * 0.80,
                    loss: -pro * 0.40,
                    time_sec: tick.timestamp_sec,
                });
            }
        }
        None
    }

    pub fn set_signals_random1(
        &mut self,
        tick: &BTickData,
        sf: &mut MLFrame,
        mbr: &MultiBarRes,
    ) -> Option<ActionSignal> {
        let kid = self.mutli_bars.medium_bars.primary_seq;
        let kid_small = self.mutli_bars.small_bars.primary_seq;
        let pro = 0.001;
        let pro = mbr.medium.primary.ta.dc.height_pop();
        let r: f32 = rand::thread_rng().gen();

        if kid % 10 == 0 {
            // if r > 0.8 {
            // println!("{}",kid);
            self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
            self.cortex_mem.set_action(&ActionSignal {
                small_kid: kid_small,
                consumed: false,
                long: true,
                profit: pro * 0.80,
                loss: -pro * 0.40,
                time_sec: tick.timestamp_sec,
            });
        } else {
        }

        None
    }
}

use super::*;
use crate::app;
use crate::bar::MultiBarRes;
use crate::collector::row_data::BTickData;
use crate::cortex::*;
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
                // self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
                // self.cortex_mem.set_action(&);
                // let co =
                Some(ActionSignal {
                    small_kid: kid_small,
                    consumed: false,
                    long: true,
                    profit: pro * 0.80,
                    loss: -pro * 0.40,
                    time_sec: tick.timestamp_sec,
                    frame_insight: sf.insight.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_signals_random1(
        &mut self,
        tick: &BTickData,
        sf: &mut MLFrame,
        mbr: &MultiBarRes,
    ) -> Option<ActionSignal> {
        let pair = tick.pair;
        let kid = self.mutli_bars.medium_bars.primary_seq;
        let kid_small = self.mutli_bars.small_bars.primary_seq;
        let pro = 0.001;
        let pro = mbr.medium.primary.ta.dc.height_pop();
        let r: f32 = rand::thread_rng().gen();
        let mut cor = self.get_cortex_mut();
        // cor.flags.add_once();

        let time = app::clock::get_clock_time_sec();

        if kid % 5 == 0 {
            let sig = FlagsRow {
                flag_id: 0,
                pair,
                eng_key: ML_ENG,
                type_key: EARLY_LONG,
                medium_bar_id: kid,
                small_bar_id: kid_small,
                time_sec: time,
                ttl: 0,
            };
            cor.flags.add_once_small(&sig);
        }

        if kid % 8 == 0 {
            let sig = FlagsRow {
                flag_id: 0,
                pair,
                eng_key: ML_ENG,
                type_key: FINAL_LONG,
                medium_bar_id: kid,
                small_bar_id: kid_small,
                time_sec: time,
                ttl: 0,
            };
            cor.flags.add_once_small(&sig);
        }

        if kid % 10 == 0 {
            // if r > 0.8 {
            // println!("{}",kid);
            // self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
            // self.cortex_mem.set_action(&);
            Some(ActionSignal {
                small_kid: kid_small,
                consumed: false,
                long: true,
                profit: pro * 0.80,
                loss: -pro * 0.40,
                time_sec: tick.timestamp_sec,
                frame_insight: sf.insight.clone(),
            })
        } else {
            None
        }
    }
}

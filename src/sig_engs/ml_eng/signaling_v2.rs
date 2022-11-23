use super::*;
use crate::app;
use crate::bar::MultiBarRes;
use crate::collector::row_data::BTickData;
use crate::cortex::*;
use rand::Rng;

impl MLEng {
    pub fn set_signals_random2(
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

        let mut open_pos = false;
        if rp.os_stoch_main < 0.2 {
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
            cortex.flags.add_once_small(&sig);
        }

        let trend_ch = &mbr.small.big.ta.trend;
        if trend_ch.bull_above && kid_small > 50 {
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
            cortex.flags.add_once_small(&sig);
            open_pos = true;
        }

        let tmaj = &mbr.major.big.ta.td;
        if tmaj.diff < 0. {
            return None;
        }

        if open_pos {
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
        // None
    }

    pub fn set_signals_random2_bk(
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

        let time = app::clock::get_clock_time_sec();

        if kid_small % 150 == 0 {
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

        if kid_small % 239 == 0 {
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

        let stoc = mbr.medium.primary.ta.rel_price.os_stoch_main;
        if stoc > 0.2 {
            return None;
        }
        let ts = &sf.score;
        if ts.diff < 0. {
            return None;
        }

        if kid_small % 50 == 0 {
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

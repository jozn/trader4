use super::*;
use crate::collector::row_data::BTickData;
use crate::cortex::types::ActionSignal;
use rand::Rng;

impl MLEng {
    pub fn set_signals_v5(&mut self, tick: &BTickData, sf: &mut MLFrame) -> Option<ActionSignal> {
        let kid = self.mutli_bars.medium_bars.primary_seq;
        let kid_small = self.mutli_bars.small_bars.primary_seq;
        let pro = 0.001;
        let r: f32 = rand::thread_rng().gen();

        if kid % 10 == 0 {
            // if r > 0.8 {
            // println!("{}",kid);
            self.cortex_mem.mark_long_final(kid, tick.timestamp_sec);
            self.cortex_mem.set_action(&ActionSignal {
                small_kid: kid_small,
                consumed: false,
                long: true,
                profit: pro * 3.,
                loss: -pro * 1.5,
                time_sec: tick.timestamp_sec,
            });
        } else {
        }

        None
    }
}

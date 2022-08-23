use super::*;
use crate::collector::row_data::BTickData;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BarSeries {
    pub cfg: BarConfig,
    pub primary_seq: i32,
    pub big_seq: i32,
    pub ticks_primary: Vec<BTickData>,
    pub ticks_big: Vec<BTickData>,
    pub bars_primary: Vec<PrimaryHolder>,
    pub bars_big: Vec<Bar>,
    primary_ta: TAMethods,
    big_ta: TAMethods,
}

impl BarSeries {
    pub fn new(cfg: &BarConfig) -> BarSeries {
        assert!(cfg.big_ticks >= cfg.primary_ticks);
        assert!(cfg.big_ticks % cfg.primary_ticks == 0);

        BarSeries {
            cfg: cfg.clone(),
            primary_seq: 1,
            big_seq: 1,
            ticks_primary: vec![],
            ticks_big: vec![],
            bars_primary: vec![],
            bars_big: vec![],
            primary_ta: TAMethods::new(&cfg),
            big_ta: TAMethods::new(&cfg),
        }
    }

    pub fn add_ticks(&mut self, ticks: Vec<BTickData>) {
        if ticks.len() == 0 {
            println!(">> Trades are empty.");
            return;
        }

        let mut last_time = ticks.first().unwrap().timestamp;
        for t in &ticks {
            if t.timestamp < last_time {
                println!(">> Ticks time are invalid");
                debug_assert!(t.timestamp < last_time);
                return; // in live
            }
            last_time = t.timestamp;
        }

        for t in &ticks {
            self.add_tick_mut(t);
        }
    }

    pub fn add_tick_mut(&mut self, tick: &BTickData) -> Option<PrimaryHolder> {
        self.ticks_primary.push(tick.clone());
        self.ticks_big.push(tick.clone());

        // we only run this block when we have enough ticks for primary
        if self.ticks_primary.len() == self.cfg.primary_ticks as usize {
            let mut finish_big = false;
            let mut bar_prim = Bar::new(&self.ticks_primary);
            bar_prim.seq = self.primary_seq;
            bar_prim.ta = cal_indicators(&mut self.primary_ta, &bar_prim);

            let mut bar_big = Bar::new(&self.ticks_big);
            bar_big.seq = self.big_seq;

            if self.ticks_big.len() == self.cfg.big_ticks as usize {
                bar_big.ta = cal_indicators(&mut self.big_ta, &bar_big);
                self.bars_big.push(bar_big.clone());
                finish_big = true;
                self.ticks_big.clear();
                self.big_seq += 1;
            } else {
                // IMPORTANT: Clone methods
                bar_big.ta = cal_indicators(&mut self.big_ta.clone(), &bar_big);
            }

            self.ticks_primary.clear();
            let ph = PrimaryHolder {
                primary: bar_prim.clone(),
                big: bar_big.clone(),
                finish_primary: true,
                finish_big,
            };
            self.bars_primary.push(ph.clone());
            self.primary_seq += 1;

            Some(ph)
        } else {
            // in here we could also build new Bars without changing states
            //  we already build .build_ph_tip()
            // inactive now
            // Some(self.build_ph_tip())
            None
        }
    }

    // build PH for not filled ticks bars. No modification to state.
    pub fn build_ph_tip(&self) -> PrimaryHolder {
        if self.ticks_primary.len() == 0 {
            if self.bars_primary.len() == 0 {
                // we should never be in here
                println!("warning! empty bars in build_ph");
                return PrimaryHolder::default();
            }
            self.bars_primary.last().unwrap().clone()
        } else {
            let mut bar_prim = Bar::new(&self.ticks_primary);
            bar_prim.seq = self.primary_seq;
            bar_prim.ta = cal_indicators(&mut self.primary_ta.clone(), &bar_prim); // note: clone

            let mut bar_big = Bar::new(&self.ticks_big);
            bar_big.seq = self.big_seq;
            bar_big.ta = cal_indicators(&mut self.big_ta.clone(), &bar_big); // note: clone
            let ph = PrimaryHolder {
                primary: bar_prim.clone(),
                big: bar_big.clone(),
                finish_primary: false,
                finish_big: false,
            };
            ph
        }
    }

    pub fn get_primary_bars(&self, start: i64, end: i64) -> Vec<Bar> {
        let mut out = vec![];
        for ph in &self.bars_primary {
            let b = &ph.primary;
            if b.open_time >= start && b.open_time <= end {
                out.push(b.clone())
            }
        }
        out
    }

    pub fn get_bars_ph(&self, start_time: i64, end_time: i64) -> Vec<PrimaryHolder> {
        let mut out = vec![];
        let niddle_opt = self
            .bars_primary
            .binary_search_by(|o| o.primary.open_time.cmp(&start_time));
        let idx = match niddle_opt {
            Ok(i) => i,
            Err(i) => i,
        };
        let idx = (idx as i64 - 2).max(0) as usize; // go 2 index before
        for ph in self.bars_primary.iter().skip(idx) {
            let b = &ph.primary;
            if b.open_time >= start_time && b.open_time <= end_time {
                out.push(ph.clone())
            }
        }

        out
    }

    pub(super) fn get_bars_first_last(&self, size: i64) -> Vec<PrimaryHolder> {
        let mut out = vec![];
        let len = self.bars_primary.len();
        let mut indx = 0; //todo: vec iter with index
        for b in self.bars_primary.iter() {
            if size > 0 &&  indx  <= size {
                out.push(b.clone());
            }
            if size < 0 &&  indx >= len as i64 + size {
                out.push(b.clone());
            }
            indx += 1;
        }
        out
    }

    pub(super) fn get_bars_index(&self, rng: Range<usize>) -> Vec<PrimaryHolder> {
        let mut out = vec![];
        let mut indx = 0; //todo: vec iter with index
        for b in self.bars_primary.iter() {
            if rng.contains(&indx) {
                out.push(b.clone())
            }
        }
        out
    }
}

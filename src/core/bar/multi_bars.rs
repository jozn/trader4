use std::ops::Range;
// use crate::bar::{BarConfig, BarSeries, PrimaryHolder};
use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiBars {
    pub major_cfg: BarConfig,
    pub major_bars: BarSeries,
    pub medium_cfg: BarConfig,
    pub medium_bars: BarSeries,
    pub small_cfg: BarConfig, // Price hunter
    pub small_bars: BarSeries,
}

// Is the respone of adding ticks. _full reperesent when bar is completed, if false their
//  coresponding bars response (PrimaryHolder) is of bar "tip" types and should be treated
//  for temporary usage.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MultiBarRes {
    pub small: PrimaryHolder,
    pub medium: PrimaryHolder,
    pub major: PrimaryHolder,
    pub small_full: bool,
    pub medium_full: bool,
    pub major_full: bool,
    pub smalls: Vec<PrimaryHolder>,
}

impl MultiBars {
    pub fn new(pair: &Pair) -> Self {
        // todo: migrate this
        let primary_ticks = if pair.is_forex() {
            150
        } else if pair.is_us_stocks() {
            300
        } else if pair.is_crypto() {
            300
        } else {
            300
        };

        let major_ticks = primary_ticks * 4;
        let major_cfg = BarConfig {
            primary_ticks: major_ticks,
            big_ticks: major_ticks * 2,
        };

        // let primary_ticks = 150;
        let medium_cfg = BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        };

        // let small_ticks = 10;
        let small_ticks = primary_ticks / 10;
        let small_cfg = BarConfig {
            primary_ticks: small_ticks,
            big_ticks: small_ticks * 3,
        };

        Self {
            major_cfg: major_cfg.clone(),
            major_bars: BarSeries::new(&major_cfg),
            medium_cfg: medium_cfg.clone(),
            medium_bars: BarSeries::new(&medium_cfg),
            small_cfg: small_cfg.clone(),
            small_bars: BarSeries::new(&small_cfg),
        }
    }

    pub fn add_tick(&mut self, tick: &BTickData) -> Option<MultiBarRes> {
        let ph_major = self.major_bars.add_tick_mut(tick);
        let ph_medium = self.medium_bars.add_tick_mut(tick);
        let ph_small = self.small_bars.add_tick_mut(tick);

        // Note: we can easily return Some(data) when small is not full, but it has
        //  a performance penality in offline simiulation for now only run logics
        //  when small bar is really full (no tip).

        let out = match ph_small.clone() {
            None => None,
            Some(ph_small) => {
                let small_full = true;
                let mut medium_full = false;
                let mut major_full = false;

                let ph_medium = match ph_medium.clone() {
                    None => self.medium_bars.build_ph_tip(),
                    Some(ph_med) => {
                        medium_full = true;
                        ph_med
                    }
                };

                let ph_major = match ph_major.clone() {
                    None => self.major_bars.build_ph_tip(),
                    Some(ph_big) => {
                        major_full = true;
                        ph_big
                    }
                };

                let smalls = self
                    .small_bars
                    .get_bars_ph(ph_medium.primary.open_time - 1, i64::MAX);

                Some(MultiBarRes {
                    small: ph_small,
                    medium: ph_medium,
                    major: ph_major,
                    small_full,
                    medium_full,
                    major_full,
                    smalls,
                })
            }
        };
        out
    }

    pub fn get_bars_dump(&self, size: i64) -> DumpDebugMultiBar {
        // let size = size as usize;
        // let med_len = self.major_bars.bars_primary.len();
        // if size * 2 > med_len {
        //     return DumpDebugMultiBar::default();
        // }
        // let mid = self.medium_bars.bars_primary.get(size as usize).unwrap();
        // let end_inx = self.major_bars.bars_primary.len() - size as usize;
        // let mid_end = self.medium_bars.bars_primary.get(end_inx).unwrap();
        DumpDebugMultiBar {
            first_bars: self._get_bars(size),
            last_bars: self._get_bars(-size),
        }
    }
    fn _get_bars(&self, count: i64) -> DumpDebugBar {
        DumpDebugBar {
            big: self.major_bars.get_bars_first_last(count),
            med: self.medium_bars.get_bars_first_last(count),
            small: self.small_bars.get_bars_first_last(count),
        }
    }

    fn _get_bars_old(&self, start: i64, end: i64) -> DumpDebugBar {
        DumpDebugBar {
            big: self.major_bars.get_bars_ph(start, end),
            med: self.medium_bars.get_bars_ph(start, end),
            small: self.small_bars.get_bars_ph(start, end),
        }
    }

    // pub fn get_bars_ph(&self, start: i64, end: i64) -> Self {
    //     Self {
    //         major_cfg: self.major_cfg.clone(),
    //         major_bars: self.major_bars.get_bars_ph(start, end),
    //         medium_cfg: BarConfig {},
    //         medium_bars: (),
    //         small_cfg: BarConfig {},
    //         small_bars: (),
    //     }
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DumpDebugMultiBar {
    pub first_bars: DumpDebugBar,
    pub last_bars: DumpDebugBar,
    // pub bars_big: Vec<Bar>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DumpDebugBar {
    pub big: Vec<PrimaryHolder>,
    pub med: Vec<PrimaryHolder>,
    pub small: Vec<PrimaryHolder>,
}

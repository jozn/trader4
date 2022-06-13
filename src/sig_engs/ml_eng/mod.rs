use crate::bar::{BarConfig, BarSeries, PrimaryHolder};
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::eng_memory::CortexMem;
use crate::cortex::types::{ActionSignal, SignalMem};
use serde::{Deserialize, Serialize};

// Sky Engine
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLEng {
    pub cortex_mem: CortexMem,
    pub frames: Vec<MLFrame>,
    pub mutli_bars: MultiBars,
}

impl MLEng {
    pub fn new(pair: &Pair) -> Self {
        MLEng {
            cortex_mem: CortexMem::new(),
            frames: vec![],
            mutli_bars: MultiBars::new(pair),
        }
    }
    pub fn add_tick(&mut self, tick: &BTickData) {
        let mul_res = self.mutli_bars.add_tick(tick);
        match mul_res {
            None => {}
            Some(mr) => {
                let mut frame = new_frame(&mr);

                if mr.medium_full {
                    self.frames.push(frame);
                }
            }
        }
    }
}

// pub fn new_frame(ph_medium: &PrimaryHolder, ph_major: &PrimaryHolder) -> MLFrame {}
pub fn new_frame(mbr: &MultiBarRes) -> MLFrame {
    let p = &mbr.medium.primary;
    let pta = &mbr.medium.primary.ta;
    let b = &mbr.medium.big;
    let bta = &mbr.medium.big.ta;

    let mut f = MLFrameInfo {
        med_low: pta.dc.low,
        med_high: pta.dc.high,
        med_mid: pta.dc.middle,
        big_low: bta.dc.low,
        big_high: bta.dc.high,
        big_mid: bta.dc.middle,
        med_dc_hl_pip: (pta.dc.high - pta.dc.low) * 10_000.,
        big_dc_hl_pip: (bta.dc.high - bta.dc.low) * 10_000.,
        bar_major: mbr.major.clone(),
        bar_medium: mbr.medium.clone(),
        bars_small: mbr.smalls.clone(),
        bar_small_tip_: Default::default(),
    };
    MLFrame {
        fid: p.seq,
        info: f,
        signal_mem: None,
        signal_action: None,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrame {
    pub fid: i32, // frame_id
    pub info: MLFrameInfo,
    #[serde(skip)]
    pub signal_mem: Option<SignalMem>,
    #[serde(skip)]
    pub signal_action: Option<ActionSignal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameInfo {
    // Donchain Channel
    pub med_low: f64,
    pub med_high: f64,
    #[serde(skip)]
    pub med_mid: f64,
    pub big_low: f64,
    pub big_high: f64,
    #[serde(skip)]
    pub big_mid: f64,

    pub med_dc_hl_pip: f64,
    pub big_dc_hl_pip: f64,
    #[serde(skip)]
    pub bar_major: PrimaryHolder,
    #[serde(skip)]
    pub bar_medium: PrimaryHolder,
    #[serde(skip)]
    pub bars_small: Vec<PrimaryHolder>,
    #[serde(skip)]
    pub bar_small_tip_: PrimaryHolder,
}

// todo: to core
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
                    None => self.medium_bars.build_ph_tip(),
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

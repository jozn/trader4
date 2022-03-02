use super::*;
use crate::bar::*;
use crate::base::*;
use crate::brain::{PairMemory, SignalsDB};
use crate::collector::import_all::BTickData;
use crate::helper;
use crate::ta::*;
use crate::types::{ActionSignalDep, SignalMemDep};
use serde::{Deserialize, Serialize};
use crate::cortex::eng_memory::CortexMem;
use crate::cortex::types::ActionSignal;

// Sky Engine
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkyEng {
    pub signal_mem: Option<SignalMemDep>,
    pub cortex_mem: CortexMem,
    pub frames: Vec<SFrame>,
    pub major_cfg: BarConfig,
    pub major_bars: BarSeries,
    pub medium_cfg: BarConfig,
    pub medium_bars: BarSeries,
    pub small_cfg: BarConfig, // Price hunter
    pub small_bars: BarSeries,
}

impl SkyEng {
    pub fn new() -> Self {
        let major_ticks = 600;
        let major_cfg = BarConfig {
            primary_ticks: major_ticks,
            big_ticks: major_ticks * 2,
        };

        let primary_ticks = 150;
        let medium_cfg = BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        };

        let small_ticks = 10;
        let small_cfg = BarConfig {
            primary_ticks: small_ticks,
            big_ticks: small_ticks * 3,
        };

        Self {
            signal_mem: None,
            cortex_mem: CortexMem::new(),
            frames: vec![],
            major_cfg: major_cfg.clone(),
            major_bars: BarSeries::new(&major_cfg),
            medium_cfg: medium_cfg.clone(),
            medium_bars: BarSeries::new(&medium_cfg),
            small_cfg: small_cfg.clone(),
            small_bars: BarSeries::new(&small_cfg),
        }
    }

    pub fn add_tick(&mut self, tick: &BTickData, mem: &mut SignalsDB) -> Option<ActionSignal> {
        let ph_big = self.major_bars.add_tick_mut(tick);
        let ph_medium = self.medium_bars.add_tick_mut(tick);
        let ph_small = self.small_bars.add_tick_mut(tick);

        match ph_small {
            None => None,
            Some(ph_small) => {
                let ph_med = match ph_medium.clone() {
                    None => self.medium_bars.build_ph_tip(),
                    Some(ph_med) => ph_med,
                };
                let time_bar_med = ph_med.primary.get_open_time_sec();
                let smalls = self
                    .small_bars
                    .get_bars_ph(ph_med.primary.open_time - 1, i64::MAX);
                // println!("len >>> {}", smalls.len());
                let ph_major = self.major_bars.build_ph_tip();

                let mut frame = new_frame(&ph_med, &ph_major);
                frame.bars_small = smalls;
                frame.bar_small_tip = ph_small;
                let _act = self.set_signals_random(tick, &mut frame);
                let act = self.cortex_mem.consume_action(time_bar_med);
                // let act = frame.set_scalper_dep(tick, mem);
                // self.add_signs(&frame);
                if ph_medium.is_some() {
                    // if self.signal_mem.is_some() {
                    //     frame.
                    // }
                    // println!("ttt");
                    frame.signal_mem = self.cortex_mem.get_snapshot(time_bar_med);
                    frame.signal_action = self.cortex_mem.get_action(time_bar_med);
                    // println!("ttt {:?}", self.cortex_mem.get_action(time_bar_med));
                    self.cortex_mem.clear_old(time_bar_med);
                    self.frames.push(frame.clone());
                };
                act
                // None //todo
                // self.cortex_mem.consume_action()
            }
        }
    }

    /*pub fn add_tick_dep(&mut self, tick: &BTickData, mem: &mut SignalsDB) -> Option<ActionSignal> {
            let ph_big = self.major_bars.add_tick_mut(tick);
            let ph_medium = self.medium_bars.add_tick_mut(tick);
            let ph_small = self.small_bars.add_tick_mut(tick);

            match ph_small {
                None => None,
                Some(ph_small) => {
                    let ph_med = match ph_medium.clone() {
                        None => self.medium_bars.build_ph_tip(),
                        Some(ph_med) => ph_med,
                    };
                    let smalls = self
                        .small_bars
                        .get_bars_ph(ph_med.primary.open_time - 1, i64::MAX);
                    // println!("len >>> {}", smalls.len());
                    let ph_major = self.major_bars.build_ph_tip();

                    let mut frame = new_frame(&ph_med, &ph_major);
                    frame.bars_small = smalls;
                    frame.bar_small_tip = ph_small;
                    let act = frame.set_scalper_dep(tick, mem);
                    // self.add_signs(&frame);
                    if ph_medium.is_some() {
                        let sig =mem.get_signal("sky1");
                        if sig.is_some() {
                            let sig = sig.unwrap();
                            if sig.final_buy {
                                frame.buy2_dep = true;
                                mem.get_signal("sky1");
                            }
                        }
                        frame.buy2_dep = true;
                        // frame.buy1 = true;
                        frame.sell2_dep = true;
                        // frame.sell1 = true;
                        self.frames.push(frame.clone());
                        // act
                        // None // todo
                    };
                    act
                }
            }
        }
    */
}

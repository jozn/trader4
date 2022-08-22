use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// A memory engine to be embeded in each sig_engs (SkyEng, MLEng)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CortexMem {
    pub signal_mem: Option<SignalMem>,
    pub action: Option<ActionSignal>,
}

impl CortexMem {
    pub fn new() -> Self {
        Self {
            signal_mem: None,
            action: None,
        }
    }

    // Early signal to remember for going long
    pub fn mark_long_early(&mut self, med_kid: i32, time_sec: i64) {
        match self.signal_mem {
            None => {
                self.signal_mem = Some(SignalMem {
                    ps_buy: true,
                    ps_medium_bar_id: med_kid,
                    ps_time_sec: time_sec,
                    fs_buy: false,
                    fs_small_bar_id: 0,
                    fs_time_sec: 0,
                });
            }
            Some(_) => {}
        }
    }
    pub fn mark_long_final(&mut self, med_kid: i32, time_sec: i64) {
        let mut mem = match &self.signal_mem {
            None => SignalMem {
                ps_buy: true,
                ps_medium_bar_id: med_kid,
                ps_time_sec: time_sec,
                fs_buy: false,
                fs_small_bar_id: 0,
                fs_time_sec: 0,
            },
            Some(sm) => sm.clone(),
        };
        mem.fs_buy = true;
        mem.fs_time_sec = time_sec;
        mem.fs_small_bar_id = med_kid;
    }
    pub fn cancel_long_mark(&mut self) {
        self.signal_mem = None
    }
    pub fn get_snapshot(&self, med_kid: i32) -> Option<SignalMem> {
        // self.signal_mem.clone()
        match &self.signal_mem {
            None => None,
            Some(sm) => {
                if sm.ps_medium_bar_id == med_kid || med_kid == 0 {
                    Some(sm.clone())
                } else {
                    None
                }
            }
        }
    }
    pub fn set_action(&mut self, act: &ActionSignal) {
        let act = act.clone();
        assert_eq!(act.consumed, false);
        self.action = Some(act);
    }
    // only retuned once
    pub fn consume_action(&mut self, time_sec: i64) -> Option<ActionSignal> {
        match &mut self.action {
            None => None,
            Some(act) => {
                if act.consumed {
                    None
                } else {
                    act.consumed = true;
                    Some(act.clone())
                }
            }
        }
    }

    pub fn get_action(&mut self, time_sec: i64) -> Option<ActionSignal> {
        match &self.action {
            None => None,
            Some(act) => Some(act.clone()),
        }
    }

    // Clear data at the end of medium Bars if action is set (one act per Bar max)
    pub fn clear_old(&mut self, time_sec: i64) {
        match &self.action {
            None => {}
            Some(act) => {
                assert!(act.consumed);
                self.signal_mem = None;
                self.action = None;
            }
        };
    }
}

use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::{GateWay, NewPos, PosRes, UpdatePos};
use crate::offline::num5;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

impl Brain3 {
    pub fn update_all_tailing_pos(&mut self) {
        for (_, p) in self.open.clone() {
            // self.on_full_tailing_pl(p);
            // todo enalbe
            // self.on_tailing(p);
        }
    }
}

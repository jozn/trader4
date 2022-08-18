use crate::app;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::{Cortex, CortexRef};
use crate::gate_api::GateWay;
use crate::sig_engs::ml_eng::MLEng;
use std::cell::RefMut;
use std::sync::Arc;

// todo: change the name
#[derive(Debug)]
pub struct Brain {
    pub con: Box<Arc<dyn GateWay>>,
    pub cortex: CortexRef,
    pub db: Vec<PairSigHolder>,
}

impl Brain {
    fn get_cortex_mut(&self) -> RefMut<Cortex> {
        self.cortex.as_ref().borrow_mut()
    }

    fn play_cortex(&self) {
        // Play Cortex mut
        let mut cort = self.cortex.as_ref().borrow_mut();
        cort.policy += 2.0;
        drop(cort);
    }

    fn init_pair(&mut self, pair: &Pair) {
        if !self.db.iter().any(|ps| &ps.pair == pair) {
            self.db.push(PairSigHolder {
                pair: Default::default(),
                ml_eng: MLEng::new(pair, self.cortex.clone()),
            })
        }
    }

    pub fn on_price_tick(&mut self, pair: &Pair, tick: BTickData) {
        app::clock::set_clock_time(tick.timestamp);
        self.init_pair(pair);

        let mut cortex = self.get_cortex_mut();
        cortex.on_price_tick(pair, tick.clone());

        drop(cortex);

        for ps in self.db.iter_mut() {
            if &ps.pair == pair {
                ps.ml_eng.add_tick(&tick);
            }
        }

        // todo: check new orders, updates, ... from Cortex and send it to gateway
    }
}

#[derive(Debug)]
pub struct PairSigHolder {
    pub pair: Pair,
    pub ml_eng: MLEng,
}

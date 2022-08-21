use crate::bar::BarConfig;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::{Cortex, CortexRef};
use crate::gate_api::{EventPosition, GateWay};
use crate::sig_engs::ml_eng::MLEng;
use crate::{app, cortex};
use std::cell::RefMut;
use std::sync::Arc;

pub type PairBarCfg = (Pair, BarConfig);

// todo: change the name
#[derive(Debug)]
pub struct Brain {
    pub con: Box<Arc<dyn GateWay>>,
    pub cortex: CortexRef,
    pub db: Vec<PairSigHolder>,
    pub set: bool,
}

impl Brain {
    pub fn new(backend: Arc<impl GateWay + 'static>, pair_conf: PairBarCfg) -> Self {
        let mut brain = Self {
            con: Box::new(backend),
            cortex: cortex::new_cortex_ref(),
            db: vec![],
            set: false,
        };
        brain
    }

    // NOT USED NOW - Called from Simulation or Bot codes when connected
    pub fn on_connect(&self) {
        println!("on_connect Brain2");
    }

    fn get_cortex_mut(&self) -> RefMut<Cortex> {
        self.cortex.as_ref().borrow_mut()
    }

    fn init_pair(&mut self, pair: &Pair) {
        if !self.db.iter().any(|ps| &ps.pair == pair) {
            // println!("time: {}", app::clock::get_clock_time_ms());
            self.db.push(PairSigHolder {
                pair: pair.clone(),
                ml_eng: MLEng::new(pair, self.cortex.clone()),
            })
        }
    }

    pub fn on_price_tick(&mut self, pair: &Pair, tick: BTickData) {
        app::clock::set_clock_time(tick.timestamp);
        self.init_pair(pair);
        if !self.set {
            self.init_pair(pair);
        }
        self.set = true;

        let mut cortex = self.get_cortex_mut();
        cortex.on_price_tick(pair, tick.clone());
        // return;
        drop(cortex);

        for ps in self.db.iter_mut() {
            if &ps.pair == pair {
                ps.ml_eng.add_tick(&tick);
            }
        }
        // println!("time: {}", app::clock::get_clock_time_ms());

        // To Gateway: New Positions and update them from Cortex
        let mut cortex = self.get_cortex_mut();
        cortex.run_on_tick_end(); // Call Cortex sim before clean up in here
        for np in cortex.new_positions.iter() {
            // cortex.sim_virtual.open_position(&np, "sky_1");
            self.con.open_position_req_new(&np);
        }
        for up in &cortex.update_positions {
            self.con.update_position(up);
        }
        cortex.new_positions.clear();
        cortex.update_positions.clear();
    }

    pub fn on_notify_position(&self, pos: EventPosition) {
        let mut cortex = self.get_cortex_mut();
        cortex.on_notify_position(pos);
    }
}

#[derive(Debug)]
pub struct PairSigHolder {
    pub pair: Pair,
    pub ml_eng: MLEng,
}

use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::*;
use crate::helper;
use crate::sky_eng::SFrame;
// use crate::sky_eng::SFrame;

impl Brain {
    // todo fix panic
    pub fn read_pair_meta(&self, si: i64) -> &PairMemory {
        self.db.get(&si).unwrap()
    }

    pub fn borrow_pair_meta(&mut self, si: i64) -> &mut PairMemory {
        let pm = self.db.get_mut(&si);
        if pm.is_none() {
            let pair = Pair::id_to_symbol(si);
            let mut pm = PairMemory::new(pair);
            self.db.insert(si, pm);
        }
        self.db.get_mut(&si).unwrap()
    }

    pub fn on_price_tick(&mut self, pair: &Pair, tick: BTickData) {
        self.sim_virtual.run_next_tick(tick.clone());

        let symbol_id = pair.to_symbol_id();
        let mut pari_mem = self.borrow_pair_meta(symbol_id);
        pari_mem.last_tick = Some(tick.clone());
        let frame_opt = pari_mem
            .sky_eng_dep
            .add_tick(&tick, &mut pari_mem.dep_signals_db);
        pari_mem.ml_eng.add_tick(&tick);
        self.update_all_tailing_pos();

        match frame_opt {
            None => {}
            Some(act) => {
                let f = &act;
                let kline_id = f.small_kid;

                if act.long {
                    if self.already_acted(symbol_id, kline_id) {
                        return;
                    }

                    let np = NewPosReq {
                        pair: pair.clone(),
                        is_short: false,
                        base_asset_size: 10_000.0,
                        exit_high_price: pair.cal_price(tick.bid_price, act.profit),
                        exit_low_price: pair.cal_price(tick.bid_price, act.loss),
                        virtual_id: self.sim_virtual.next_virtual_id(), // todo
                        is_virtual: false,                              // todo tailing
                        signal_key: "sky_1".to_string(),
                        at_price: tick.ask_price,
                        time_sec: tick.timestamp_sec as u64,
                        frame: SFrame::default(),
                    };

                    // println!("Open long {:#?}", np);
                    self.sim_virtual.open_position(&np, "sky_1");
                    if !np.is_virtual {
                        self.con.open_position_req_new(&np);
                    }
                }
            }
        }
    }
}

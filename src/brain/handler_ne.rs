use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::*;
use crate::helper;
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

    // new engine
    pub fn on_price_tick(&mut self, pair: &Pair, tick: BTickData) {
        let symbol_id = pair.to_symbol_id();
        let mut pari_mem = self.borrow_pair_meta(symbol_id);
        pari_mem.last_tick = Some(tick.clone());
        let frame_opt = pari_mem.sky_eng.add_tick(&tick);
        self.update_all_tailing_pos();

        match frame_opt {
            None => {}
            Some(frame) => {
                // let dcs = &frame.dcs;
                let f = &frame;
                let kline_id = f.fid;
                let sp = f.big_dc_hl_pip / 1.5;
                let sp = 6.;

                if f.buy1 {
                    // if f.sell1 {
                    // if dcs.sell2 {
                    let np = NewPos {
                        pair: pair.clone(),
                        is_short: false,
                        base_asset_size: 10_000.0,
                        // exit_high_price: pair.cal_price(tick.bid_price, 12.5),
                        exit_high_price: pair.cal_price(tick.bid_price, sp),
                        // exit_low_price: pair.cal_price(tick.bid_price, -12.5),
                        exit_low_price: pair.cal_price(tick.bid_price, -sp / 2.),
                        at_price: tick.ask_price,
                        time_sec: tick.timestamp_sec as u64,
                        frame: frame.clone(),
                    };

                    if self.already_acted(symbol_id, kline_id) {
                        return;
                    }

                    // println!("Open long {:#?}", np);
                    self.con.open_position_req_new(&np);
                }

                if f.sell1 {}
            }
        }
    }
}

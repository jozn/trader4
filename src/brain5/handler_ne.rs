use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::*;
use crate::{candle, helper};

impl Brain5 {
    // todo fix panic
    pub fn read_pair_meta(&self, si: i64) -> &PairMemory {
        self.db.get(&si).unwrap()
    }

    pub fn borrow_pair_meta(&mut self, si: i64) -> &mut PairMemory {
        let pm = self.db.get_mut(&si);
        if pm.is_none() {
            let pair = Pair::id_to_symbol(si);
            let mut pm = PairMemory::new(pair, &CandleConfig::default());
            self.db.insert(si, pm);
        }
        self.db.get_mut(&si).unwrap()
    }

    // new engine
    pub fn on_price_tick_ne_dc_v4(&mut self, pair: &Pair, tick: Tick) {
        let symbol_id = pair.to_symbol_id();
        let mut pari_mem = self.borrow_pair_meta(symbol_id);
        pari_mem.last_tick = Some(tick.clone());
        let frame_opt = pari_mem.ne4.add_tick(&tick);
        self.update_all_tailing_pos();

        match frame_opt {
            None => {}
            Some(frame) => {
                let dcs = &frame.dcs;
                let f = &frame;
                let kline_id = f.fid;

                if dcs.buy2 {
                    // if dcs.sell2 {
                    let np = NewPos {
                        pair: pair.clone(),
                        is_short: false,
                        base_asset_size: 10_000.0,
                        exit_high_price: pair.cal_price(tick.bid_price, 7.5),
                        exit_low_price: pair.cal_price(tick.bid_price, -7.5),
                        at_price: tick.ask_price,
                        time_sec: tick.time_s,
                        frame: frame.clone(),
                    };

                    if self.already_acted(symbol_id, kline_id) {
                        return;
                    }

                    // println!("Open long {:#?}", np);
                    self.con.open_position_req_new(&np);
                }

                if dcs.sell2 {}
            }
        }
    }
}

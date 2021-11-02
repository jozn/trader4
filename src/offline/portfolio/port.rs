use super::*;
use crate::candle::Tick;
use crate::offline::report::Report;
use chrono::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Portfolio {
    pub pos_id: u64,
    pub free_usd: f64, // todo xlot
    pub opens: Vec<Position>,
    pub closed: Vec<Position>,
    pub report: Report,
}

impl Portfolio {
    pub fn new(balance: f64) -> Self {
        Self {
            pos_id: 0,
            free_usd: balance,
            ..Default::default()
        }
    }

    pub fn buy_long(&mut self, param: &PosParam) {
        let usd_vol = param.get_usd();
        if !self.has_enough_balance(usd_vol) {
            return;
        }
        println!("buy long long");

        let mut pos = Position::new_long(param);

        self.report
            .on_new_trade(&pos, self.get_total_balance(param.price));

        // self.free_usd -= usd as f64 * 1000.;
        self.free_usd -= param.get_usd();

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    pub fn sell_long(&mut self, param: &PosParam) {
        let pos = self.opens.iter().find(|p| p.pos_id == param.pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                p.close_pos(param.price, param.time);

                self.report
                    .on_close_trade(&p, self.get_total_balance(param.price));

                let got_usd = p.final_balance;
                self.free_usd += got_usd;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    pub fn sell_short(&mut self, param: &PosParam) {
        let usd_vol = param.get_usd();
        if !self.has_enough_balance(usd_vol) {
            return;
        }

        let mut pos = Position::new_short(param);

        self.report
            .on_new_trade(&pos, self.get_total_balance(param.price));

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    pub fn buy_short(&mut self, param: &PosParam) {
        let pos = self.opens.iter().find(|p| p.pos_id == param.pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                p.close_pos(param.price, param.time);

                self.report
                    .on_close_trade(&p, self.get_total_balance(param.price));

                let got_coin = p.final_balance;
                self.free_usd += p.profit;

                // self.free_asset += got_coin;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    // Close
    pub fn try_close_satasfied_postions(&mut self, param: &PosParam) -> bool {
        let mut done = false;
        for p in self.opens.clone().iter_mut() {
            let mut param2 = param.clone();
            param2.pos_id = p.pos_id;

            p.update_ailing(param.price);

            if p.should_close_bk_simple(param.price) {
                match p.direction {
                    PosDir::Long => {
                        done = true;
                        self.sell_long(&param2);
                    }
                    PosDir::Short => {
                        done = true;
                        self.buy_short(&param2);
                    }
                }
            }
        }
        done
    }

    // Close
    pub fn close_all_positions(&mut self, param: &PosParam) {
        for p in self.opens.clone().iter() {
            let mut param2 = param.clone();
            param2.pos_id = p.pos_id;
            match p.direction {
                PosDir::Long => {
                    self.sell_long(&param2);
                }
                PosDir::Short => {
                    self.buy_short(&param2);
                }
            }
        }
    }

    // Utils
    fn has_enough_balance(&self, usd_vol: f64) -> bool {
        let b = self.get_free_balance();
        let res = if b > usd_vol { true } else { false };
        res
    }

    pub fn get_total_balance(&self, price: XPrice) -> f64 {
        let mut ob = 0.0;
        for p in self.opens.iter() {
            // ob += p.pos_size_xlot * price;
            if p.direction == PosDir::Long {
                ob += p.pos_size_usd;
            }
        }

        self.free_usd + ob
    }

    fn get_free_balance(&self) -> f64 {
        let mut short_debt = 0.0;
        for (i, p) in self.opens.iter().enumerate() {
            if p.direction == PosDir::Short {
                short_debt += p.pos_size_usd;
            }
        }
        self.free_usd - short_debt
    }

    pub fn update_pos(&mut self, pos: &mut Position) {
        self._remove_pos(pos.pos_id);
        self.opens.push(pos.clone());
    }

    fn _close_pos(&mut self, pos: &mut Position) {
        self._remove_pos(pos.pos_id);
        self.closed.push(pos.clone());
    }

    fn next_pos_id(&mut self) -> u64 {
        self.pos_id += 1;
        self.pos_id
    }

    // Remove from both open and closed position vector.
    fn _remove_pos(&mut self, pos_id: u64) {
        let mut idx = -1_i32;
        for (i, p) in self.opens.iter().enumerate() {
            if p.pos_id == pos_id {
                idx = i as i32;
            }
        }
        if idx >= 0 {
            self.opens.remove(idx as usize);
        }

        let mut idx = -1_i32;
        for (i, p) in self.closed.iter().enumerate() {
            if p.pos_id == pos_id {
                idx = i as i32;
            }
        }
        if idx >= 0 {
            self.opens.remove(idx as usize);
        }
    }

    pub fn report(&self, t: &Tick) {
        println!("Report of buy - sell");

        let mut val = 0.0;
        let port = &self;
        for p in &port.opens {
            if !p.finished {
                val += p.pos_size_usd;
            }
        }

        let mut winer_num = 0;
        let mut winer = 0.;
        let mut looser_num = 0;
        let mut looser = 0.;
        let mut fees = 0.;
        for p in &port.closed {
            if p.finished {
                if p.profit > 0. {
                    winer_num += 1;
                    winer += p.profit
                }
                if p.profit < 0. {
                    looser_num += 1;
                    looser += p.profit
                }
                fees += p.spread_fees;
                // let l = p.clone().long.unwrap();
                // fees = l.fee_sell_usd + l.buy_fee_coin;
            }
        }
        let last = t;

        let toatl_balnce = val + port.free_usd;
        // println!("{:#?}", port.longs);
        // println!("{:#?}", port);
        // println!(" pos : {:#?} ", port);

        println!("{:} {} {} ", port.free_usd, val * last.price, toatl_balnce);
        println!(" win : {} {} ", winer_num, winer);
        println!(" loose : {} {} ", looser_num, looser);
        println!(" fees : {} ", fees);
    }
}

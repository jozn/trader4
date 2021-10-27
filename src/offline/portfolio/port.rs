use super::*;
use crate::candle::Tick;
use chrono::*;
use serde::{Deserialize, Serialize};

// TODO: Short selling is not ready as we need to have a live dept toatl balance of opened short postions when of account

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Portfolio {
    pub pos_id: u64,
    pub free_usd: f64,       // todo xlot
    pub free_asset_dep: f64, // todo wallet along usd // todo asset
    pub opens: Vec<Position>,
    pub closed: Vec<Position>,
}

impl Portfolio {
    pub fn new(balance: f64) -> Self {
        Self {
            pos_id: 0,
            free_usd: balance,
            ..Default::default()
        }
    }

    // Manual - Not Working balance should be implemented
    pub fn add_pos_incorrect(&mut self, pos: &mut Position) {
        self.pos_id += 1;
        assert_eq!(pos.pos_id, 0);
        pos.pos_id = self.pos_id;

        self.opens.push(pos.clone());
    }

    pub fn buy_long(&mut self, price: XPrice, usd: XLot, time: u64) {
        if !self.has_enough_balance() {
            return;
        }

        let mut pos = Position::new_long(price, usd, time);

        self.free_usd -= usd as f64 * 1000.;

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    pub fn sell_long(&mut self, price: XPrice, pos_id: u64, time: u64) {
        let pos = self.opens.iter().find(|p| p.pos_id == pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                p.close_pos(price, time);

                let got_usd = p.final_balance;
                self.free_usd += got_usd;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    pub fn sell_short(&mut self, price: XPrice, usd_size: XLot, time: u64) {
        // let val = price * coin;
        // if val < 10. {
        //     return;
        // }
        if !self.has_enough_balance() {
            return;
        }

        let mut pos = Position::new_short(price, usd_size, time);

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    pub fn buy_short(&mut self, price: XPrice, pos_id: u64, time: u64) {
        let pos = self.opens.iter().find(|p| p.pos_id == pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                p.close_pos(price, time);

                let got_coin = p.final_balance;
                self.free_usd += p.profit;

                // self.free_asset += got_coin;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    // Close
    pub fn try_close_satasfied_postions(&mut self, price: XPrice, time: u64) {
        for p in self.opens.clone().iter() {
            if p.should_close(price) {
                match p.direction {
                    PosDir::Long => {
                        self.sell_long(price, p.pos_id, time);
                    }
                    PosDir::Short => {
                        self.buy_short(price, p.pos_id, time);
                    }
                }
            }
        }
    }

    // Close
    pub fn close_all_positions(&mut self, price: XPrice, time: u64) {
        for p in self.opens.clone().iter() {
            match p.direction {
                PosDir::Long => {
                    self.sell_long(price, p.pos_id, time);
                }
                PosDir::Short => {
                    self.buy_short(price, p.pos_id, time);
                }
            }
        }
    }

    pub fn try_close_pos_bk(&mut self, price: XPrice, time: u64) {
        for p in self.opens.clone().iter() {
            match p.direction {
                PosDir::Long => {
                    // Profit
                    if p.open_xprice + p.profit_xpip <= price {
                        self.sell_long(price, p.pos_id, time);
                    }
                    // Lose
                    if p.open_xprice - p.to_stop_loss_xpip >= price {
                        self.sell_long(price, p.pos_id, time);
                    }
                }
                PosDir::Short => {
                    // Profit
                    if p.open_xprice - p.profit_xpip >= price {
                        self.buy_short(price, p.pos_id, time);
                    }
                    // Lose
                    if p.open_xprice + p.to_stop_loss_xpip <= price {
                        self.buy_short(price, p.pos_id, time);
                    }
                }
            }
        }
    }

    // Utils

    fn has_enough_balance(&self) -> bool {
        let b = self.get_free_balance();
        if b > 0.1 * self.free_usd {
            true
        } else {
            false
        }
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
            short_debt += p.pos_size_usd;
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
                // val += p
                // val += p.long.clone().unwrap().got_coin;
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

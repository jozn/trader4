use serde::{Deserialize, Serialize};

// TODO: Short selling is not ready as we need to have a live dept toatl balance of opened short postions when of account

// Note: we use signed numbers for easier cal.

pub type XPrice = i64; // 10^5 of price
pub type XPip = i64; // 10 of one pip == 1/100_000
pub type XLot = i64; // 100 of one lot == 1000$
pub type XSpread = i64; // xlot

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Portfolio {
    pub pos_id: u64,
    pub free_usd: f64,       // todo xlot
    pub free_asset_dep: f64, // todo wallet along usd // todo asset
    pub opens: Vec<Position>,
    pub closed: Vec<Position>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos_id: u64,
    pub direction: PosDir,
    pub pos_size_usd: f64,
    pub pos_size_xlot: XLot,
    pub open_xprice: XPrice,
    pub open_time: u64,
    pub to_exit_xpip: XPip,
    pub to_stop_loss_xpip: XPip,
    pub spread: XSpread,
    pub close_xprice: XPrice,
    pub close_time: u64,
    pub finished: bool, // tod: status
    pub profit_xpip: XPip,
    pub profit: f64,
    pub spread_fees: f64,
    pub final_balance: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PosDir {
    Long,
    Short,
}

impl Default for PosDir {
    fn default() -> Self {
        PosDir::Long
    }
}

impl Portfolio {
    pub fn new() -> Self {
        Self::default()
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

                // self.free_asset += got_coin;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    // Close
    pub fn try_close_pos(&mut self, price: XPrice, time: u64) {
        for p in self.opens.clone().iter() {
            if p.should_close(price) {
                self.sell_long(price, p.pos_id, time);
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

    fn get_total_balance(&self) {}

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
}

impl Position {
    pub fn new_long(open_price: XPrice, pos_size: XLot, time: u64) -> Self {
        assert!(pos_size > 0);

        Self {
            pos_id: 0,
            direction: PosDir::Long,
            pos_size_usd: (pos_size * 1_000) as f64,
            pos_size_xlot: pos_size,
            open_xprice: open_price,
            open_time: time,
            to_exit_xpip: 80,
            to_stop_loss_xpip: 80,
            spread: 8,
            close_xprice: 0,
            close_time: 0,
            finished: false,
            profit_xpip: 0,
            profit: 0.0,
            final_balance: 0.0,
            ..Default::default()
        }
    }

    pub fn new_short(open_price: XPrice, pos_size: XLot, time: u64) -> Self {
        assert!(pos_size > 0);

        Self {
            pos_id: 0,
            direction: PosDir::Short,
            pos_size_usd: (pos_size * 1_000) as f64,
            pos_size_xlot: pos_size,
            open_xprice: open_price,
            open_time: time,
            to_exit_xpip: 50,
            to_stop_loss_xpip: 35,
            spread: 8,
            close_xprice: 0,
            close_time: 0,
            finished: false,
            profit_xpip: 0,
            profit: 0.0,
            final_balance: 0.0,
            ..Default::default()
        }
    }

    pub fn close_pos(&mut self, close_price: XPrice, time: u64) {
        match self.direction {
            PosDir::Long => self.close_long(close_price, time),
            PosDir::Short => {}
        };
    }

    fn close_long(&mut self, close_price: XPrice, time: u64) {
        let pl_xpip = close_price - self.open_xprice;
        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = close_price;
        self.close_time = time;
        self.finished = true;

        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / 100_000.);
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.spread_fees = self.pos_size_usd * (self.spread as f64 / 100_000.);
        self.final_balance = self.pos_size_usd + pure_pl;
    }

    fn should_close(&self, close_price: XPrice) -> bool {
        let mut trig = false;
        let pl_xpip = match self.direction {
            PosDir::Long => {
                let pl = close_price - self.open_xprice;
                if pl > 0 && pl >= self.to_exit_xpip {
                    trig = true
                }

                if pl < 0 && pl.abs() >= self.to_stop_loss_xpip {
                    trig = true
                }
            }
            PosDir::Short => {
                // todo cal
            }
        };
        trig
    }

    // bk
    pub fn close_pos_bk(&mut self, close_price: XPrice, time: u64) {
        // let pl_xpip = match self.direction {
        //     PosDir::Long => self.open_xprice - close_price,
        //     PosDir::Short => close_price - self.open_xprice,
        // };

        let pl_xpip = match self.direction {
            PosDir::Long => close_price - self.open_xprice,
            PosDir::Short => self.open_xprice - close_price,
        };

        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = close_price;
        self.close_time = time;
        self.finished = true;

        // let pure_pl = (self.pos_size_xlot * pure_pl_xpip) as f64 / 100_000.; // todo fix cal
        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / 100_000.); // todo fix cal
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.final_balance = self.pos_size_usd + pure_pl;
    }
}

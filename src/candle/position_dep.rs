use super::*;
use PosDir::{Long, Short};

const FEE_RATE: f64 = 0.001;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Portfolio_Dep {
    pub pos_id: u64,
    pub free_usd: f64,
    pub free_coin: f64, // todo wallet along usd
    pub opens: Vec<Position_Dep>,
    pub closed: Vec<Position_Dep>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position_Dep {
    pub pos_id: u64,
    pub direction: PosDir,
    pub long: Option<PositionLong>,
    pub short: Option<PositionShort>,
    pub open_price: f64,
    pub open_time: u64,
    pub to_exit_per: f64,
    pub to_stop_loss_per: f64,
    pub close_price: f64,
    pub close_time: u64,
    pub finished: bool, // tod: status
    pub profit: f64,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionLong {
    pub buy_val: f64, // ex: usd
    pub got_coin: f64,
    pub buy_fee_coin: f64,
    pub sell_got_usd: f64,
    pub fee_sell_usd: f64,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionShort {
    pub sold_coin: f64,    // ex: btc counts
    pub got_usd: f64,      // sold_coin - sell_fee
    pub sell_fee_usd: f64, // in other pari todo add an enum with value of security of it
    pub bought_coin: f64,
    pub buy_fee_coin: f64,
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

impl Portfolio_Dep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pos(&mut self, pos: &mut Position_Dep) {
        self.pos_id += 1;
        assert_eq!(pos.pos_id, 0);
        pos.pos_id = self.pos_id;

        self.opens.push(pos.clone());
    }

    pub fn buy_long(&mut self, price: f64, usd: f64, time: u64) {
        if price < 10. {
            return;
        }

        let mut pos = Position_Dep::new_long(price, usd, time);

        self.free_usd -= usd;

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    pub fn sell_long(&mut self, price: f64, pos_id: u64, time: u64) {
        let pos = self.opens.iter().find(|p| p.pos_id == pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                p.sell_long(price, time);

                let got_usd = p.long.clone().unwrap().sell_got_usd;
                self.free_usd += got_usd;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    pub fn sell_short(&mut self, price: f64, coin: f64, time: u64) {
        let val = price * coin;
        if val < 10. {
            return;
        }

        let mut pos = Position_Dep::new_short(price, coin, time);

        self.free_coin -= coin;

        pos.pos_id = self.next_pos_id();
        self.opens.push(pos);
    }

    pub fn buy_short(&mut self, price: f64, pos_id: u64, time: u64) {
        let pos = self.opens.iter().find(|p| p.pos_id == pos_id);
        match pos {
            None => {}
            Some(p) => {
                let mut p = p.clone();
                p.buy_short(price, time);

                let got_coin = p.short.clone().unwrap().bought_coin;

                self.free_coin += got_coin;

                self._remove_pos(p.pos_id);
                self.closed.push(p);
            }
        }
    }

    pub fn close_pos(&mut self, price: f64, pos_id: u64, time: u64) {
        let pos = self.opens.iter().find(|p| p.pos_id == pos_id);
        match pos {
            None => {}
            Some(p) => match p.direction {
                Long => {
                    self.sell_long(price, pos_id, time);
                }
                Short => {
                    self.buy_short(price, pos_id, time);
                }
            },
        }
    }

    pub fn update_pos(&mut self, pos: &mut Position_Dep) {
        self._remove_pos(pos.pos_id);
        self.opens.push(pos.clone());
    }

    fn _close_pos(&mut self, pos: &mut Position_Dep) {
        self._remove_pos(pos.pos_id);
        self.closed.push(pos.clone());
    }

    fn get_total_balance(&self) {}

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

impl Position_Dep {
    pub fn new_long(price: f64, usd: f64, time: u64) -> Self {
        assert!(usd > 10.);

        let q = usd / price;
        let fee = q * FEE_RATE;
        let got_coin = q - fee;

        Self {
            pos_id: 0,
            direction: PosDir::Long,
            long: Some(PositionLong {
                buy_val: usd,
                got_coin: got_coin,
                buy_fee_coin: fee,
                sell_got_usd: 0.0,
                fee_sell_usd: 0.0,
            }),
            short: None,
            open_price: price,
            open_time: time,
            to_exit_per: 1.5,
            to_stop_loss_per: 1.10,
            close_price: 0.0,
            close_time: 0,
            finished: false,
            profit: 0.,
        }
    }

    pub fn new_short(price: f64, coin: f64, time: u64) -> Self {
        let q = coin * price;
        assert!(q > 10.);

        let fee = coin * FEE_RATE;
        let total_enter = (coin - fee) * price;

        Self {
            pos_id: 0,
            direction: PosDir::Short,
            long: None,
            short: Some(PositionShort {
                sold_coin: coin,
                got_usd: total_enter,
                sell_fee_usd: fee,
                bought_coin: 0.0,
                buy_fee_coin: 0.0,
            }),
            open_price: price,
            open_time: time,
            to_exit_per: -1.5,
            to_stop_loss_per: -1.0,
            close_price: 0.0,
            close_time: 0,
            finished: false,
            profit: 0.0,
        }
    }

    pub fn sell_long(&mut self, price: f64, time: u64) {
        assert!(self.direction == Long);
        assert!(self.long.is_some());

        let profit = match &mut self.long {
            None => 0.0,
            Some(l) => {
                let q = price * l.got_coin;
                let fee = q * FEE_RATE;
                let got_usd = q - fee;

                l.sell_got_usd = got_usd;
                l.fee_sell_usd = fee;

                let profit = l.buy_val - got_usd;
                profit
            }
        };

        self.close_price = price;
        self.close_time = time / 1000;
        self.profit = profit;
        self.finished = true;
    }

    pub fn buy_short(&mut self, price: f64, time: u64) {
        assert!(self.direction == Short);
        assert!(self.short.is_some());

        // let mut ts_cost
        let profit = match &mut self.short {
            None => {}
            Some(s) => {
                let q = price * s.sold_coin;
                let fee = q * FEE_RATE;
                let got_coin = q - fee;

                s.bought_coin = got_coin;
                s.buy_fee_coin = fee;

                // cal profit
                let profit =
                    (self.open_price - price) * s.sold_coin - (s.sell_fee_usd + fee * price);
                self.profit = profit;
            }
        };
        self.close_price = price;
        self.close_time = time / 1000;
        self.finished = true;
    }
}

use crate::base::SimpleCrossEvent;
use crate::candle::{CandleSeriesTA, Tick};
use crate::loader::CSVForexRecord;
use crate::loader::*;
use crate::offline::strategy1::Strategy1;
use crate::offline::to_csv_out;
use crate::portfolio::*;
use crate::run::*;
use crate::*;

#[derive(Debug, Default, Clone)]
pub struct SimMacdWorld {
    ticks: Vec<CSVForexRecord>,
    last_tick: Tick,
    // balance: Vec<f64>,
    // pub port: Portfolio,
    strategy1: Strategy1,
    it_num: usize,
    pos_id: usize,
    tick_cnt: usize,
}

impl SimMacdWorld {
    pub fn new() -> Self {
        Self {
            ticks: loader::_load(14000_000, "/media/hamid/K/forex1/EURUSD_tab3.csv"),
            it_num: 0,
            strategy1: Strategy1::new(),
            // port: Portfolio::new(100_000.),
            ..Default::default()
        }
    }

    pub fn new_runner() -> WorldRunner {
        let hull_world = SimMacdWorld::new();
        let mut wr = WorldRunner::new(hull_world);
        wr
    }
}

impl TRunner for SimMacdWorld {
    fn get_next_tick(&mut self) -> Option<Tick> {
        let idx = self.it_num;
        self.it_num += 1;

        if idx < self.ticks.len() {
            let forex_csv_rec = self.ticks.get(idx).unwrap();
            let mut t = forex_csv_rec.to_tick();
            t.price = t.price * 100_000.;
            self.last_tick = t.clone();
            Some(t)
        } else {
            None
        }
    }

    fn on_next_tick_bulk(&mut self, cst: &CandleSeriesTA) {
        self.tick_cnt += 1;
        let t = &self.last_tick;

        // Close
        // self.port.try_close_satasfied_postions(t.price as i64, t.time_s);
        self.strategy1.try_close_satasfied_postions(t);

        let price = t.price;
        // let kt = &cst.medium.kline_ta_tip.clone().unwrap();
        let kt_opt = &cst.medium.klines_ta.last();
        if kt_opt.is_none() {
            return;
        }
        let kt = kt_opt.unwrap();
        let kid = kt.kline.bucket;
        let ma = kt.ta1.sma50;
        let macd_out = kt.ta1.macd.clone();

        let up = macd_out.signal.0;
        let down = macd_out.signal.1;

        match up {
            SimpleCrossEvent::Bull(_) => {
                if macd_out.macd < 0. && price > ma {
                    self.strategy1.buy(kid, t);
                    println!("long {} - {} - {:#?}", price, kt.kline.bucket, &macd_out);
                    // self.port.buy_long(t.price as i64, 10, t.time_s);
                }
            }
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {}
        }

        match down {
            SimpleCrossEvent::Bull(_) => {
                // self.port.buy_long(t.price as i64, 1, t.time_s);
            }
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {
                if macd_out.macd > 0. && price < ma {
                    self.strategy1.sell(kid, t);
                    // self.port.sell_short(t.price as i64, 10, t.time_s);
                }
            }
        }
    }

    fn on_price_tick(&mut self, cst: &CandleSeriesTA, tikc: &Tick) {
        // let t = tikc;
        // let b = self.port.get_total_balance(tikc.price as i64);
        if self.tick_cnt % 2000 == 0 {
            // self.balance.push(b);
            self.strategy1.collect_balance(tikc);
        }
        self.strategy1.try_close_satasfied_postions(tikc);
    }

    fn on_exit(&mut self) {
        // self.port.close_all_positions()
        println!("on exit - all pos {}", self.pos_id);
        self.ticks.clear(); // for debug - clear array
                            // println!("{:#?}", self);
                            // self.port
                            //     .close_all_positions(self.last_tick.price as i64, self.last_tick.time_s);
        self.strategy1.close_all_exit(&self.last_tick);
        // self.port.report(&self.last_tick);
        // println!("on exit");

        // self.balance
        //     .push(self.port.get_total_balance(self.last_tick.price as i64));

        self.strategy1.collect_balance(&self.last_tick);

        // println!("balance {:#?}", self.balance);

        // balance csv
        // let o: Vec<f64> = self.port.closed.iter().map(|p| p.final_balance).collect();
        // let os = to_csv_out(&o);
        // println!("{}", os);

        self.strategy1.report();

        println!("=====================================================");

        // println!("{:#?}", self.port);
    }
}

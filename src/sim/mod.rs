pub mod proc;
pub mod trend;

use super::candle::*;
use super::portfolio::*;
use super::*;
use crate::forex::CSVForexRecord;

#[derive(Debug, Default, Clone)]
pub struct Runner {
    pub tick_id: u64,
    pub port: Portfolio,
    pub candles: CandleSeriesTA,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            tick_id: 0,
            port: Portfolio {
                free_usd: 100_000.0,
                free_asset_dep: 0.1,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        // let trades = loader_trade::load_trades_from_bin();
        let trades = forex::_load(1000_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");
        let mut cst = CandleSeriesTA::new();

        let mut mt = MiniTick::new(0.0);

        let mut trade_cnt = 0;
        let mut tick_id = 0;

        for t in trades.iter() {
            let mut tick = t.to_tick();
            tick.price = tick.price * 100_000.;

            let should = mt.add(tick.clone());
            if should {
                println!("Ticked: >>> {:#?} - {}", tick_id, trade_cnt);
                tick_id += 1;
                let old = mt.ticks_arr;
                mt = MiniTick::new(tick.price);
                cst.add_ticks(old);

                // LOGIC MUST BE ADDED HERE
                self.port.try_close_pos(tick.price as i64, tick.time_s);
                proc::proc_tick_buy(&cst, &mut self.port, &tick);
                // END OF LOGIC
            }

            if trade_cnt % 1000 == 0 {
                // println!("Ticked: >>> {:#?}", trade_cnt);
            }
            trade_cnt += 1;

            // Tick buy / sell
            // proc::proc_tick_sell(&cst, &mut self.port);
            if trade_cnt % 1000 == 0 {
                // println!("buy *********************** ");
                // proc::proc_tick_buy_random(&cst, &mut self.port)
            }
        }
        cst.add_ticks(mt.ticks_arr);
        // println!("{:#?}", cst.medium);
        println!("ticks: >>> {:#?}", tick_id);
        println!("candle serires: >>> {:#?}", trade_cnt);
        self.candles = cst;
        self.report();
    }

    fn report(&self) {
        println!("Report of buy - sell");

        let mut val = 0.0;
        let port = &self.port;
        for p in &self.port.opens {
            if !p.finished {
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
        let last = self.candles.klines.medium.klines.last().unwrap();

        let toatl_balnce = val * last.close + port.free_usd;
        // println!("{:#?}", port.longs);
        // println!("{:#?}", port);
        println!(" pos : {:#?} ", self.port);

        println!("{:} {} {} ", port.free_usd, val * last.close, toatl_balnce);
        println!(" win : {} {} ", winer_num, winer);
        println!(" loose : {} {} ", looser_num, looser);
        println!(" fees : {} ", fees);
        println!(" candles medium : {} ", self.candles.medium.klines_ta.len());
    }
}

#[derive(Debug, Default, Clone)]
pub struct MiniTick {
    active: bool,
    tick_start_milli: u64,
    last_tick_price: f64,
    high_price: f64,
    low_price: f64,
    ticks_arr: TimeSerVec<Tick>,
}

impl MiniTick {
    fn new(last_price: f64) -> Self {
        Self {
            active: true,
            last_tick_price: last_price,
            low_price: f64::MAX,
            ..Default::default()
        }
    }

    fn add(&mut self, forex_tick: Tick) -> bool {
        assert!(self.active);
        if self.tick_start_milli == 0 {
            self.tick_start_milli = forex_tick.time_s;
        }

        let tdiff = forex_tick.time_s - self.tick_start_milli;

        self.high_price = self.high_price.max(forex_tick.price);
        self.low_price = self.low_price.min(forex_tick.price);

        let mut should_tick = false;

        self.ticks_arr.push(forex_tick.clone());

        if self.ticks_arr.len() >= 10 {
            should_tick = true;
        } else if tdiff < 1000 {
            // todo remove or change this as we are working with seconds
            // at least one second for each tick
            should_tick = false;
        } else if tdiff > 300_000 {
            // at least 5 min run
            should_tick = true;
        } else {
            let pdiff = (self.last_tick_price - forex_tick.price).abs();
            // if (pdiff / self.low_price) > 0.00002 {
            if pdiff >= 0.5 {
                // 0.2 pip
                should_tick = true;
            }
        };
        if should_tick {
            self.active = false;
        }
        should_tick
        // true
    }
}

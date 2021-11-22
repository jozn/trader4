use crate::candle::{CandleSeriesTA, Tick, TimeSerVec};
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub trait TRunner: Debug {
    fn get_next_tick(&mut self) -> Option<Tick>; // should blocks and returns next tick, None means end of data, exist.
    fn on_next_tick_bulk(&mut self, cst: &CandleSeriesTA); // Called in each MiniTick full,
    fn on_price_tick(&mut self, cst: &CandleSeriesTA, tikc: &Tick); // Called after each price tick
    fn on_exit(&mut self, cst: &CandleSeriesTA);
}

#[derive(Debug)]
pub struct WorldRunner {
    pub run_id: u64,
    pub candles: CandleSeriesTA,
    world: Box<dyn TRunner>,
}

impl WorldRunner {
    pub fn new(w: impl TRunner + 'static) -> Self {
        Self {
            run_id: 0,
            candles: Default::default(),
            world: Box::new(w),
        }
    }

    pub fn run(&mut self) {
        let mut mt = MiniTick::new(0.0);

        loop {
            let mut world = self.world.deref_mut();
            let tick_opt = world.get_next_tick();

            let mut is_exist = false;
            let should_run = match tick_opt.clone() {
                None => {
                    // last run
                    is_exist = true;
                    true
                }
                Some(tick) => mt.add(tick),
            };

            if should_run {
                self.run_id += 1;

                let mut last_price = 0.0;
                let bulk_new_ticks = mt.ticks_arr;
                if bulk_new_ticks.len() > 0 {
                    last_price = bulk_new_ticks.last().unwrap().price;
                    self.candles.add_ticks(bulk_new_ticks);
                    // Run the wold tick
                    world.on_next_tick_bulk(&self.candles)
                }
                mt = MiniTick::new(last_price);
            }

            match tick_opt {
                None => {}
                Some(t) => {
                    world.on_price_tick(&self.candles, &t);
                }
            }

            if is_exist {
                // println!("Report of buy - sell");
                world.on_exit(&self.candles);
                break;
            }
        }
    }

    fn report(&self) {}
}

#[derive(Debug, Default, Clone)]
pub struct MiniTick {
    active: bool,
    tick_start_milli: u64,
    last_tick_price: f64,
    high_price: f64,
    low_price: f64,
    pub ticks_arr: TimeSerVec<Tick>,
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

    pub fn add(&mut self, forex_tick: Tick) -> bool {
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

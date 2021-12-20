pub mod back;
pub mod postion;
pub mod shared;

pub mod kline_ta_csv;
pub mod report;
pub mod run;

pub use back::*;
pub use postion::*;
pub use shared::*;

fn round5(v: f64) -> f64 {
    let pre = 100_000.;
    ((v * pre) as i64) as f64 / pre
}

fn num5i64(v: f64) -> i64 {
    num5(v) as i64
}

pub fn num5(v: f64) -> f64 {
    let n = v.abs() as u64;
    let dig = if n >= 10_000 {
        0
    } else if n >= 1000 {
        1
    } else if n >= 100 {
        2
    } else if n >= 10 {
        3
    } else if n >= 1 {
        4
    } else {
        5
    };

    let dig_per = 10_f64.powf(dig as f64);

    ((v * dig_per) as i64) as f64 / dig_per
}

// todo: we are currently do not account spreads: asks and bids prices > impl this

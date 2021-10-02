pub mod kline_ta_csv;
pub mod model;
pub mod tok_kline_ta_csv;

pub use kline_ta_csv::*;
pub use model::*;
pub use tok_kline_ta_csv::*;

// Offline package used for anylsing data for data scince in Excel,...

fn round5(v: f64) -> f64 {
    let pre = 100_000.;
    ((v * pre) as i64) as f64 / pre
}

fn num5i64(v: f64) -> i64 {
    num5(v) as i64
}

fn num5(v: f64) -> f64 {
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

pub mod brain4;
pub mod handler_ne;
pub mod tailing;

pub use brain4::*;
// pub use dc_strategy::*;
// pub use handler::*;
pub use tailing::*;

// Brain2 is just a playground for algoritms development afetr success merge this
//  with brain1 > Brain2 is single asset and reduceid codes for simpliciyt

// todo move to core helper
pub fn rond5(num: f64) -> f64 {
    ((num * 100_000.0) as u64) as f64 / 100_000.0
}

pub fn cal_price(price: f64, pip: f64) -> f64 {
    let p = 1. + (pip / 10_000.);
    rond5(price * p)
}

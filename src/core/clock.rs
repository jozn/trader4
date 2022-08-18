use std::sync::atomic::AtomicI64;
use std::sync::Mutex;
use std::{env, io};

use once_cell::sync::OnceCell;

#[derive(Debug)]
struct Clock {
    last_time_ms: AtomicI64,
    mtx_last_time: Mutex<i64>,
}
static INSTANCE: OnceCell<Clock> = OnceCell::new();

pub fn set_clock_time(time_ms: i64) {
    let mut s = INSTANCE.get_or_init(|| Clock {
        last_time_ms: AtomicI64::new(time_ms),
        mtx_last_time: Mutex::new(time_ms),
    });
    let mut t = s.mtx_last_time.lock().unwrap();
    let mut tf = *t;
    if time_ms > tf {
        *t = time_ms;
        // tf = time_ms;
    }
    // t = time_ms;
}

pub fn get_clock_time() -> i64 {
    let c = INSTANCE.get();
    match c {
        None => 0,
        Some(c) => {
            let mut t = c.mtx_last_time.lock().unwrap();
            let out = *t;
            out
        }
    }
}

// static TIME : AtomicI64

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock() {
        for i in 1..1000 {
            set_clock_time(i);
            let time = get_clock_time();
            assert_eq!(time, i);
            if i % 100 == 0 {
                // println!("{} , {}", i, time);
            }
        }
    }

    #[test]
    fn test_next() {}
}

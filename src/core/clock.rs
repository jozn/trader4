use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicI64;
use std::sync::Mutex;

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
    let mut last_guard = s.mtx_last_time.lock().unwrap();
    let mut last_time = *last_guard;
    if time_ms > last_time {
        *last_guard = time_ms;
    }
}

// Return last ticks clock in ms
pub fn get_clock_time_ms() -> i64 {
    let c = INSTANCE.get();
    match c {
        None => 0,
        Some(clock) => {
            let mut last_guard = clock.mtx_last_time.lock().unwrap();
            let time_ms = *last_guard;
            time_ms
        }
    }
}

pub fn get_clock_time_sec() -> i64 {
    get_clock_time_ms() / 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock() {
        for i in 1..1000 {
            set_clock_time(i);
            let time = get_clock_time_ms();
            assert_eq!(time, i);
            if i % 100 == 0 {
                // println!("{} , {}", i, time);
            }
        }
    }

    #[test]
    fn test_next() {}
}

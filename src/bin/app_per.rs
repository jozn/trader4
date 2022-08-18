use std::borrow::BorrowMut;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use trader4;

// Performance play of app

fn main() {
    clock_performance();
}

fn clock_performance() {
    println!("start of clock");
    for i in 0..100_000_000 {
        trader4::app::clock::set_clock_time(i);
        if i % 5_000_000 == 0 {
            println!("clock {}", i);
        }
        // rc.num +=1;
    }
    println!("end of clock");
}

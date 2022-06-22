use rand::prelude::*;

// A simple sim of return on capital in trading

fn main() {
    for i in 1..100 {
        println!("================{}=================", i);
        sim();
    }
}

fn sim() {
    let mut balance = 1000.;
    let mut rng = rand::thread_rng();
    let mut win = 0;
    let mut loose = 0;
    let mut profit = 0.;
    let mut decline = 0.;
    for i in 1..=20 {
        let r: f64 = rng.gen(); // generates a float between 0 and 1
        let mut sign = "+";
        if r < 0.6 {
            win += 1;
            let ch = balance * 0.4;
            balance += ch;
            profit += ch;
        } else {
            sign = "-";
            loose += 1;
            let ch = balance * 0.2;
            balance -= ch;
            decline += ch;
        }
        println!("{} {} {}", i, sign, balance);
    }
    let ratio = win as f64 / (win + loose) as f64;
    let ret = (ratio * 2. - (1. - ratio) * 1.) / (ratio * 2. + (1. - ratio) * 1.); // retiontion
    let ret2 = (profit - decline) / (profit + decline);
    println!("win/lose {}/{}  {}  ret: {}", win, loose, ratio, ret);
    println!("ret2: {}", ret2);
    println!("+{}  -{}", profit,decline);
}

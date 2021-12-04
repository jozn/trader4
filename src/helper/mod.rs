use chrono::NaiveDateTime;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time_sec() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    // println!("{:?}", since_the_epoch);
    since_the_epoch.as_secs()
}

pub fn get_time_ms() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    // println!("{:?}", since_the_epoch);
    since_the_epoch.as_secs() * 1000
}

pub fn to_csv_out<T: Serialize>(arr: &Vec<T>) -> String {
    let mut str_out = vec![];
    let mut wtr = csv::Writer::from_writer(&mut str_out);

    for v in arr {
        wtr.serialize(v);
    }

    let s = wtr.flush();
    drop(wtr);

    let s = String::from_utf8(str_out).unwrap();

    format!("{:}", s)
}

pub fn to_time_string(time_sec: i64) -> String {
    let open_time = NaiveDateTime::from_timestamp(time_sec, 0);
    let ots = open_time.format("%Y-%m-%d %H:%M:%S").to_string();
    ots
}

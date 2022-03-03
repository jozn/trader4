use chrono::NaiveDateTime;
use rand::Rng;
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

pub fn get_rand(max: u64) -> u64 {
    rand::thread_rng().gen_range(0..max)
}

pub fn to_csv_out<T: Serialize>(arr: &Vec<T>, tab: bool) -> String {
    to_csv_out_v2(arr,tab,true)
    /*let mut str_out = vec![];

    let mut wtr = if tab {
        csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .has_headers(true)
            .from_writer(&mut str_out)
    } else {
        csv::Writer::from_writer(&mut str_out)
    };
    // let mut wtr = csv::Writer::from_writer(&mut str_out);

    for v in arr {
        wtr.serialize(v);
    }

    let s = wtr.flush();
    drop(wtr);

    let s = String::from_utf8(str_out).unwrap();

    format!("{:}", s)*/
}

// like above but with header
pub fn to_csv_out_v2<T: Serialize>(arr: &Vec<T>, tab: bool, header: bool) -> String {
    let mut str_out = vec![];

    let mut wtr = if tab {
        csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .has_headers(header)
            .from_writer(&mut str_out)
    } else {
        csv::WriterBuilder::new()
            // .quote_style(csv::QuoteStyle::NonNumeric)
            .has_headers(header)
            .from_writer(&mut str_out)
        // csv::Writer::from_writer(&mut str_out)
    };
    // let mut wtr = csv::Writer::from_writer(&mut str_out);

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

pub fn time_tag_string() -> String {
    to_time_string(get_time_sec() as i64)
}

pub fn to_json_out<T: Serialize>(arr: &Vec<T>) -> String {
    let o = serde_json::to_string_pretty(arr).unwrap();
    o
}

pub fn to_date(time_s: u64) -> String {
    let open_time = NaiveDateTime::from_timestamp(time_s as i64, 0);
    open_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn to_duration(time_s: i64) -> String {
    let time_s = time_s.abs();
    let seconds = time_s % 60;
    let minutes = (time_s / 60) % 60;
    let hours = (time_s / 3660);
    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}

use chrono::NaiveDateTime;
use rand::Rng;
use regex::Regex;
use serde::Serialize;
use std::io::Read;
use std::path::Path;
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
    // since_the_epoch.as_secs() * 1000
    since_the_epoch.as_millis() as u64
}

pub fn get_rand(max: u64) -> u64 {
    rand::thread_rng().gen_range(0..max)
}

pub fn rond(num: f64, fract: u32) -> f64 {
    let frac = 10_u32.pow(fract) as f64;
    ((num * frac) as i64) as f64 / frac
}

pub fn create_folders_for_file(file_abs_path: &str) {
    let file_name = Path::new(file_abs_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let p = file_abs_path.replace(file_name, "");
    std::fs::create_dir_all(p);
}

pub fn to_csv_out<T: Serialize>(arr: &Vec<T>, tab: bool) -> String {
    to_csv_out_v2(arr, tab, true)
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

// v3 is our workaround the csv problem, currently if we send nested tuple to csv and the
//  header flag be on it cause it to only print the headers for all rows. This function
//  works around this by sending first row with headers on (if set) and the rest of rows
//  with header flag off.
pub fn to_csv_out_v3<T: Serialize + Clone>(arr: &Vec<T>, tab: bool, header: bool) -> String {
    let first_row = arr.get(0).unwrap().clone();
    let first_arr = vec![first_row];
    let first_csv = to_csv_out_v2(&first_arr, tab, header);
    let rest_rows: Vec<T> = arr.iter().skip(1).map(|v| v.clone()).collect();
    let rest_csv = to_csv_out_v2(&rest_rows, tab, false);
    format!("{}{}", first_csv, rest_csv)
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

pub fn remove_json_comments(str: &str) -> String {
    let mut stripped = String::new();
    json_comments::CommentSettings::all()
        .strip_comments(str.as_bytes())
        .read_to_string(&mut stripped);
    stripped
}

//////////// Timer

pub struct RunTimer {
    start: u64,
    end: u64,
}

impl RunTimer {
    pub fn new(tag: &str) -> Self {
        if tag.len() > 0 {
            println!("starting {} ...", tag);
        }
        Self {
            start: get_time_ms(),
            end: 0,
        }
    }

    pub fn end(&mut self) {
        self.end = get_time_ms();
    }

    pub fn end_print(&self, msg: &str) {
        let end_time = get_time_ms();
        let run_time = (end_time - self.start) as f64;
        println!("{} [{} sec]", msg, run_time / 1000.);
    }
}

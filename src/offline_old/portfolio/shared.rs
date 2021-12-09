use chrono::*;

// Note: we use signed numbers for easier cal.

pub type XPrice = i64; // 10^5 of price
pub type XPip = i64; // 10 of one pip == 1/100_000
pub type XLot = i64; // 100 of one lot == 1000$
pub type XSpread = i64; // xlot

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

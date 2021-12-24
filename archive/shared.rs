use chrono::*;

/*// todo move to helper

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
*/
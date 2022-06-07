use serde::{Deserialize, Serialize};
// todo: change all timesatps to millisconds only for internals?

// We start weeks from first week in 2021. First week id 1.
// Week day start from Sunday(1) to Saturday(7)
// Note: We have not yet investigate for dates before 2021: negative numbers and zero
// From history.rs
const YEAR_ZERO_WEEK: i64 = 1609632000_000; // Sunday, 3 January 2021 00:00:00
const MS_IN_WEEK: i64 = 7 * 86400_000;

// dep
pub struct WeekData {
    pub week_id: u16,
    pub start_ms: i64, // In milli_second
    pub end_ms: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DayInfo {
    pub week_id: i32, // Start from 2021
    pub day_id: i32,  // 1-7
    pub start_ms: i64,
    pub end_ms: i64,
}
// todo: remove second funs all should be milli
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WeekInfo {
    pub week_id: i32,
    pub start: i64,
    pub end: i64,
}

pub fn timestamp_to_week(mil_seconds: i64) -> WeekInfo {
    let week_id = (mil_seconds - YEAR_ZERO_WEEK) / (86400_000 * 7) + 1;
    let start = YEAR_ZERO_WEEK + (week_id - 1) * MS_IN_WEEK;
    let end = start + MS_IN_WEEK;

    WeekInfo {
        week_id: week_id as i32,
        start: start,
        end: end,
    }
}

pub fn week_to_week_info(week_id: i32) -> WeekInfo {
    let time = (week_id - 1) as i64 * MS_IN_WEEK + YEAR_ZERO_WEEK;
    timestamp_to_week(time + 1)
}

pub fn timestamp_to_day(mil_sec: i64) -> DayInfo {
    let wi = timestamp_to_week(mil_sec);
    let dat_id = (mil_sec - wi.start) / 86400_000 + 1;
    DayInfo {
        week_id: wi.week_id,
        day_id: dat_id as i32,
        start_ms: wi.start,
        end_ms: wi.end,
    }
}

// From history.rs
fn _get_weeks_num(m_seconds: i64) -> i64 {
    (m_seconds - YEAR_ZERO_WEEK) / (86400_000 * 7) + 1
}

fn _get_weeks_times(weed_id: i64) -> (i64, i64) {
    let start = YEAR_ZERO_WEEK + (weed_id - 1) * MS_IN_WEEK;
    (start, start + MS_IN_WEEK)
}

////////////////////////////////////////////////////////
// todo move all to cortex

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ActionSignalDep {
    pub small_kid: i32,
    pub long: bool,
    pub profit: f64,
    pub loss: f64,
}

// Internal to signals engines
// ps_ : primary_signal
// fs_ : final_signal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalMemDep {
    pub ps_buy: bool,
    pub ps_small_bar_id: i32,
    pub fs_buy: bool,
    pub fs_small_bar_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_timestamps() {
        let wi = timestamp_to_week(1609632000_000);
        let wd = timestamp_to_day(1609632000_000);

        println!("{:#?}", wi);
        println!("{:#?}", wd);

        let d2 = 1709632000_000;
        let wi = timestamp_to_week(d2);
        println!("{:#?}", wi);

        let d3 = 1710028800_000; // end of week above
        let wi = timestamp_to_week(d3);
        println!("{:#?}", wi);
        let wi = timestamp_to_week(d3 - 1);
        println!("{:#?}", wi);
        let wi = timestamp_to_week(d3 + 1);
        println!("{:#?}", wi);

        let wi = week_to_week_info(166);
        println!("{:#?}", wi);
    }
}

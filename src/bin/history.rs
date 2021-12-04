use std::thread;
use trader2;
use trader2::ctrader::Config;
use trader2::online::assets;
use trader2::online::assets::Pair;
use trader2::{helper, online};

fn main() {
    run();
}

const YEAR_ZERO_WEEK: i64 = 1609632000_000; // Sunday, 3 January 2021 00:00:00
const START_WEEK: i64 = 1625356800_000; // 3 Jan 2021
const MS_IN_WEEK: i64 = 7 * 86400_000;

fn run() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "l4jT24BWu3etFSEVViQKu1NsGpBYf2nKN0DyUGgqjy0".to_string(),
        ctid: 22851452,
    };

    let pair_ids = assets::get_all_symbols_ids();
    for pair_id in pair_ids {
        let pair = assets::Pair::id_to_symbol(pair_id);
        let cfg = cfg.clone();
        thread::spawn(move || {
            let mut current_week_start_ms = START_WEEK;
            loop {
                let week_id = get_weeks_num(current_week_start_ms);

                let end_week_time = current_week_start_ms + MS_IN_WEEK;
                let csv_res = trader2::collector::collect_data_from_api_csv(
                    &cfg,
                    &pair,
                    current_week_start_ms,
                    end_week_time,
                );

                let folder = get_folder_path(&pair, week_id);
                let file_path = get_folder_path(&pair, week_id);
                std::fs::create_dir_all(&folder);
                std::fs::write(&file_path, &csv_res);

                println!(
                    "{:?} > Completed Week {} - Size: {} -  Path: {} ",
                    pair,
                    week_id,
                    csv_res.len(),
                    file_path
                );

                current_week_start_ms += MS_IN_WEEK;
                if current_week_start_ms >= helper::get_time_ms() as i64 {
                    println!("{:?} > Finished!", pair);
                    break;
                }
            }
        });
    }

    std::thread::sleep(std::time::Duration::new(20000, 0));
}

fn get_weeks_num(seconds: i64) -> i64 {
    (seconds - YEAR_ZERO_WEEK) / (86400_000 * 7)
}

fn get_file_path(pair: &Pair, weeks_num: i64) -> String {
    let s = format!("./data/{:?}/{}.csv", pair, weeks_num);
    s
}

fn get_folder_path(pair: &Pair, weeks_num: i64) -> String {
    let s = format!("./data/{:?}/{}.csv", pair, weeks_num);
    s
}

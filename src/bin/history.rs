use std::thread;
use trader3;
use trader3::ctrader::{CTrader, Config};
use trader3::online::assets;
use trader3::online::assets::Pair;
use trader3::{helper, online};

fn main() {
    run();
}

const YEAR_ZERO_WEEK: i64 = 1609632000_000; // Sunday, 3 January 2021 00:00:00
                                            // const START_WEEK: i64 = 1625356800_000; // 3 Jan 2021
const START_WEEK: i64 = YEAR_ZERO_WEEK + 15 * MS_IN_WEEK; // 3 Jan 2021
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
    // let pair_ids = vec![8];// todo: remove
    for pair_id in pair_ids {
        let pair = assets::Pair::id_to_symbol(pair_id);
        let cfg = cfg.clone();
        thread::spawn(move || {
            let mut current_week_start_ms = START_WEEK;

            loop {
                let week_id = get_weeks_num(current_week_start_ms);

                println!(
                    "{:?} - {} ",
                    pair,
                    helper::to_time_string(helper::get_time_sec() as i64)
                );
                let con_res = CTrader::connect2(&cfg);

                let end_week_time = current_week_start_ms + MS_IN_WEEK;
                let csv_res = trader3::collector::collect_data_from_api_csv(
                    con_res,
                    &pair,
                    current_week_start_ms,
                    end_week_time,
                );

                let folder = get_folder_path(&pair);
                let file_path = get_file_path(&pair, week_id);
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

    std::thread::sleep(std::time::Duration::new(2000000, 0));
}

fn get_weeks_num(seconds: i64) -> i64 {
    (seconds - YEAR_ZERO_WEEK) / (86400_000 * 7) + 1
}

fn get_file_path(pair: &Pair, weeks_num: i64) -> String {
    let s = format!("./data/{:?}/{}.csv", pair, weeks_num);
    s
}

fn get_folder_path(pair: &Pair) -> String {
    let s = format!("./data/{:?}", pair);
    s
}

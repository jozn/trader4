use crate::online::bot::Bot;
use super::*;

pub fn run_bot() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "l4jT24BWu3etFSEVViQKu1NsGpBYf2nKN0DyUGgqjy0".to_string(),
        ctid: 22851452,
    };
    let (mut cti, rc_event) = CTrader::connect(&cfg);
    // let mut ct = cti.lock().unwrap();
    let mut ct = cti;
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);

    std::thread::sleep(std::time::Duration::new(2, 0));

    let bot1 = Bot {
        con: ct,
        db: vec![]
    };

    bot1.on_connect();
    bot1.listen_events(rc_event);
}

pub fn run_bot_old() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "l4jT24BWu3etFSEVViQKu1NsGpBYf2nKN0DyUGgqjy0".to_string(),
        ctid: 22851452,
    };
    let (mut cti, rc_event) = CTrader::connect(&cfg);
    // let mut ct = cti.lock().unwrap();
    let mut ct = cti;
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);

    std::thread::sleep(std::time::Duration::new(2, 0));

    let bot1 = Bot1 {
        con: ct,
        last_tick: None,
        mini_tick: Default::default(),
        ticks_arr: Default::default(),
        candles: Default::default(),
    };

    bot1.on_connect();
    bot1.listen_events(rc_event);
}

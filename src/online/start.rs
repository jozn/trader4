use super::*;
use crate::online::bot::Bot;

pub fn run_bot() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "l4jT24BWu3etFSEVViQKu1NsGpBYf2nKN0DyUGgqjy0".to_string(),
        ctid: 22851452,
    };
    // let (mut cti, rc_event) = CTrader::connect(&cfg);
    let con_res = CTrader::connect2(&cfg);
    // let mut ct = cti.lock().unwrap();
    let mut ct = con_res.conn;
    // ct.application_auth_req(&cfg.client_id, &cfg.client_secret);

    // std::thread::sleep(std::time::Duration::new(2, 0));

    let bot1 = Bot {
        con: ct,
        db: vec![],
    };

    bot1.on_connect();
    bot1.listen_events(con_res.response_chan);
}

pub mod connection;
pub mod helper;
pub mod proc;

pub use connection::*;
pub use helper::*;
pub use proc::*;

pub fn play() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "KY38LUX8bXJxVGyIDRJfAz0PsEn-mRypVOpEsQd1C8k".to_string(),
        ctid: 22758966,
    };
    let mut cti = CTrader::connect(&cfg);
    // let mut ct = cti.lock().unwrap();
    let mut ct = cti;
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);

    std::thread::sleep(std::time::Duration::new(2, 0));

    ct.list_assets_req();
    ct.version_req();
    ct.list_symbols_req();
    ct.symbol_by_id_req(vec![1, 22398]);

    // ct.subscribe_spots_req(vec![22397,22398]);

    ct.get_trendbars_req();
    ct.get_tick_data_req();

    std::thread::sleep(std::time::Duration::new(100000, 0));
}

// Todo: clean imports
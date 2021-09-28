use serde::{Deserialize, Serialize};

use crate::pb;
use byteorder::ByteOrder;

use crate::pb::PayloadType;
use bytes::BufMut;
use native_tls::{TlsConnector, TlsStream};
use std::convert::{TryFrom, TryInto};
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tcp_stream::HandshakeError;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub client_secret: String,
    pub client_token: String,
    pub ctid: i64,
}

pub type CTraderInst = Arc<CTrader>;

#[derive(Debug)]
pub struct CTrader {
    // req_counter: Box<u64>,
    // req_counter2: std::sync::atomic::AtomicU64,
    // req_counter2: std::cell::RefCell<u64>,
    cfg: Config,
    w_ch: std::sync::mpsc::SyncSender<Vec<u8>>,
    pub(crate) stream: Arc<Mutex<TlsStream<TcpStream>>>,
}

impl CTrader {
    pub fn connect(cfg: &Config) -> Arc<CTrader> {
        let addr = format!("{}:{}", cfg.host, cfg.port);

        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(&addr).unwrap();
        stream.set_read_timeout(Some(Duration::new(4, 0)));
        let mut stream = connector.connect(&cfg.host, stream).unwrap();

        stream
            .get_mut()
            .set_read_timeout(Some(Duration::new(0, 500000))); // 0.5 second

        // Channel making
        let (sender_ch, reciver_ch) = std::sync::mpsc::sync_channel(1000);
        let mut out = Self {
            // req_counter: Box::new(0),
            // req_counter2: Default::default(),
            cfg: cfg.clone(),
            w_ch: sender_ch,
            stream: Arc::new(Mutex::new(stream)),
        };

        // let ro = Arc::new(Mutex::new(out));
        let ro = Arc::new(out);
        dispatch_write_thread(ro.clone(), reciver_ch);
        dispatch_read_thread(ro.clone());

        ro
    }

    fn send(&self, msg: impl prost::Message, msg_type: u32) {
        let _msg_id = format!("t {:?}", std::time::SystemTime::now());

        let mut buff = to_pb_frame(msg, msg_type);
        self.w_ch.send(buff);
    }
}

// For Api only
impl CTrader {
    pub fn application_auth_req(&self, client_id: &str, client_secret: &str) {
        let req_pb_auth = pb::ApplicationAuthReq {
            // payload_type: None, //Some(pb::PayloadType::OaApplicationAuthReq as i32),
            payload_type: Some(pb::PayloadType::OaApplicationAuthReq as i32), //Some(pb::PayloadType::OaApplicationAuthReq as i32),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        };

        self.send(req_pb_auth, pb::PayloadType::OaApplicationAuthReq as u32)
    }

    pub fn account_auth_req(&self) {
        let req_pb_account = pb::AccountAuthReq {
            // payload_type: None,
            payload_type: Some(pb::PayloadType::OaAccountAuthReq as i32),
            ctid_trader_account_id: self.cfg.ctid as i64,
            access_token: self.cfg.client_token.clone(),
        };

        self.send(req_pb_account, pb::PayloadType::OaAccountAuthReq as u32)
    }

    pub fn version_req(&self) {
        let api_id = pb::PayloadType::OaVersionReq as u32;

        let req_pb_ver = pb::VersionReq {
            // payload_type: Some(api_id as i32),
            payload_type: None,
        };

        self.send(req_pb_ver, api_id)
    }

    pub fn list_assets_req(&self) {
        let api_id = pb::PayloadType::OaAssetListReq as u32;

        let req_pb_account = pb::AssetListReq {
            payload_type: Some(api_id as i32),
            ctid_trader_account_id: self.cfg.ctid as i64,
        };

        self.send(req_pb_account, api_id)
    }

    pub fn list_symbols_req(&self) {
        let api_id = pb::PayloadType::OaSymbolsListReq as u32;

        let req_pb_sym = pb::SymbolsListReq {
            payload_type: Some(api_id as i32),
            ctid_trader_account_id: self.cfg.ctid as i64,
            // include_archived_symbols: Some(false)
            include_archived_symbols: None,
        };

        self.send(req_pb_sym, api_id)
    }

    pub fn symbol_by_id_req(&self, symbols: Vec<i64>) {
        let api_id = pb::PayloadType::OaSymbolByIdReq as u32;

        let req_pb = pb::SymbolByIdReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid as i64,
            symbol_id: symbols,
        };

        self.send(req_pb, api_id)
    }

    //todo
    pub fn symbol_for_conv_req(&self, symbols: Vec<i64>) {
        let api_id = pb::PayloadType::OaSymbolsForConversionReq as u32;

        let req_pb = pb::SymbolsForConversionReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid,
            first_asset_id: 0,
            last_asset_id: 0,
        };

        self.send(req_pb, api_id)
    }

    pub fn subscribe_spots_req(&self, symbols: Vec<i64>) {
        let api_id = pb::PayloadType::OaSubscribeSpotsReq as u32;

        let req_pb = pb::SubscribeSpotsReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid,
            symbol_id: symbols,
        };

        self.send(req_pb, api_id)
    }

    pub fn unsubscribe_spots_req(&self, symbols: Vec<i64>) {
        let api_id = pb::PayloadType::OaUnsubscribeSpotsReq as u32;

        let req_pb = pb::UnsubscribeSpotsReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid,
            symbol_id: symbols,
        };

        self.send(req_pb, api_id)
    }

    //todo
    pub fn deal_list_req(&self, symbols: Vec<i64>) {
        let api_id = pb::PayloadType::OaSubscribeSpotsReq as u32;

        let req_pb = pb::DealListReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid,
            from_timestamp: 0,
            to_timestamp: 0,
            max_rows: None,
        };

        self.send(req_pb, api_id)
    }

    pub fn get_trendbars_req(&self) {
        let api_id = pb::PayloadType::OaGetTrendbarsReq as u32;

        let d = 1632500363306;
        let req_pb = pb::GetTrendbarsReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid,
            from_timestamp: d - 86400_000,
            to_timestamp: 1632500363306,
            period: pb::TrendbarPeriod::M1 as i32,
            symbol_id: 1,
        };

        self.send(req_pb, api_id)
    }

    pub fn get_tick_data_req(&self) {
        let api_id = pb::PayloadType::OaGetTickdataReq as u32;

        let d = 1630000000_000;
        let req_pb = pb::GetTickDataReq {
            payload_type: None,
            ctid_trader_account_id: self.cfg.ctid,
            symbol_id: 1,
            r#type: 1,
            from_timestamp: d - 100_000,
            to_timestamp: d,
        };

        self.send(req_pb, api_id)
    }

    pub fn send_heartbeat_event(&self) {
        // println!("=========== Sending heart beat");
        let pong = pb::ProtoHeartbeatEvent {
            payload_type: Some(pb::ProtoPayloadType::HeartbeatEvent as i32),
        };

        self.send(pong, 51)
    }
}

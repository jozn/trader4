use serde::{Deserialize, Serialize};

use crate::online::pb;
use byteorder::ByteOrder;

use crate::core::helper;
use crate::online::pb::PayloadType;
use crate::online::ResponseEvent::DisConnected;
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

pub fn dispatch_read_thread(ctrader: CTraderInst) {
    // let stream_lock = ctrader.stream.clone();
    let stream_lock = ctrader.inner.clone();
    thread::spawn(move || {
        let mut total_buff = bytes::BytesMut::with_capacity(100_000_000); // ~100MB

        let mut refresh_time = helper::get_time_sec();
        let mut reconnect_time = helper::get_time_sec();
        // we read socket for maximum of 200 times(or 100ms at max) and then block for 500ms read socket - so we can read 400 times per second.
        'outer: loop {
            let start_time = helper::get_time_ms();
            for i in 0..200 {
                // Note: Each cTrader send frame seems to be maxed at 16KB.
                let mut read_vec = [0; 1024 * 1024].to_vec(); // Note: if we take this out of loop for performance it will produce a bug

                let mut locket_stream = stream_lock.lock().unwrap();
                let read_res = locket_stream.borrow_mut().stream.read(&mut read_vec);
                drop(locket_stream);
                if ctrader.is_disconnect() {
                    println!("[ENDING] Existing read loop.");
                    break 'outer;
                }
                match read_res {
                    Ok(0) => {
                        // Every 10 seconds send reconnect event
                        if helper::get_time_sec() - reconnect_time > 10 {
                            println!("cTrader socket is closed. Needs to reconnect");
                            ctrader.response_chan.send(ResponseEvent::DisConnected);
                            reconnect_time = helper::get_time_sec();
                        }
                        total_buff.clear();
                        // No break we only break if we are call disconented
                        // break;
                    }
                    Ok(v) => {
                        // todo buffer
                        let data = &read_vec[0..v];
                        total_buff.put_slice(data);
                        // let mut tb = total_buff.to_vec().as_slice();
                        let mut tb = total_buff.to_vec();
                        // println!("[IN]-> len {:?} --- {}", v, tb.len());
                        if tb.len() > 3 {
                            let frame_len = byteorder::BE::read_u32(&tb[0..4]);
                            // Note: This is actully should be == but we do a bigger check if it sends
                            //  more than multi per frame
                            if tb.len() as u32 >= (frame_len + 4) {
                                process_res_data(&tb, ctrader.clone());
                                total_buff.clear();
                            } else {
                            }
                        }
                    }
                    Err(e) => {
                        // we do not block, it will be error if there is nothing to read
                        // println!(">>> read err  {:?}", e);
                    }
                }

                // each loop 100ms budget at most
                if helper::get_time_ms() - start_time > 100 {
                    break;
                }
            }

            // Tick(Refresh) the world
            if helper::get_time_sec() - refresh_time > 60 {
                ctrader.response_chan.clone().send(RE::Refresh);
                refresh_time = helper::get_time_sec();
            }

            std::thread::sleep(Duration::from_millis(500)); // 0.5 seconds
        }
    });
}

pub fn dispatch_write_thread(ctrader: CTraderInst, ch: mpsc::Receiver<Vec<u8>>) {
    // let stream_lock = ctrader.stream.clone();
    let stream_lock = ctrader.inner.clone();
    thread::spawn(move || loop {
        let msg_data = ch.recv();
        if ctrader.is_disconnect() {
            println!("[ENDING] Existing ping loop.");
            break;
        }
        match msg_data {
            Ok(msg_data) => {
                // println!("------------- wire {:?}", msg_data.len());
                let mut locket_stream = stream_lock.lock().unwrap();
                locket_stream.borrow_mut().stream.write(&msg_data);
            }
            Err(e) => {
                println!("Error in sending data thread channel {:?}", e);
            }
        }
    });
}

pub fn dispatch_ping_loop(ctrader: CTraderInst) {
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::new(30, 0));
        if ctrader.is_disconnect() {
            println!("[ENDING] Existing ping loop.");
            break;
        }
        ctrader.send_heartbeat_event();
    });
}

pub fn process_res_data(data: &[u8], ctrader: CTraderInst) {
    let mut data = data.clone();
    if data.len() < 4 {
        println!("[Error]: Data frame len is invalid. {}", data.len());
        return;
    }
    let data = &data[4..];
    let msg: Result<pb::ProtoMessage, prost::DecodeError> = prost::Message::decode(data);
    // println!("++++++++++++++++ msg {:?}", &msg);
    match msg {
        Err(e) => {
            println!("[Error]: in decoding pb res {} -- {:?}", data.len(), e);
        }
        Ok(pb_msg) => {
            id_to_res_event(pb_msg, ctrader);
        }
    }
}

pub fn to_pb_frame(msg: impl prost::Message, msg_type: u32) -> Vec<u8> {
    let mut buff = vec![];
    let res = prost::Message::encode(&msg, &mut buff);

    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let msg_id = format!("{:}", t.as_micros());

    let pb_msg = pb::ProtoMessage {
        payload_type: msg_type,
        payload: Some(buff),
        client_msg_id: Some(msg_id),
    };

    let mut buff = vec![];
    let res = prost::Message::encode(&pb_msg, &mut buff);

    let mut out_buff = [0; 4].to_vec(); // For cTrader Frame 4 bytes + pb proto bytes
    byteorder::BE::write_u32(&mut out_buff, buff.len() as u32);
    out_buff.write(&buff);

    assert_eq!(out_buff.len(), buff.len() + 4);
    out_buff
}

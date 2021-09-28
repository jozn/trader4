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

pub fn id_to_res_event(pb_msg: pb::ProtoMessage, ctrader: CTraderInst) {
    println!("++++++++++++++ {}", pb_msg.payload_type);
    // This is ping request
    if pb_msg.payload_type == pb::ProtoPayloadType::HeartbeatEvent as u32 {
        ctrader.send_heartbeat_event();
        return;
    }

    // This is error
    if pb_msg.payload_type == pb::ProtoPayloadType::ErrorRes as u32 {
        let b = pb_msg.payload.unwrap();
        let msg: pb::ProtoMessage = prost::Message::decode(b.as_slice()).unwrap();
        println!(">> eroro respone  {:?}", msg);
        return;
    }

    if pb_msg.payload.is_none() {
        println!(">>> Error: Empty payload data.");
        return;
    }

    let b = pb_msg.payload.unwrap();
    // Note: we Added type as IDE can find .from_i32 from prost macros
    let e: Option<pb::PayloadType> = pb::PayloadType::from_i32(pb_msg.payload_type as i32);
    println!(">> enum {:?}   - len = {}", e, b.len());
    if e.is_none() {
        println!(
            "[Error]: could not pares recived recived proto. {}",
            pb_msg.payload_type
        );
        return;
    }

    let pl_type = e.unwrap();

    use PayloadType::*;
    match pl_type {
        OaApplicationAuthReq => {}
        OaApplicationAuthRes => {
            let msg: pb::ApplicationAuthRes = prost::Message::decode(b.as_slice()).unwrap();
            println!("======> res auth: {:#?}", msg);
            ctrader.account_auth_req();
        }
        OaAccountAuthReq => {}
        OaAccountAuthRes => {
            let msg: pb::AccountAuthRes = to_pb_res(&b);
        }
        OaVersionReq => {}
        OaVersionRes => {
            let msg: pb::VersionRes = to_pb_res(&b);
        }
        OaNewOrderReq => {}
        OaTrailingSlChangedEvent => {}
        OaCancelOrderReq => {}
        OaAmendOrderReq => {}
        OaAmendPositionSltpReq => {}
        OaClosePositionReq => {}
        OaAssetListReq => {}
        OaAssetListRes => {
            let msg: pb::AssetListRes = to_pb_res(&b);
        }
        OaSymbolsListReq => {}
        OaSymbolsListRes => {
            let msg: pb::SymbolsListRes = to_pb_res(&b);
        }
        OaSymbolByIdReq => {}
        OaSymbolByIdRes => {
            let msg: pb::SymbolByIdRes = to_pb_res(&b);
        }
        OaSymbolsForConversionReq => {}
        OaSymbolsForConversionRes => {
            let msg: pb::SymbolsForConversionRes = to_pb_res(&b);
        }
        OaSymbolChangedEvent => {}
        OaTraderReq => {}
        OaTraderRes => {}
        OaTraderUpdateEvent => {}
        OaReconcileReq => {}
        OaReconcileRes => {}
        OaExecutionEvent => {}
        OaSubscribeSpotsReq => {}
        OaSubscribeSpotsRes => {
            let msg: pb::SubscribeSpotsRes = to_pb_res(&b);
        }
        OaUnsubscribeSpotsReq => {}
        OaUnsubscribeSpotsRes => {
            let msg: pb::UnsubscribeSpotsReq = to_pb_res(&b);
        }
        OaSpotEvent => {
            let msg: pb::SpotEvent = to_pb_res(&b);
        }
        OaOrderErrorEvent => {}
        OaDealListReq => {}
        OaDealListRes => {
            let msg: pb::DealListRes = to_pb_res(&b);
        }
        OaSubscribeLiveTrendbarReq => {}
        OaUnsubscribeLiveTrendbarReq => {}
        OaGetTrendbarsReq => {}
        OaGetTrendbarsRes => {
            let msg: pb::GetTrendbarsRes = to_pb_res(&b);
        }
        OaExpectedMarginReq => {}
        OaExpectedMarginRes => {}
        OaMarginChangedEvent => {}
        OaErrorRes => {
            let msg: pb::ErrorRes = to_pb_res(&b);
        }
        OaCashFlowHistoryListReq => {}
        OaCashFlowHistoryListRes => {}
        OaGetTickdataReq => {}
        OaGetTickdataRes => {
            let msg: pb::GetTickDataRes = to_pb_res(&b);
        }
        OaAccountsTokenInvalidatedEvent => {}
        OaClientDisconnectEvent => {}
        OaGetAccountsByAccessTokenReq => {}
        OaGetAccountsByAccessTokenRes => {}
        OaGetCtidProfileByTokenReq => {}
        OaGetCtidProfileByTokenRes => {}
        OaAssetClassListReq => {}
        OaAssetClassListRes => {}
        OaDepthEvent => {}
        OaSubscribeDepthQuotesReq => {}
        OaSubscribeDepthQuotesRes => {}
        OaUnsubscribeDepthQuotesReq => {}
        OaUnsubscribeDepthQuotesRes => {}
        OaSymbolCategoryReq => {}
        OaSymbolCategoryRes => {}
        OaAccountLogoutReq => {}
        OaAccountLogoutRes => {}
        OaAccountDisconnectEvent => {}
        OaSubscribeLiveTrendbarRes => {}
        OaUnsubscribeLiveTrendbarRes => {}
        OaMarginCallListReq => {}
        OaMarginCallListRes => {}
        OaMarginCallUpdateReq => {}
        OaMarginCallUpdateRes => {}
        OaMarginCallUpdateEvent => {}
        OaMarginCallTriggerEvent => {}
        OaRefreshTokenReq => {}
        OaRefreshTokenRes => {}
    }
}

// todo: error
fn to_pb_res<T: prost::Message + Default>(arr: &Vec<u8>) -> T {
    let msg: T = prost::Message::decode(arr.as_slice()).unwrap();
    println!("======> res ============ : {:#?}", msg);
    msg
}

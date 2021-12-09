use crate::pb;
use crate::pb::*;

// Deprecated
pub trait OnResponse {
    fn onApplicationAuthRes(&mut self, res: pb::ApplicationAuthRes) {}

    fn onAccountAuthRes(&mut self, res: pb::AccountAuthRes) {}
}

pub type RE = ResponseEvent; // shortcut

#[derive(Debug, Clone)]
pub enum ResponseEvent {
    Refresh,      // Send by us to refresh the world and update things - every seconds
    DisConnected, // The socket is disconnected needs to reconnect again
    ApplicationAuthRes(pb::ApplicationAuthRes),
    AccountAuthRes(pb::AccountAuthRes),
    VersionRes(pb::VersionRes),
    AssetListRes(pb::AssetListRes),
    SymbolsListRes(pb::SymbolsListRes),
    SymbolByIdRes(pb::SymbolByIdRes),
    SymbolsForConversionRes(pb::SymbolsForConversionRes),
    TraderRes(pb::TraderRes),
    TraderUpdatedEvent(pb::TraderUpdatedEvent),
    ReconcileRes(pb::ReconcileRes),
    ExecutionEvent(pb::ExecutionEvent),
    SubscribeSpotsRes(pb::SubscribeSpotsRes),
    UnsubscribeSpotsReq(pb::UnsubscribeSpotsReq),
    SpotEvent(pb::SpotEvent),
    OrderErrorEvent(pb::OrderErrorEvent),
    DealListRes(pb::DealListRes),
    GetTrendbarsRes(pb::GetTrendbarsRes),
    ErrorRes(pb::ErrorRes),
    GetTickDataRes(pb::GetTickDataRes),
    AssetClassListRes(pb::AssetClassListRes),
    SubscribeDepthQuotesRes(pb::SubscribeDepthQuotesRes),
    UnsubscribeDepthQuotesRes(pb::UnsubscribeDepthQuotesRes),
    SymbolCategoryListRes(pb::SymbolCategoryListRes),
}

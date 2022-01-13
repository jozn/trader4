pub mod assets;
pub mod gen;

// Note: TAsset and TSymbol is the same of cli of assets cTrader lists downloader code, but in here
//  we replace String types with "&'static str" for simple outcome. This two codes must
//  be in sync.

#[derive(Debug, Clone)]
pub struct TAsset {
    pub name: &'static str,
    pub asset_id: i64,
    pub digits: i32,
}

#[derive(Debug, Clone)]
pub struct TSymbol {
    pub name: &'static str,
    pub symbol_id: i64,
    pub base_asset: &'static str,
    pub quote_asset: &'static str,
    pub category: &'static str,
    pub class: &'static str,
    pub description: &'static str,
    pub digits: i32,
    pub pip: i32,
}

/// --- INTENSIVE COMMANDS 1 - 49
/// --- COMMON API 50 - 69
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoPayloadType {
    /// common intensive
    ProtoMessage = 5,
    /// common commands
    ErrorRes = 50,
    HeartbeatEvent = 51,
}
/// COMMON error codes 1 - 99
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoErrorCode {
    /// Generic error.
    UnknownError = 1,
    /// Message is not supported. Wrong message.
    UnsupportedMessage = 2,
    /// Generic error.  Usually used when input value is not correct.
    InvalidRequest = 3,
    /// Deal execution is reached timeout and rejected.
    TimeoutError = 5,
    /// Generic error for requests by id.
    EntityNotFound = 6,
    /// Connection to Server is lost or not supported.
    CantRouteRequest = 7,
    /// Message is too large.
    FrameTooLong = 8,
    /// Market is closed.
    MarketClosed = 9,
    /// Order is blocked (e.g. under execution) and change cannot be applied.
    ConcurrentModification = 10,
    /// Message is blocked by server.
    BlockedPayloadType = 11,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoMessage {
    /// Contains id of ProtoPayloadType or other custom PayloadTypes (e.g. PayloadType)
    #[prost(uint32, required, tag = "1")]
    pub payload_type: u32,
    /// Serialized protobuf message that corresponds to payloadType
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payload: std::option::Option<std::prelude::rust_2015::Vec<u8>>,
    /// Request message id, assigned by the client that will be returned in the response
    #[prost(string, optional, tag = "3")]
    pub client_msg_id: std::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoErrorRes {
    #[prost(
        enumeration = "ProtoPayloadType",
        optional,
        tag = "1",
        default = "ErrorRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Contains name of ProtoErrorCode or other custom ErrorCodes (e.g. ProtoCHErrorCode)
    #[prost(string, required, tag = "2")]
    pub error_code: ::prost::alloc::string::String,
    /// Error description
    #[prost(string, optional, tag = "3")]
    pub description: std::option::Option<::prost::alloc::string::String>,
    /// CS-10489 Epoch timestamp in second
    #[prost(uint64, optional, tag = "4")]
    pub maintenance_end_timestamp: std::option::Option<u64>,
}
///* Event that is sent from Open API proxy and can be used as criteria that connection is healthy when no other messages are sent by cTrader platform. Open API client can send this message when he needs to keep the connection open for a period without other messages longer than 30 seconds
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHeartbeatEvent {
    #[prost(
        enumeration = "ProtoPayloadType",
        optional,
        tag = "1",
        default = "HeartbeatEvent"
    )]
    pub payload_type: std::option::Option<i32>,
}
///* Asset entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The unique asset ID.
    #[prost(int64, required, tag = "1")]
    pub asset_id: i64,
    /// The asset name.
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// User friendly name.
    #[prost(string, optional, tag = "3")]
    pub display_name: std::option::Option<::prost::alloc::string::String>,
    /// Precision of the asset.
    #[prost(int32, optional, tag = "4")]
    pub digits: std::option::Option<i32>,
}
///* Trading symbol entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symbol {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Number of price digits to be displayed.
    #[prost(int32, required, tag = "2")]
    pub digits: i32,
    /// Pip position on digits.
    #[prost(int32, required, tag = "3")]
    pub pip_position: i32,
    /// If TRUE then the short selling with the symbol is enabled.
    #[prost(bool, optional, tag = "4")]
    pub enable_short_selling: std::option::Option<bool>,
    /// If TRUE then setting of guaranteedStopLoss is available for limited risk accounts.
    #[prost(bool, optional, tag = "5")]
    pub guaranteed_stop_loss: std::option::Option<bool>,
    /// Day of the week when SWAP charge amount will be tripled. Doesn't impact Rollover Commission.
    #[prost(enumeration = "DayOfWeek", optional, tag = "6", default = "Monday")]
    pub swap_rollover3_days: std::option::Option<i32>,
    /// SWAP charge for long positions.
    #[prost(double, optional, tag = "7")]
    pub swap_long: std::option::Option<f64>,
    /// SWAP charge for short positions.
    #[prost(double, optional, tag = "8")]
    pub swap_short: std::option::Option<f64>,
    /// Maximum allowed volume in cents for an order with a symbol.
    #[prost(int64, optional, tag = "9")]
    pub max_volume: std::option::Option<i64>,
    /// Minimum allowed volume in cents for an order with a symbol.
    #[prost(int64, optional, tag = "10")]
    pub min_volume: std::option::Option<i64>,
    /// Step of the volume in cents for an order.
    #[prost(int64, optional, tag = "11")]
    pub step_volume: std::option::Option<i64>,
    /// Value of max exposure per symbol, per account. Blocks execution if breached.
    #[prost(uint64, optional, tag = "12")]
    pub max_exposure: std::option::Option<u64>,
    /// Symbol trading interval, specified in seconds starting from SUNDAY 00:00 in specified time zone.
    #[prost(message, repeated, tag = "13")]
    pub schedule: std::prelude::rust_2015::Vec<Interval>,
    /// Commission base amount. Total commission depends on commissionType. Use preciseTradingCommissionRate.
    #[deprecated]
    #[prost(int64, optional, tag = "14")]
    pub commission: std::option::Option<i64>,
    /// Commission type. See CommissionType for details.
    #[prost(
        enumeration = "CommissionType",
        optional,
        tag = "15",
        default = "UsdPerMillionUsd"
    )]
    pub commission_type: std::option::Option<i32>,
    /// Minimum allowed distance between stop loss and current market price.
    #[prost(uint32, optional, tag = "16")]
    pub sl_distance: std::option::Option<u32>,
    /// Minimum allowed distance between take profit and current market price.
    #[prost(uint32, optional, tag = "17")]
    pub tp_distance: std::option::Option<u32>,
    /// Minimum allowed distance between guaranteed stop loss and current market price.
    #[prost(uint32, optional, tag = "18")]
    pub gsl_distance: std::option::Option<u32>,
    /// Guaranteed stop loss fee.
    #[prost(int64, optional, tag = "19")]
    pub gsl_charge: std::option::Option<i64>,
    /// Unit of distance measure for slDistance, tpDistance, gslDistance.
    #[prost(
        enumeration = "SymbolDistanceType",
        optional,
        tag = "20",
        default = "SymbolDistanceInPoints"
    )]
    pub distance_set_in: std::option::Option<i32>,
    /// Minimum commission amount per trade. Use preciseMinCommission.
    #[deprecated]
    #[prost(int64, optional, tag = "21")]
    pub min_commission: std::option::Option<i64>,
    /// Minimum commission Type. See MinCommissionType for details.
    #[prost(
        enumeration = "MinCommissionType",
        optional,
        tag = "22",
        default = "Currency"
    )]
    pub min_commission_type: std::option::Option<i32>,
    /// Currency for minimum commission. (USD or quote currency).
    #[prost(string, optional, tag = "23", default = "USD")]
    pub min_commission_asset: std::option::Option<::prost::alloc::string::String>,
    /// Amount of commission per trade for Shariah Compliant accounts in deposit currency (swapFree = TRUE).
    #[prost(int64, optional, tag = "24")]
    pub rollover_commission: std::option::Option<i64>,
    /// Initial period before the first rolloverCommission will be charged on the account.
    #[prost(int32, optional, tag = "25")]
    pub skip_rollover_days: std::option::Option<i32>,
    /// Time zone for the symbol trading intervals.
    #[prost(string, optional, tag = "26")]
    pub schedule_time_zone: std::option::Option<::prost::alloc::string::String>,
    /// Rules for trading with the symbol. See TradingMode for details.
    #[prost(enumeration = "TradingMode", optional, tag = "27", default = "Enabled")]
    pub trading_mode: std::option::Option<i32>,
    /// Day of the week (in UTC) when Administrative Fee charge amount will be tripled. Applied only if RolloverChargePeriod = 0 or 1
    #[prost(enumeration = "DayOfWeek", optional, tag = "28", default = "Monday")]
    pub rollover_commission3_days: std::option::Option<i32>,
    /// Specifies type of SWAP computation as PIPS (0) or PERCENTAGE (1, annual, in percent)
    #[prost(
        enumeration = "SwapCalculationType",
        optional,
        tag = "29",
        default = "Pips"
    )]
    pub swap_calculation_type: std::option::Option<i32>,
    /// Lot size of the Symbol (in cents)
    #[prost(int64, optional, tag = "30")]
    pub lot_size: std::option::Option<i64>,
    /// Commission base amount. Total commission depends on commissionType: for non-percentage types it is multiplied by 10^8.
    #[prost(int64, optional, tag = "31")]
    pub precise_trading_commission_rate: std::option::Option<i64>,
    /// Minimum commission amount per trade multiplied by 10^8.
    #[prost(int64, optional, tag = "32")]
    pub precise_min_commission: std::option::Option<i64>,
    /// List of holidays for this symbol specified by broker.
    #[prost(message, repeated, tag = "33")]
    pub holiday: std::prelude::rust_2015::Vec<Holiday>,
}
///* Lightweight symbol entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightSymbol {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Name of the symbol (e.g. EUR/USD).
    #[prost(string, optional, tag = "2")]
    pub symbol_name: std::option::Option<::prost::alloc::string::String>,
    /// If TRUE then symbol is visible for traders.
    #[prost(bool, optional, tag = "3")]
    pub enabled: std::option::Option<bool>,
    /// Base asset.
    #[prost(int64, optional, tag = "4")]
    pub base_asset_id: std::option::Option<i64>,
    /// Quote asset.
    #[prost(int64, optional, tag = "5")]
    pub quote_asset_id: std::option::Option<i64>,
    /// Id of the symbol category used for symbols grouping.
    #[prost(int64, optional, tag = "6")]
    pub symbol_category_id: std::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub description: std::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchivedSymbol {
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, required, tag = "3")]
    pub utc_last_update_timestamp: i64,
    #[prost(string, optional, tag = "4")]
    pub description: std::option::Option<::prost::alloc::string::String>,
}
///* Symbol category entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolCategory {
    /// The unique identifier of the symbol category.
    #[prost(int64, required, tag = "1")]
    pub id: i64,
    /// Link to the asset class. One asset class can have many symbol categories.
    #[prost(int64, required, tag = "2")]
    pub asset_class_id: i64,
    /// Category name.
    #[prost(string, required, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
///* Symbol trading session entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interval {
    /// Interval start, specified in seconds starting from SUNDAY 00:00 in specified time zone (inclusive to the interval).
    #[prost(uint32, required, tag = "3")]
    pub start_second: u32,
    /// Interval end, specified in seconds starting from SUNDAY 00:00 in specified time zone (exclusive from the interval).
    #[prost(uint32, required, tag = "4")]
    pub end_second: u32,
}
///* Trading account entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trader {
    /// The unique Trader's Account ID used to match the responses to the Trader's Account.
    #[prost(int64, required, tag = "1")]
    pub ctid_trader_account_id: i64,
    /// Current account balance.
    #[prost(int64, required, tag = "2")]
    pub balance: i64,
    /// Balance version used to identify the final balance. Increments each time when the trader's account balance is changed.
    #[prost(int64, optional, tag = "3")]
    pub balance_version: std::option::Option<i64>,
    /// Amount of broker's bonus allocated to the account.
    #[prost(int64, optional, tag = "4")]
    pub manager_bonus: std::option::Option<i64>,
    /// Amount of introducing broker bonus allocated to the account.
    #[prost(int64, optional, tag = "5")]
    pub ib_bonus: std::option::Option<i64>,
    /// Broker's bonus that cannot be withdrew from the account as cash.
    #[prost(int64, optional, tag = "6")]
    pub non_withdrawable_bonus: std::option::Option<i64>,
    /// Access rights that an owner has to the account in cTrader platform. See AccessRights for details.
    #[prost(
        enumeration = "AccessRights",
        optional,
        tag = "7",
        default = "FullAccess"
    )]
    pub access_rights: std::option::Option<i32>,
    /// Deposit currency of the account.
    #[prost(int64, required, tag = "8")]
    pub deposit_asset_id: i64,
    /// If TRUE then account is Shariah compliant.
    #[prost(bool, optional, tag = "9")]
    pub swap_free: std::option::Option<bool>,
    /// Account leverage (e.g. If leverage = 1:50 then value = 5000).
    #[prost(uint32, optional, tag = "10")]
    pub leverage_in_cents: std::option::Option<u32>,
    /// Margin computation type for the account (MAX, SUM, NET).
    #[prost(enumeration = "TotalMarginCalculationType", optional, tag = "11")]
    pub total_margin_calculation_type: std::option::Option<i32>,
    /// Maximum allowed leverage for the account. Used as validation when a Trader can change leverage value.
    #[prost(uint32, optional, tag = "12")]
    pub max_leverage: std::option::Option<u32>,
    /// If TRUE then account is AMF compliant. Use isLimitedRisk and limitedRiskMarginCalculationStrategy.
    #[deprecated]
    #[prost(bool, optional, tag = "13")]
    pub french_risk: std::option::Option<bool>,
    /// ID of the account that is unique per server (Broker).
    #[prost(int64, optional, tag = "14")]
    pub trader_login: std::option::Option<i64>,
    /// Account type: HEDGED, NETTED, etc.
    #[prost(enumeration = "AccountType", optional, tag = "15", default = "Hedged")]
    pub account_type: std::option::Option<i32>,
    /// Some whitelabel assigned to trader by broker at the moment of account creation.
    #[prost(string, optional, tag = "16")]
    pub broker_name: std::option::Option<::prost::alloc::string::String>,
    /// Unix timestamp of the account registration. Should be used as minimal date in historical data requests.
    #[prost(int64, optional, tag = "17")]
    pub registration_timestamp: std::option::Option<i64>,
    /// If TRUE then account is compliant to use specific margin calculation strategy.
    #[prost(bool, optional, tag = "18")]
    pub is_limited_risk: std::option::Option<bool>,
    /// Special strategy used in margin calculations for this account (if account isLimitedRisk).
    #[prost(
        enumeration = "LimitedRiskMarginCalculationStrategy",
        optional,
        tag = "19",
        default = "AccordingToLeverage"
    )]
    pub limited_risk_margin_calculation_strategy: std::option::Option<i32>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects balance, managerBonus, ibBonus, nonWithdrawableBonus.
    #[prost(uint32, optional, tag = "20")]
    pub money_digits: std::option::Option<u32>,
}
///* Trade position entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// The unique ID of the position. Note: trader might have two positions with the same id if positions are taken from accounts from different brokers.
    #[prost(int64, required, tag = "1")]
    pub position_id: i64,
    /// Position details. See TradeData for details.
    #[prost(message, required, tag = "2")]
    pub trade_data: TradeData,
    /// Current status of the position.
    #[prost(enumeration = "PositionStatus", required, tag = "3")]
    pub position_status: i32,
    /// Total amount of charged swap on open position.
    #[prost(int64, required, tag = "4")]
    pub swap: i64,
    /// VWAP price of the position based on all executions (orders) linked to the position.
    #[prost(double, optional, tag = "5")]
    pub price: std::option::Option<f64>,
    /// Current stop loss price.
    #[prost(double, optional, tag = "6")]
    pub stop_loss: std::option::Option<f64>,
    /// Current take profit price.
    #[prost(double, optional, tag = "7")]
    pub take_profit: std::option::Option<f64>,
    /// Time of the last change of the position, including amend SL/TP of the position, execution of related order, cancel or related order, etc.
    #[prost(int64, optional, tag = "8")]
    pub utc_last_update_timestamp: std::option::Option<i64>,
    /// Current unrealized commission related to the position.
    #[prost(int64, optional, tag = "9")]
    pub commission: std::option::Option<i64>,
    /// Rate for used margin computation. Represented as Base/Deposit.
    #[prost(double, optional, tag = "10")]
    pub margin_rate: std::option::Option<f64>,
    /// Amount of unrealized commission related to following of strategy provider.
    #[prost(int64, optional, tag = "11")]
    pub mirroring_commission: std::option::Option<i64>,
    /// If TRUE then position's stop loss is guaranteedStopLoss.
    #[prost(bool, optional, tag = "12")]
    pub guaranteed_stop_loss: std::option::Option<bool>,
    /// Amount of margin used for the position in deposit currency.
    #[prost(uint64, optional, tag = "13")]
    pub used_margin: std::option::Option<u64>,
    /// Stop trigger method for SL/TP of the position.
    #[prost(
        enumeration = "OrderTriggerMethod",
        optional,
        tag = "14",
        default = "Trade"
    )]
    pub stop_loss_trigger_method: std::option::Option<i32>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects swap, commission, mirroringCommission, usedMargin.
    #[prost(uint32, optional, tag = "15")]
    pub money_digits: std::option::Option<u32>,
}
///* Position/order trading details entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeData {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Volume in cents.
    #[prost(int64, required, tag = "2")]
    pub volume: i64,
    /// Buy, Sell.
    #[prost(enumeration = "TradeSide", required, tag = "3")]
    pub trade_side: i32,
    /// Time when position was opened or order was created.
    #[prost(int64, optional, tag = "4")]
    pub open_timestamp: std::option::Option<i64>,
    /// Text label specified during order request.
    #[prost(string, optional, tag = "5")]
    pub label: std::option::Option<::prost::alloc::string::String>,
    /// If TRUE then position/order stop loss is guaranteedStopLoss.
    #[prost(bool, optional, tag = "6")]
    pub guaranteed_stop_loss: std::option::Option<bool>,
    /// User-specified comment.
    #[prost(string, optional, tag = "7")]
    pub comment: std::option::Option<::prost::alloc::string::String>,
}
///* Trade order entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// The unique ID of the order. Note: trader might have two orders with the same id if orders are taken from accounts from different brokers.
    #[prost(int64, required, tag = "1")]
    pub order_id: i64,
    /// Detailed trader data.
    #[prost(message, required, tag = "2")]
    pub trade_data: TradeData,
    /// Order type.
    #[prost(enumeration = "OrderType", required, tag = "3")]
    pub order_type: i32,
    /// Order status.
    #[prost(enumeration = "OrderStatus", required, tag = "4")]
    pub order_status: i32,
    /// If the order has time in force GTD then expiration is specified.
    #[prost(int64, optional, tag = "6")]
    pub expiration_timestamp: std::option::Option<i64>,
    /// Price at which an order was executed. For order with FILLED status.
    #[prost(double, optional, tag = "7")]
    pub execution_price: std::option::Option<f64>,
    /// Part of the volume that was filled.
    #[prost(int64, optional, tag = "8")]
    pub executed_volume: std::option::Option<i64>,
    /// Timestamp of the last update of the order.
    #[prost(int64, optional, tag = "9")]
    pub utc_last_update_timestamp: std::option::Option<i64>,
    /// Used for Market Range order with combination of slippageInPoints to specify price range were order can be executed.
    #[prost(double, optional, tag = "10")]
    pub base_slippage_price: std::option::Option<f64>,
    /// Used for Market Range and STOP_LIMIT orders to to specify price range were order can be executed.
    #[prost(int64, optional, tag = "11")]
    pub slippage_in_points: std::option::Option<i64>,
    /// If TRUE then the order is closing part of whole position. Must have specified positionId.
    #[prost(bool, optional, tag = "12")]
    pub closing_order: std::option::Option<bool>,
    /// Valid only for LIMIT orders.
    #[prost(double, optional, tag = "13")]
    pub limit_price: std::option::Option<f64>,
    /// Valid only for STOP and STOP_LIMIT orders.
    #[prost(double, optional, tag = "14")]
    pub stop_price: std::option::Option<f64>,
    /// Absolute stopLoss price.
    #[prost(double, optional, tag = "15")]
    pub stop_loss: std::option::Option<f64>,
    /// Absolute takeProfit price.
    #[prost(double, optional, tag = "16")]
    pub take_profit: std::option::Option<f64>,
    /// Optional ClientOrderId. Max Length = 50 chars.
    #[prost(string, optional, tag = "17")]
    pub client_order_id: std::option::Option<::prost::alloc::string::String>,
    /// Order's time in force. Depends on order type.
    #[prost(
        enumeration = "TimeInForce",
        optional,
        tag = "18",
        default = "ImmediateOrCancel"
    )]
    pub time_in_force: std::option::Option<i32>,
    /// ID of the position linked to the order (e.g. closing order, order that increase volume of a specific position, etc.).
    #[prost(int64, optional, tag = "19")]
    pub position_id: std::option::Option<i64>,
    /// Relative stopLoss that can be specified instead of absolute as one. Specified in 1/100_000 of unit of a price. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
    #[prost(int64, optional, tag = "20")]
    pub relative_stop_loss: std::option::Option<i64>,
    /// Relative takeProfit that can be specified instead of absolute one. Specified in 1/100_000 of unit of a price. ForBUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
    #[prost(int64, optional, tag = "21")]
    pub relative_take_profit: std::option::Option<i64>,
    /// If TRUE then order was stopped out from server side.
    #[prost(bool, optional, tag = "22")]
    pub is_stop_out: std::option::Option<bool>,
    /// If TRUE then order is trailingStopLoss. Valid for STOP_LOSS_TAKE_PROFIT order.
    #[prost(bool, optional, tag = "23")]
    pub trailing_stop_loss: std::option::Option<bool>,
    /// Trigger method for the order. Valid only for STOP and STOP_LIMIT orders.
    #[prost(
        enumeration = "OrderTriggerMethod",
        optional,
        tag = "24",
        default = "Trade"
    )]
    pub stop_trigger_method: std::option::Option<i32>,
}
///* Bonus deposit/withdrawal entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BonusDepositWithdraw {
    /// Type of the operation. Deposit/Withdrawal.
    #[prost(enumeration = "ChangeBonusType", required, tag = "1")]
    pub operation_type: i32,
    /// The unique ID of the bonus deposit/withdrawal operation.
    #[prost(int64, required, tag = "2")]
    pub bonus_history_id: i64,
    /// Total amount of broker's bonus after the operation.
    #[prost(int64, required, tag = "3")]
    pub manager_bonus: i64,
    /// Amount of bonus deposited/withdrew by manager.
    #[prost(int64, required, tag = "4")]
    pub manager_delta: i64,
    /// Total amount of introducing broker's bonus after the operation.
    #[prost(int64, required, tag = "5")]
    pub ib_bonus: i64,
    /// Amount of bonus deposited/withdrew by introducing broker.
    #[prost(int64, required, tag = "6")]
    pub ib_delta: i64,
    /// Time when the bonus operation was executed.
    #[prost(int64, required, tag = "7")]
    pub change_bonus_timestamp: i64,
    /// Note added to operation. Visible to the trader.
    #[prost(string, optional, tag = "8")]
    pub external_note: std::option::Option<::prost::alloc::string::String>,
    /// ID of introducing broker who deposited/withdrew bonus.
    #[prost(int64, optional, tag = "9")]
    pub introducing_broker_id: std::option::Option<i64>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects managerBonus, managerDelta, ibBonus, ibDelta.
    #[prost(uint32, optional, tag = "10")]
    pub money_digits: std::option::Option<u32>,
}
///* Account deposit/withdrawal operation entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositWithdraw {
    /// Type of the operation. Deposit/Withdrawal.
    #[prost(enumeration = "ChangeBalanceType", required, tag = "1")]
    pub operation_type: i32,
    /// The unique ID of the deposit/withdrawal operation.
    #[prost(int64, required, tag = "2")]
    pub balance_history_id: i64,
    /// Account balance after the operation was executed.
    #[prost(int64, required, tag = "3")]
    pub balance: i64,
    /// Amount of deposit/withdrawal operation.
    #[prost(int64, required, tag = "4")]
    pub delta: i64,
    /// Time when deposit/withdrawal operation was executed.
    #[prost(int64, required, tag = "5")]
    pub change_balance_timestamp: i64,
    /// Note added to operation. Visible to the trader.
    #[prost(string, optional, tag = "6")]
    pub external_note: std::option::Option<::prost::alloc::string::String>,
    /// Balance version used to identify the final balance. Increments each time when the trader's account balance is changed.
    #[prost(int64, optional, tag = "7")]
    pub balance_version: std::option::Option<i64>,
    /// Total account's equity after balance operation was executed.
    #[prost(int64, optional, tag = "8")]
    pub equity: std::option::Option<i64>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects balance, delta, equity.
    #[prost(uint32, optional, tag = "9")]
    pub money_digits: std::option::Option<u32>,
}
///* Execution entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deal {
    /// The unique ID of the execution deal.
    #[prost(int64, required, tag = "1")]
    pub deal_id: i64,
    /// Source order of the deal.
    #[prost(int64, required, tag = "2")]
    pub order_id: i64,
    /// Source position of the deal.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// Volume sent for execution, in cents.
    #[prost(int64, required, tag = "4")]
    pub volume: i64,
    /// Filled volume, in cents.
    #[prost(int64, required, tag = "5")]
    pub filled_volume: i64,
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs.
    #[prost(int64, required, tag = "6")]
    pub symbol_id: i64,
    /// Time when the deal was sent for execution.
    #[prost(int64, required, tag = "7")]
    pub create_timestamp: i64,
    /// Time when the deal was executed.
    #[prost(int64, required, tag = "8")]
    pub execution_timestamp: i64,
    /// Timestamp when the deal was created, executed or rejected.
    #[prost(int64, optional, tag = "9")]
    pub utc_last_update_timestamp: std::option::Option<i64>,
    /// Execution price.
    #[prost(double, optional, tag = "10")]
    pub execution_price: std::option::Option<f64>,
    /// Buy/Sell.
    #[prost(enumeration = "TradeSide", required, tag = "11")]
    pub trade_side: i32,
    /// Status of the deal.
    #[prost(enumeration = "DealStatus", required, tag = "12")]
    pub deal_status: i32,
    /// Rate for used margin computation. Represented as Base/Deposit.
    #[prost(double, optional, tag = "13")]
    pub margin_rate: std::option::Option<f64>,
    /// Amount of trading commission associated with the deal.
    #[prost(int64, optional, tag = "14")]
    pub commission: std::option::Option<i64>,
    /// Base to USD conversion rate on the time of deal execution.
    #[prost(double, optional, tag = "15")]
    pub base_to_usd_conversion_rate: std::option::Option<f64>,
    /// Closing position detail. Valid only for closing deal.
    #[prost(message, optional, tag = "16")]
    pub close_position_detail: std::option::Option<ClosePositionDetail>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects commission.
    #[prost(uint32, optional, tag = "17")]
    pub money_digits: std::option::Option<u32>,
}
///* Trading details for closing deal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePositionDetail {
    /// Position price at the moment of filling the closing order.
    #[prost(double, required, tag = "1")]
    pub entry_price: f64,
    /// Amount of realized gross profit after closing deal execution.
    #[prost(int64, required, tag = "2")]
    pub gross_profit: i64,
    /// Amount of realized swap related to closed volume.
    #[prost(int64, required, tag = "3")]
    pub swap: i64,
    /// Amount of realized commission related to closed volume.
    #[prost(int64, required, tag = "4")]
    pub commission: i64,
    /// Account balance after closing deal execution.
    #[prost(int64, required, tag = "5")]
    pub balance: i64,
    /// Quote/Deposit currency conversion rate on the time of closing deal execution.
    #[prost(double, optional, tag = "6")]
    pub quote_to_deposit_conversion_rate: std::option::Option<f64>,
    /// Closed volume in cents.
    #[prost(int64, optional, tag = "7")]
    pub closed_volume: std::option::Option<i64>,
    /// Balance version of the account related to closing deal operation.
    #[prost(int64, optional, tag = "8")]
    pub balance_version: std::option::Option<i64>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects grossProfit, swap, commission, balance.
    #[prost(uint32, optional, tag = "9")]
    pub money_digits: std::option::Option<u32>,
}
///* Historical Trendbar entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trendbar {
    /// Bar volume in ticks.
    #[prost(int64, required, tag = "3")]
    pub volume: i64,
    /// Bar period.
    #[prost(enumeration = "TrendbarPeriod", optional, tag = "4", default = "M1")]
    pub period: std::option::Option<i32>,
    /// Low price of the bar.
    #[prost(int64, optional, tag = "5")]
    pub low: std::option::Option<i64>,
    /// Delta between open and low price. open = low + deltaOpen.
    #[prost(uint64, optional, tag = "6")]
    pub delta_open: std::option::Option<u64>,
    /// Delta between close and low price. close = low + deltaClose.
    #[prost(uint64, optional, tag = "7")]
    pub delta_close: std::option::Option<u64>,
    /// Delta between high and low price. high = low + deltaHigh.
    #[prost(uint64, optional, tag = "8")]
    pub delta_high: std::option::Option<u64>,
    /// Timestamp of the bar. Equal to the timestamp of the open tick.
    #[prost(uint32, optional, tag = "9")]
    pub utc_timestamp_in_minutes: std::option::Option<u32>,
}
///* Expected margin computation entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpectedMargin {
    /// Volume in cents used for computation of expected margin.
    #[prost(int64, required, tag = "1")]
    pub volume: i64,
    /// Buy margin amount.
    #[prost(int64, required, tag = "2")]
    pub buy_margin: i64,
    /// Sell margin amount.
    #[prost(int64, required, tag = "3")]
    pub sell_margin: i64,
}
///* Historical tick data type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickData {
    /// Tick timestamp.
    #[prost(int64, required, tag = "1")]
    pub timestamp: i64,
    /// Tick price.
    #[prost(int64, required, tag = "2")]
    pub tick: i64,
}
///* Trader profile entity. Empty due to GDPR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CtidProfile {
    #[prost(int64, required, tag = "1")]
    pub user_id: i64,
}
///* Trader account entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CtidTraderAccount {
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.cTrader platform. Different brokers might have different ids
    #[prost(uint64, required, tag = "1")]
    pub ctid_trader_account_id: u64,
    /// If TRUE then the account is belong to Live environment and live host must be used to authorize it
    #[prost(bool, optional, tag = "2")]
    pub is_live: std::option::Option<bool>,
    /// TraderLogin for a specific account. Value is displayed on Client App UI
    #[prost(int64, optional, tag = "3")]
    pub trader_login: std::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub last_closing_deal_timestamp: std::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub last_balance_update_timestamp: std::option::Option<i64>,
}
///* Asset class entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetClass {
    /// Unique asset ID.
    #[prost(int64, optional, tag = "1")]
    pub id: std::option::Option<i64>,
    /// Asset class name.
    #[prost(string, optional, tag = "2")]
    pub name: std::option::Option<::prost::alloc::string::String>,
}
///* Depth of market entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepthQuote {
    /// Quote ID.
    #[prost(uint64, required, tag = "1")]
    pub id: u64,
    /// Quote size in cents.
    #[prost(uint64, required, tag = "3")]
    pub size: u64,
    /// Bid price for bid quotes.
    #[prost(uint64, optional, tag = "4")]
    pub bid: std::option::Option<u64>,
    /// Ask price for ask quotes.
    #[prost(uint64, optional, tag = "5")]
    pub ask: std::option::Option<u64>,
}
///* Margin call entity, specifies threshold for exact margin call type. Only 3 instances of margin calls are supported, identified by marginCallType. See NotificationType for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCall {
    #[prost(enumeration = "NotificationType", required, tag = "1")]
    pub margin_call_type: i32,
    #[prost(double, required, tag = "2")]
    pub margin_level_threshold: f64,
    #[prost(int64, optional, tag = "3")]
    pub utc_last_update_timestamp: std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Holiday {
    /// Unique ID of holiday.
    #[prost(int64, required, tag = "1")]
    pub holiday_id: i64,
    /// Name of holiday.
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Description of holiday.
    #[prost(string, optional, tag = "3")]
    pub description: std::option::Option<::prost::alloc::string::String>,
    /// Timezone used for holiday.
    #[prost(string, required, tag = "4")]
    pub schedule_time_zone: ::prost::alloc::string::String,
    /// Amount of days from 01-01-1970, multiply it by 86400000 to get unix timestamp.
    #[prost(int64, required, tag = "5")]
    pub holiday_date: i64,
    /// If TRUE, then the holiday happens each year.
    #[prost(bool, required, tag = "6")]
    pub is_recurring: bool,
    /// Amount of seconds from 00:00:00 of the holiday day when holiday actually starts.
    #[prost(int32, optional, tag = "7")]
    pub start_second: std::option::Option<i32>,
    /// Amount of seconds from 00:00:00 of the holiday day when holiday actually finishes.
    #[prost(int32, optional, tag = "8")]
    pub end_second: std::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadType {
    OaApplicationAuthReq = 2100,
    OaApplicationAuthRes = 2101,
    OaAccountAuthReq = 2102,
    OaAccountAuthRes = 2103,
    OaVersionReq = 2104,
    OaVersionRes = 2105,
    OaNewOrderReq = 2106,
    OaTrailingSlChangedEvent = 2107,
    OaCancelOrderReq = 2108,
    OaAmendOrderReq = 2109,
    OaAmendPositionSltpReq = 2110,
    OaClosePositionReq = 2111,
    OaAssetListReq = 2112,
    OaAssetListRes = 2113,
    OaSymbolsListReq = 2114,
    OaSymbolsListRes = 2115,
    OaSymbolByIdReq = 2116,
    OaSymbolByIdRes = 2117,
    OaSymbolsForConversionReq = 2118,
    OaSymbolsForConversionRes = 2119,
    OaSymbolChangedEvent = 2120,
    OaTraderReq = 2121,
    OaTraderRes = 2122,
    OaTraderUpdateEvent = 2123,
    OaReconcileReq = 2124,
    OaReconcileRes = 2125,
    OaExecutionEvent = 2126,
    OaSubscribeSpotsReq = 2127,
    OaSubscribeSpotsRes = 2128,
    OaUnsubscribeSpotsReq = 2129,
    OaUnsubscribeSpotsRes = 2130,
    OaSpotEvent = 2131,
    OaOrderErrorEvent = 2132,
    OaDealListReq = 2133,
    OaDealListRes = 2134,
    OaSubscribeLiveTrendbarReq = 2135,
    OaUnsubscribeLiveTrendbarReq = 2136,
    OaGetTrendbarsReq = 2137,
    OaGetTrendbarsRes = 2138,
    OaExpectedMarginReq = 2139,
    OaExpectedMarginRes = 2140,
    OaMarginChangedEvent = 2141,
    OaErrorRes = 2142,
    OaCashFlowHistoryListReq = 2143,
    OaCashFlowHistoryListRes = 2144,
    OaGetTickdataReq = 2145,
    OaGetTickdataRes = 2146,
    OaAccountsTokenInvalidatedEvent = 2147,
    OaClientDisconnectEvent = 2148,
    OaGetAccountsByAccessTokenReq = 2149,
    OaGetAccountsByAccessTokenRes = 2150,
    OaGetCtidProfileByTokenReq = 2151,
    OaGetCtidProfileByTokenRes = 2152,
    OaAssetClassListReq = 2153,
    OaAssetClassListRes = 2154,
    OaDepthEvent = 2155,
    OaSubscribeDepthQuotesReq = 2156,
    OaSubscribeDepthQuotesRes = 2157,
    OaUnsubscribeDepthQuotesReq = 2158,
    OaUnsubscribeDepthQuotesRes = 2159,
    OaSymbolCategoryReq = 2160,
    OaSymbolCategoryRes = 2161,
    OaAccountLogoutReq = 2162,
    OaAccountLogoutRes = 2163,
    OaAccountDisconnectEvent = 2164,
    OaSubscribeLiveTrendbarRes = 2165,
    OaUnsubscribeLiveTrendbarRes = 2166,
    OaMarginCallListReq = 2167,
    OaMarginCallListRes = 2168,
    OaMarginCallUpdateReq = 2169,
    OaMarginCallUpdateRes = 2170,
    OaMarginCallUpdateEvent = 2171,
    OaMarginCallTriggerEvent = 2172,
    OaRefreshTokenReq = 2173,
    OaRefreshTokenRes = 2174,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DayOfWeek {
    None = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}
///* Enum for specifying type of trading commission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommissionType {
    /// USD per million USD volume - usually used for FX. Example: 50 USD for 1 mil USD of trading volume.
    UsdPerMillionUsd = 1,
    /// USD per 1 lot - usually used for CFDs and futures for commodities, and indices. Example: 15 USD for 1 contract.
    UsdPerLot = 2,
    /// Percentage of trading volume - usually used for Equities. Example: 0.005% of notional trading volume. Multiplied by 100,00.
    PercentageOfValue = 3,
    /// Quote ccy of Symbol per 1 lot - will be used for CFDs and futures for commodities, and indices. Example: 15 EUR for 1 contract of DAX.
    QuoteCcyPerLot = 4,
}
///* Enum for specifying stop loss and take profit distances.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SymbolDistanceType {
    SymbolDistanceInPoints = 1,
    SymbolDistanceInPercentage = 2,
}
///* Enum for specifying type of minimum trading commission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MinCommissionType {
    Currency = 1,
    QuoteCurrency = 2,
}
///* Enum for specifying symbol trading mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradingMode {
    Enabled = 0,
    DisabledWithoutPendingsExecution = 1,
    DisabledWithPendingsExecution = 2,
    CloseOnlyMode = 3,
}
///* Enum for specifying SWAP calculation type for symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SwapCalculationType {
    ///Specifies type of SWAP computation as PIPS (0)
    Pips = 0,
    ///Specifies type of SWAP computation as PERCENTAGE (1, annual, in percent)
    Percentage = 1,
}
///* Enum for specifying access right for a trader.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessRights {
    /// Enable all trading.
    FullAccess = 0,
    /// Only closing trading request are enabled.
    CloseOnly = 1,
    /// View only access.
    NoTrading = 2,
    /// No access.
    NoLogin = 3,
}
///* Enum for specifying margin calculation type for an account.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TotalMarginCalculationType {
    Max = 0,
    Sum = 1,
    Net = 2,
}
///* Enum for specifying type of an account.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    /// Allows multiple positions on a trading account for a symbol.
    Hedged = 0,
    /// Only one position per symbol is allowed on a trading account.
    Netted = 1,
    /// Spread betting type account.
    SpreadBetting = 2,
}
///* Position status ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionStatus {
    Open = 1,
    Closed = 2,
    /// Empty position is created for pending order.
    Created = 3,
    Error = 4,
}
///* Trader side ENUM. Used for order, position, deal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeSide {
    Buy = 1,
    Sell = 2,
}
///* Order type ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLossTakeProfit = 4,
    MarketRange = 5,
    StopLimit = 6,
}
///* Order's time in force ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeInForce {
    GoodTillDate = 1,
    GoodTillCancel = 2,
    ImmediateOrCancel = 3,
    FillOrKill = 4,
    MarketOnOpen = 5,
}
///* Order status ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    /// Order request validated and accepted for execution.
    Accepted = 1,
    /// Order is fully filled.
    Filled = 2,
    /// Order is rejected due to validation.
    Rejected = 3,
    /// Order expired. Might be valid for orders with partially filled volume that were expired on LP.
    Expired = 4,
    /// Order is cancelled. Might be valid for orders with partially filled volume that were cancelled by LP.
    Cancelled = 5,
}
///* Stop Order and Stop Lost triggering method ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderTriggerMethod {
    /// Stop Order: buy is triggered by ask, sell by bid; Stop Loss Order: for buy position is triggered by bid and for sell position by ask.
    Trade = 1,
    /// Stop Order: buy is triggered by bid, sell by ask; Stop Loss Order: for buy position is triggered by ask and for sell position by bid.
    Opposite = 2,
    /// The same as TRADE, but trigger is checked after the second consecutive tick.
    DoubleTrade = 3,
    /// The same as OPPOSITE, but trigger is checked after the second consecutive tick.
    DoubleOpposite = 4,
}
///* Execution event type ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    /// Order passed validation.
    OrderAccepted = 2,
    /// Order filled.
    OrderFilled = 3,
    /// Pending order is changed with a new one.
    OrderReplaced = 4,
    /// Order cancelled.
    OrderCancelled = 5,
    /// Order with GTD time in force is expired.
    OrderExpired = 6,
    /// Order is rejected due to validations.
    OrderRejected = 7,
    /// Cancel order request is rejected.
    OrderCancelRejected = 8,
    /// Type related to SWAP execution events.
    Swap = 9,
    /// Type related to event of deposit or withdrawal cash flow operation.
    DepositWithdraw = 10,
    /// Order is partially filled.
    OrderPartialFill = 11,
    /// Type related to event of bonus deposit or bonus withdrawal.
    BonusDepositWithdraw = 12,
}
///* Bonus operation type ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeBonusType {
    BonusDeposit = 0,
    BonusWithdraw = 1,
}
///* Balance operation entity. Covers all cash movement operations related to account, trading, IB operations, mirroring, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeBalanceType {
    /// Cash deposit.
    BalanceDeposit = 0,
    /// Cash withdrawal.
    BalanceWithdraw = 1,
    /// Received mirroring commission.
    BalanceDepositStrategyCommissionInner = 3,
    /// Paid mirroring commission.
    BalanceWithdrawStrategyCommissionInner = 4,
    /// For IB account. Commissions paid by trader.
    BalanceDepositIbCommissions = 5,
    /// For IB account. Withdrawal of commissions shared with broker.
    BalanceWithdrawIbSharedPercentage = 6,
    /// For IB account. Commissions paid by sub-ibs.
    BalanceDepositIbSharedPercentageFromSubIb = 7,
    /// For IB account. Commissions paid by broker.
    BalanceDepositIbSharedPercentageFromBroker = 8,
    /// Deposit rebate for trading volume for period.
    BalanceDepositRebate = 9,
    /// Withdrawal of rebate.
    BalanceWithdrawRebate = 10,
    /// Mirroring commission.
    BalanceDepositStrategyCommissionOuter = 11,
    /// Mirroring commission.
    BalanceWithdrawStrategyCommissionOuter = 12,
    /// For IB account. Share commission with the Broker.
    BalanceWithdrawBonusCompensation = 13,
    /// IB commissions.
    BalanceWithdrawIbSharedPercentageToBroker = 14,
    /// Deposit dividends payments.
    BalanceDepositDividends = 15,
    /// Negative dividend charge for short position.
    BalanceWithdrawDividends = 16,
    /// Charge for guaranteedStopLoss.
    BalanceWithdrawGslCharge = 17,
    /// Charge of rollover fee for Shariah compliant accounts.
    BalanceWithdrawRollover = 18,
    /// Broker's operation to deposit bonus.
    BalanceDepositNonwithdrawableBonus = 19,
    /// Broker's operation to withdrawal bonus.
    BalanceWithdrawNonwithdrawableBonus = 20,
    /// Deposits of negative SWAP.
    BalanceDepositSwap = 21,
    /// SWAP charges.
    BalanceWithdrawSwap = 22,
    /// Mirroring commission.
    BalanceDepositManagementFee = 27,
    /// Mirroring commission. Deprecated since 7.1 in favor of BALANCE_WITHDRAW_COPY_FEE (34).
    BalanceWithdrawManagementFee = 28,
    /// Mirroring commission.
    BalanceDepositPerformanceFee = 29,
    BalanceWithdrawForSubaccount = 30,
    BalanceDepositToSubaccount = 31,
    BalanceWithdrawFromSubaccount = 32,
    BalanceDepositFromSubaccount = 33,
    /// Withdrawal fees to Strategy Provider.
    BalanceWithdrawCopyFee = 34,
    /// Withdraw of inactivity fee from the balance
    BalanceWithdrawInactivityFee = 35,
    BalanceDepositTransfer = 36,
    BalanceWithdrawTransfer = 37,
    BalanceDepositConvertedBonus = 38,
}
///* Deal status ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DealStatus {
    /// Deal filled.
    Filled = 2,
    /// Deal is partially filled.
    PartiallyFilled = 3,
    /// Deal is correct but was rejected by liquidity provider (e.g. no liquidity).
    Rejected = 4,
    /// Deal rejected by server (e.g. no price quotes).
    InternallyRejected = 5,
    /// Deal is rejected by LP due to error (e.g. symbol is unknown).
    Error = 6,
    /// Liquidity provider did not sent response on the deal during specified execution time period.
    Missed = 7,
}
///* Trendbar period ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrendbarPeriod {
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
    M10 = 6,
    M15 = 7,
    M30 = 8,
    H1 = 9,
    H4 = 10,
    H12 = 11,
    D1 = 12,
    W1 = 13,
    Mn1 = 14,
}
///* Price quote type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuoteType {
    Bid = 1,
    Ask = 2,
}
///* Open API application permission in regards to token ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientPermissionScope {
    /// Allows to use only view commends. Trade is prohibited.
    ScopeView = 0,
    /// Allows to use all commands.
    ScopeTrade = 1,
}
///* Type of notification, currently only 3 instances of marginCall are supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
    /// one of three margin calls, they are all similar.
    MarginLevelThreshold1 = 61,
    /// one of three margin calls, they are all similar.
    MarginLevelThreshold2 = 62,
    /// one of three margin calls, they are all similar.
    MarginLevelThreshold3 = 63,
}
///* Error code ENUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    ///Authorization
    ///
    /// When token used for account authorization is expired.
    OaAuthTokenExpired = 1,
    /// When account is not authorized.
    AccountNotAuthorized = 2,
    /// When client tries to authorize after it was already authorized
    AlreadyLoggedIn = 14,
    /// Open API client is not activated or wrong client credentials.
    ChClientAuthFailure = 101,
    /// When a command is sent for not authorized Open API client.
    ChClientNotAuthenticated = 102,
    /// Client is trying to authenticate twice.
    ChClientAlreadyAuthenticated = 103,
    /// Access token is invalid.
    ChAccessTokenInvalid = 104,
    /// Trading service is not available.
    ChServerNotReachable = 105,
    /// Trading account is not found.
    ChCtidTraderAccountNotFound = 106,
    /// Could not find this client id.
    ChOaClientNotFound = 107,
    ///General
    ///
    /// Request frequency is reached.
    RequestFrequencyExceeded = 108,
    /// Server is under maintenance.
    ServerIsUnderMaintenance = 109,
    /// Operations are not allowed for this account.
    ChannelIsBlocked = 110,
    /// Limit of connections is reached for this Open API client.
    ConnectionsLimitExceeded = 67,
    /// Not allowed to increase risk for Positions with Guaranteed Stop Loss.
    WorseGslNotAllowed = 68,
    /// Trading disabled because symbol has holiday.
    SymbolHasHoliday = 69,
    ///Pricing
    ///
    /// When trying to subscribe to depth, trendbars, etc. without spot subscription.
    NotSubscribedToSpots = 112,
    /// When subscription is requested for an active.
    AlreadySubscribed = 113,
    /// Symbol not found.
    SymbolNotFound = 114,
    /// Note: to be merged with SYMBOL_NOT_FOUND.
    UnknownSymbol = 115,
    /// When requested period (from,to) is too large or invalid values are set to from/to.
    IncorrectBoundaries = 35,
    ///Trading
    ///
    /// Trading cannot be done as not quotes are available. Applicable for Book B.
    NoQuotes = 117,
    /// Not enough funds to allocate margin.
    NotEnoughMoney = 118,
    /// Max exposure limit is reached for a {trader, symbol, side}.
    MaxExposureReached = 119,
    /// Position not found.
    PositionNotFound = 120,
    /// Order not found.
    OrderNotFound = 121,
    /// When trying to close a position that it is not open.
    PositionNotOpen = 122,
    /// Position in the state that does not allow to perform an operation.
    PositionLocked = 123,
    /// Trading account reached its limit for max number of open positions and orders.
    TooManyPositions = 124,
    /// Invalid volume.
    TradingBadVolume = 125,
    /// Invalid stop price.
    TradingBadStops = 126,
    /// Invalid price (e.g. negative).
    TradingBadPrices = 127,
    /// Invalid stake volume (e.g. negative).
    TradingBadStake = 128,
    /// Invalid protection prices.
    ProtectionIsTooCloseToMarket = 129,
    /// Invalid expiration.
    TradingBadExpirationDate = 130,
    /// Unable to apply changes as position has an order under execution.
    PendingExecution = 131,
    /// Trading is blocked for the symbol.
    TradingDisabled = 132,
    /// Trading account is in read only mode.
    TradingNotAllowed = 133,
    /// Unable to cancel order.
    UnableToCancelOrder = 134,
    /// Unable to amend order.
    UnableToAmendOrder = 135,
    /// Short selling is not allowed.
    ShortSellingNotAllowed = 136,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LimitedRiskMarginCalculationStrategy {
    AccordingToLeverage = 0,
    AccordingToGsl = 1,
    AccordingToGslAndLeverage = 2,
}
///* Request for the authorizing an application to work with the cTrader platform Proxies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationAuthReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaApplicationAuthReq"
    )]
    pub payload_type: std::option::Option<i32>,
    ///The unique Client ID provided during the registration.
    #[prost(string, required, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    ///The unique Client Secret provided during the registration.
    #[prost(string, required, tag = "3")]
    pub client_secret: ::prost::alloc::string::String,
}
///* Response to the ApplicationAuthReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationAuthRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaApplicationAuthRes"
    )]
    pub payload_type: std::option::Option<i32>,
}
///* Request for the authorizing trading account session. Requires established authorized connection with the client application using ApplicationAuthReq.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAuthReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAccountAuthReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Access Token issued for providing access to the Trader's Account.
    #[prost(string, required, tag = "3")]
    pub access_token: ::prost::alloc::string::String,
}
///* Response to the ApplicationAuthRes request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAuthRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAccountAuthRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Generic response when an ERROR occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaErrorRes"
    )]
    pub payload_type: std::option::Option<i32>,
    ///The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, optional, tag = "2")]
    pub ctid_trader_account_id: std::option::Option<i64>,
    /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, required, tag = "3")]
    pub error_code: ::prost::alloc::string::String,
    /// The error description.
    #[prost(string, optional, tag = "4")]
    pub description: std::option::Option<::prost::alloc::string::String>,
    /// The timestamp in seconds when the current maintenance session will be ended.
    #[prost(int64, optional, tag = "5")]
    pub maintenance_end_timestamp: std::option::Option<i64>,
}
///* Event that is sent when the connection with the client application is cancelled by the server. All the sessions for the traders' accounts will be terminated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientDisconnectEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaClientDisconnectEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The disconnection reason explained. For example: The application access was blocked by cTrader Administrator.
    #[prost(string, optional, tag = "2")]
    pub reason: std::option::Option<::prost::alloc::string::String>,
}
///* Event that is sent when a session to a specific trader's account is terminated by the server but the existing connections with the other trader's accounts are maintained.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountsTokenInvalidatedEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAccountsTokenInvalidatedEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub ctid_trader_account_ids: std::prelude::rust_2015::Vec<i64>,
    /// The disconnection reason explained. For example: Access Token is expired or recalled.
    #[prost(string, optional, tag = "3")]
    pub reason: std::option::Option<::prost::alloc::string::String>,
}
///* Request for getting the proxy version. Can be used to check the current version of the Open API scheme.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaVersionReq"
    )]
    pub payload_type: std::option::Option<i32>,
}
///* Response to the VersionReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaVersionRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The current version of the server application.
    #[prost(string, required, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
///* Request for sending a new trading order. Allowed only if the accessToken has the "trade" permissions for the trading account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOrderReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaNewOrderReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique identifier of a symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// The type of an order - MARKET, LIMIT, STOP, MARKET_RANGE, STOP_LIMIT.
    #[prost(enumeration = "OrderType", required, tag = "4")]
    pub order_type: i32,
    /// The trade direction - BUY or SELL.
    #[prost(enumeration = "TradeSide", required, tag = "5")]
    pub trade_side: i32,
    /// The volume represented in 0.01 of a unit (e.g. US$ 10.00 = 1000).
    #[prost(int64, required, tag = "6")]
    pub volume: i64,
    /// The limit price, can be specified for the LIMIT order only.
    #[prost(double, optional, tag = "7")]
    pub limit_price: std::option::Option<f64>,
    /// Stop Price, can be specified for the STOP and the STOP_LIMIT orders only.
    #[prost(double, optional, tag = "8")]
    pub stop_price: std::option::Option<f64>,
    /// The specific order execution or expiration instruction - GOOD_TILL_DATE, GOOD_TILL_CANCEL, IMMEDIATE_OR_CANCEL, FILL_OR_KILL, MARKET_ON_OPEN.
    #[prost(
        enumeration = "TimeInForce",
        optional,
        tag = "9",
        default = "GoodTillCancel"
    )]
    pub time_in_force: std::option::Option<i32>,
    /// The exact Order expiration time. Should be set for the Good Till Date orders.
    #[prost(int64, optional, tag = "10")]
    pub expiration_timestamp: std::option::Option<i64>,
    /// The absolute Stop Loss price (1.23456 for example). Not supported for the MARKER orders.
    #[prost(double, optional, tag = "11")]
    pub stop_loss: std::option::Option<f64>,
    /// The absolute Take Profit price (1.23456 for example). Unsupported for the MARKER orders.
    #[prost(double, optional, tag = "12")]
    pub take_profit: std::option::Option<f64>,
    /// User-specified comment. MaxLength = 512.
    #[prost(string, optional, tag = "13")]
    pub comment: std::option::Option<::prost::alloc::string::String>,
    /// Base price to calculate relative slippage price for MARKET_RANGE order.
    #[prost(double, optional, tag = "14")]
    pub base_slippage_price: std::option::Option<f64>,
    /// Slippage distance for MARKET_RANGE and STOP_LIMIT order.
    #[prost(int32, optional, tag = "15")]
    pub slippage_in_points: std::option::Option<i32>,
    /// User-specified label. MaxLength = 100.
    #[prost(string, optional, tag = "16")]
    pub label: std::option::Option<::prost::alloc::string::String>,
    /// Reference to the existing position if the Order is intended to modify it.
    #[prost(int64, optional, tag = "17")]
    pub position_id: std::option::Option<i64>,
    /// Optional user-specific clientOrderId (similar to FIX ClOrderID). MaxLength = 50.
    #[prost(string, optional, tag = "18")]
    pub client_order_id: std::option::Option<::prost::alloc::string::String>,
    /// Relative Stop Loss that can be specified instead of the absolute as one. Specified in 1/100000 of unit of a price. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
    #[prost(int64, optional, tag = "19")]
    pub relative_stop_loss: std::option::Option<i64>,
    /// Relative Take Profit that can be specified instead of the absolute one. Specified in 1/100000 of unit of a price. For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
    #[prost(int64, optional, tag = "20")]
    pub relative_take_profit: std::option::Option<i64>,
    /// If TRUE then stopLoss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
    #[prost(bool, optional, tag = "21")]
    pub guaranteed_stop_loss: std::option::Option<bool>,
    /// If TRUE then the Stop Loss is Trailing.
    #[prost(bool, optional, tag = "22")]
    pub trailing_stop_loss: std::option::Option<bool>,
    /// Trigger method for the STOP or the STOP_LIMIT pending order.
    #[prost(
        enumeration = "OrderTriggerMethod",
        optional,
        tag = "23",
        default = "Trade"
    )]
    pub stop_trigger_method: std::option::Option<i32>,
}
///* Event that is sent following the successful order acceptance or execution by the server. Acts as response to the NewOrderReq, CancelOrderReq, AmendOrderReq, AmendPositionSLTPReq, ClosePositionReq requests. Also, the event is sent when a Deposit/Withdrawal took place.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaExecutionEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Type of the order operation. For example: ACCEPTED, FILLED, etc.
    #[prost(enumeration = "ExecutionType", required, tag = "3")]
    pub execution_type: i32,
    /// Reference to the position linked with the execution
    #[prost(message, optional, tag = "4")]
    pub position: std::option::Option<Position>,
    /// Reference to the initial order.
    #[prost(message, optional, tag = "5")]
    pub order: std::option::Option<Order>,
    /// Reference to the deal (execution).
    #[prost(message, optional, tag = "6")]
    pub deal: std::option::Option<Deal>,
    /// Reference to the Bonus Deposit or Withdrawal operation.
    #[prost(message, optional, tag = "7")]
    pub bonus_deposit_withdraw: std::option::Option<BonusDepositWithdraw>,
    /// Reference to the Deposit or Withdrawal operation.
    #[prost(message, optional, tag = "8")]
    pub deposit_withdraw: std::option::Option<DepositWithdraw>,
    ///The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, optional, tag = "9")]
    pub error_code: std::option::Option<::prost::alloc::string::String>,
    /// If TRUE then the event generated by the server logic instead of the trader's request. (e.g. stop-out).
    #[prost(bool, optional, tag = "10")]
    pub is_server_event: std::option::Option<bool>,
}
///* Request for cancelling existing pending order. Allowed only if the accessToken has "trade" permissions for the trading account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaCancelOrderReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the order.
    #[prost(int64, required, tag = "3")]
    pub order_id: i64,
}
///* Request for amending the existing pending order. Allowed only if the Access Token has "trade" permissions for the trading account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmendOrderReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAmendOrderReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the order.
    #[prost(int64, required, tag = "3")]
    pub order_id: i64,
    /// Volume, represented in 0.01 of a unit (e.g. cents).
    #[prost(int64, optional, tag = "4")]
    pub volume: std::option::Option<i64>,
    /// The Limit Price, can be specified for the LIMIT order only.
    #[prost(double, optional, tag = "5")]
    pub limit_price: std::option::Option<f64>,
    /// The Stop Price, can be specified for the STOP and the STOP_LIMIT orders.
    #[prost(double, optional, tag = "6")]
    pub stop_price: std::option::Option<f64>,
    /// The exact Order expiration time. Should be set for the Good Till Date orders.
    #[prost(int64, optional, tag = "7")]
    pub expiration_timestamp: std::option::Option<i64>,
    /// The absolute Stop Loss price (e.g. 1.23456). Not supported for the MARKER orders.
    #[prost(double, optional, tag = "8")]
    pub stop_loss: std::option::Option<f64>,
    /// The absolute Take Profit price (e.g. 1.23456). Not supported for the MARKER orders.
    #[prost(double, optional, tag = "9")]
    pub take_profit: std::option::Option<f64>,
    /// Slippage distance for the MARKET_RANGE and the STOP_LIMIT orders.
    #[prost(int32, optional, tag = "10")]
    pub slippage_in_points: std::option::Option<i32>,
    /// The relative Stop Loss can be specified instead of the absolute one. Specified in 1/100000 of a unit of price. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
    #[prost(int64, optional, tag = "11")]
    pub relative_stop_loss: std::option::Option<i64>,
    /// The relative Take Profit can be specified instead of the absolute one. Specified in 1/100000 of a unit of price. For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
    #[prost(int64, optional, tag = "12")]
    pub relative_take_profit: std::option::Option<i64>,
    /// If TRUE then the Stop Loss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
    #[prost(bool, optional, tag = "13")]
    pub guaranteed_stop_loss: std::option::Option<bool>,
    /// If TRUE then the Trailing Stop Loss is applied.
    #[prost(bool, optional, tag = "14")]
    pub trailing_stop_loss: std::option::Option<bool>,
    /// Trigger method for the STOP or the STOP_LIMIT pending order.
    #[prost(
        enumeration = "OrderTriggerMethod",
        optional,
        tag = "15",
        default = "Trade"
    )]
    pub stop_trigger_method: std::option::Option<i32>,
}
///* Request for amending StopLoss and TakeProfit of existing position. Allowed only if the accessToken has "trade" permissions for the trading account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmendPositionSltpReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAmendPositionSltpReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position to amend.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// Absolute Stop Loss price (1.23456 for example).
    #[prost(double, optional, tag = "4")]
    pub stop_loss: std::option::Option<f64>,
    /// Absolute Take Profit price (1.26543 for example).
    #[prost(double, optional, tag = "5")]
    pub take_profit: std::option::Option<f64>,
    ///If TRUE then the Stop Loss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
    #[prost(bool, optional, tag = "7")]
    pub guaranteed_stop_loss: std::option::Option<bool>,
    ///If TRUE then the Trailing Stop Loss is applied.
    #[prost(bool, optional, tag = "8")]
    pub trailing_stop_loss: std::option::Option<bool>,
    /// The Stop trigger method for the Stop Loss/Take Profit order.
    #[prost(
        enumeration = "OrderTriggerMethod",
        optional,
        tag = "9",
        default = "Trade"
    )]
    pub stop_loss_trigger_method: std::option::Option<i32>,
}
///* Request for closing or partially closing of an existing position. Allowed only if the accessToken has "trade" permissions for the trading account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePositionReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaClosePositionReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position to close.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// Volume to close, represented in 0.01 of a unit (e.g. cents).
    #[prost(int64, required, tag = "4")]
    pub volume: i64,
}
///* Event that is sent when the level of the Trailing Stop Loss is changed due to the price level changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrailingSlChangedEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaTrailingSlChangedEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// The unique ID of the order.
    #[prost(int64, required, tag = "4")]
    pub order_id: i64,
    /// New value of the Stop Loss price.
    #[prost(double, required, tag = "5")]
    pub stop_price: f64,
    /// The exact UTC time when the Stop Loss was updated.
    #[prost(int64, required, tag = "6")]
    pub utc_last_update_timestamp: i64,
}
///* Request for the list of assets available for a trader's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAssetListReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Response to the AssetListReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAssetListRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of assets.
    #[prost(message, repeated, tag = "3")]
    pub asset: std::prelude::rust_2015::Vec<Asset>,
}
///* Request for a list of symbols available for a trading account. Symbol entries are returned with the limited set of fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolsListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolsListReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(bool, optional, tag = "3", default = "false")]
    pub include_archived_symbols: std::option::Option<bool>,
}
///* Response to the SymbolsListReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolsListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolsListRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of symbols.
    #[prost(message, repeated, tag = "3")]
    pub symbol: std::prelude::rust_2015::Vec<LightSymbol>,
    /// The list of archived symbols.
    #[prost(message, repeated, tag = "4")]
    pub archived_symbol: std::prelude::rust_2015::Vec<ArchivedSymbol>,
}
///* Request for getting a full symbol entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolByIdReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolByIdReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: std::prelude::rust_2015::Vec<i64>,
}
///* Response to the SymbolByIdReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolByIdRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolByIdRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Symbol entity with the full set of fields.
    #[prost(message, repeated, tag = "3")]
    pub symbol: std::prelude::rust_2015::Vec<Symbol>,
    /// Archived symbols.
    #[prost(message, repeated, tag = "4")]
    pub archived_symbol: std::prelude::rust_2015::Vec<ArchivedSymbol>,
}
///* Request for getting a conversion chain between two assets that consists of several symbols. Use when no direct quote is available
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolsForConversionReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolsForConversionReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The ID of the firs asset in the conversation chain. e.g.: for EUR/USD the firstAssetId is EUR ID and lastAssetId is USD ID.
    #[prost(int64, required, tag = "3")]
    pub first_asset_id: i64,
    /// The ID of the last asset in the conversation chain. e.g.: for EUR/USD the firstAssetId is EUR ID and lastAssetId is USD ID.
    #[prost(int64, required, tag = "4")]
    pub last_asset_id: i64,
}
///* Response to the SymbolsForConversionReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolsForConversionRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolsForConversionRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Conversion chain of the symbols (e.g. EUR/USD, USD/JPY, GBP/JPY -> EUR/GBP).
    #[prost(message, repeated, tag = "3")]
    pub symbol: std::prelude::rust_2015::Vec<LightSymbol>,
}
///* Event that is sent when the symbol is changed on the Server side.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolChangedEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolChangedEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: std::prelude::rust_2015::Vec<i64>,
}
///* Request for a list of asset classes available for the trader's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetClassListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAssetClassListReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Response to the AssetListReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetClassListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAssetClassListRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// List of the asset classes.
    #[prost(message, repeated, tag = "3")]
    pub asset_class: std::prelude::rust_2015::Vec<AssetClass>,
}
///* Request for getting data of Trader's Account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaTraderReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Response to the TraderReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaTraderRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Trader account information.
    #[prost(message, required, tag = "3")]
    pub trader: Trader,
}
///* Event that is sent when a Trader is updated on Server side.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderUpdatedEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaTraderUpdateEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Trader account information.
    #[prost(message, required, tag = "3")]
    pub trader: Trader,
}
///* Request for getting Trader's current open positions and pending orders data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaReconcileReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* The response to the ReconcileReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaReconcileRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of trader's account open positions.
    #[prost(message, repeated, tag = "3")]
    pub position: std::prelude::rust_2015::Vec<Position>,
    /// The list of trader's account pending orders.
    #[prost(message, repeated, tag = "4")]
    pub order: std::prelude::rust_2015::Vec<Order>,
}
///* Event that is sent when errors occur during the order requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderErrorEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaOrderErrorEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    ///Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "5")]
    pub ctid_trader_account_id: i64,
    /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, required, tag = "2")]
    pub error_code: ::prost::alloc::string::String,
    /// The unique ID of the order.
    #[prost(int64, optional, tag = "3")]
    pub order_id: std::option::Option<i64>,
    /// The unique ID of the position.
    #[prost(int64, optional, tag = "6")]
    pub position_id: std::option::Option<i64>,
    /// The error description.
    #[prost(string, optional, tag = "7")]
    pub description: std::option::Option<::prost::alloc::string::String>,
}
///* Request for getting Trader's deals historical data (execution details).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DealListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaDealListReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The UNIX time from which the search starts >=0 (1-1-1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The UNIX time where to stop searching <= 2147483646000 (19-1-2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
    /// The maximum number of the deals to return.
    #[prost(int32, optional, tag = "5")]
    pub max_rows: std::option::Option<i32>,
}
///* The response to the DealListRes request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DealListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaDealListRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of the deals.
    #[prost(message, repeated, tag = "3")]
    pub deal: std::prelude::rust_2015::Vec<Deal>,
    /// If TRUE then the response will provide more than 10000 deals.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
///* Request for getting the margin estimate. Can be used before sending a new order request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpectedMarginReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaExpectedMarginReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// Volume represented in 0.01 of a unit (e.g. cents).
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub volume: std::prelude::rust_2015::Vec<i64>,
}
///*The response to the ExpectedMarginReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpectedMarginRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaExpectedMarginRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The buy and sell margin estimate.
    #[prost(message, repeated, tag = "3")]
    pub margin: std::prelude::rust_2015::Vec<ExpectedMargin>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects margin.buyMargin, margin.sellMargin.
    #[prost(uint32, optional, tag = "4")]
    pub money_digits: std::option::Option<u32>,
}
///* Event that is sent when the margin allocated to a specific position is changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginChangedEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginChangedEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position.
    #[prost(uint64, required, tag = "3")]
    pub position_id: u64,
    /// The new value of the margin used.
    #[prost(uint64, required, tag = "4")]
    pub used_margin: u64,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects usedMargin.
    #[prost(uint32, optional, tag = "5")]
    pub money_digits: std::option::Option<u32>,
}
///* Request for getting Trader's historical data of deposits and withdrawals.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashFlowHistoryListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaCashFlowHistoryListReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The UNIX time from which the search starts >=0 (1-1-1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The UNIX time where to stop searching <= 2147483646000 (19-1-2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
}
///* Response to the CashFlowHistoryListReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashFlowHistoryListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaCashFlowHistoryListRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of deposit and withdrawal operations.
    #[prost(message, repeated, tag = "3")]
    pub deposit_withdraw: std::prelude::rust_2015::Vec<DepositWithdraw>,
}
///* Request for getting the list of granted trader's account for the access token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountListByAccessTokenReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetAccountsByAccessTokenReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The Access Token issued for providing access to the Trader's Account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
///* Response to the GetAccountListByAccessTokenReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountListByAccessTokenRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetAccountsByAccessTokenRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The Access Token issued for providing access to the Trader's Account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    /// SCOPE_VIEW, SCOPE_TRADE.
    #[prost(enumeration = "ClientPermissionScope", optional, tag = "3")]
    pub permission_scope: std::option::Option<i32>,
    /// The list of the accounts.
    #[prost(message, repeated, tag = "4")]
    pub ctid_trader_account: std::prelude::rust_2015::Vec<CtidTraderAccount>,
}
///* Request to refresh the access token using refresh token of granted trader's account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTokenReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaRefreshTokenReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The Refresh Token issued for updating Access Token.
    #[prost(string, required, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
}
///* Response to the RefreshTokenReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTokenRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaRefreshTokenRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The Access Token issued for providing access to the Trader's Account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    /// bearer
    #[prost(string, required, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// Access Token expiration in seconds
    #[prost(int64, required, tag = "4")]
    pub expires_in: i64,
    #[prost(string, required, tag = "5")]
    pub refresh_token: ::prost::alloc::string::String,
}
//+------------------------------------------------------------------+
//|                              Quotes                              |
//+------------------------------------------------------------------+

///* Request for subscribing on spot events of the specified symbol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeSpotsReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSubscribeSpotsReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: std::prelude::rust_2015::Vec<i64>,
}
///* Response to the SubscribeSpotsReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeSpotsRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSubscribeSpotsRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Request for unsubscribing from the spot events of the specified symbol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeSpotsReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaUnsubscribeSpotsReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: std::prelude::rust_2015::Vec<i64>,
}
///* Response to the SubscribeSpotsRes request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeSpotsRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaUnsubscribeSpotsRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Event that is sent when a new spot event is generated on the server side. Requires subscription on the spot events, see SubscribeSpotsReq.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSpotEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// Bid price. Specified in 1/100_000 of unit of a price. (e.g. 1.23 -> 123_000)
    #[prost(uint64, optional, tag = "4")]
    pub bid: std::option::Option<u64>,
    /// Ask price. Specified in 1/100_000 of unit of a price.
    #[prost(uint64, optional, tag = "5")]
    pub ask: std::option::Option<u64>,
    /// Returns live trend bar. Requires subscription on the trend bars.
    #[prost(message, repeated, tag = "6")]
    pub trendbar: std::prelude::rust_2015::Vec<Trendbar>,
    ///Last session close. Specified in 1/100_000 of unit of a price.
    #[prost(uint64, optional, tag = "7")]
    pub session_close: std::option::Option<u64>,
}
///* Request for subscribing for live trend bars. Requires subscription on the spot events, see SubscribeSpotsReq.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLiveTrendbarReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSubscribeLiveTrendbarReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "TrendbarPeriod", required, tag = "3")]
    pub period: i32,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "4")]
    pub symbol_id: i64,
}
///* Response to the SubscribeLiveTrendbarReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLiveTrendbarRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSubscribeLiveTrendbarRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Request for unsubscribing from the live trend bars.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeLiveTrendbarReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaUnsubscribeLiveTrendbarReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "TrendbarPeriod", required, tag = "3")]
    pub period: i32,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "4")]
    pub symbol_id: i64,
}
///* Response to the SubscribeLiveTrendbarReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeLiveTrendbarRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaUnsubscribeLiveTrendbarRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Request for getting historical trend bars for the symbol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrendbarsReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetTrendbarsReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The exact time of starting the search in milliseconds. Must be bigger or equal to zero (1-1-1970). Validation: toTimestamp - fromTimestamp <= X, where X depends on series period: M1, M2, M3, M4, M5: 302400000 (5 weeks); M10, M15, M30, H1: 21168000000 (35 weeks), H4, H12, D1: 31622400000 (1 year); W1, MN1: 158112000000 (5 years).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The exact time of finishing the search in milliseconds. Smaller or equal to 2147483646000 (19-1-2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "TrendbarPeriod", required, tag = "5")]
    pub period: i32,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "6")]
    pub symbol_id: i64,
}
///* Response to the GetTrendbarsReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrendbarsRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetTrendbarsRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "TrendbarPeriod", required, tag = "3")]
    pub period: i32,
    /// Equals to toTimestamp from the request.
    #[prost(int64, required, tag = "4")]
    pub timestamp: i64,
    /// The list of trend bars.
    #[prost(message, repeated, tag = "5")]
    pub trendbar: std::prelude::rust_2015::Vec<Trendbar>,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, optional, tag = "6")]
    pub symbol_id: std::option::Option<i64>,
}
///* Request for getting historical tick data for the symbol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTickDataReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetTickdataReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// Bid/Ask (1/2).
    #[prost(enumeration = "QuoteType", required, tag = "4")]
    pub r#type: i32,
    /// The exact time of starting the search in milliseconds. Must be bigger of equal to zero (1-1-1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "5")]
    pub from_timestamp: i64,
    /// The exact time of finishing the search in milliseconds <= 2147483646000 (19-1-2038).
    #[prost(int64, required, tag = "6")]
    pub to_timestamp: i64,
}
///* Response to the GetTickDataReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTickDataRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetTickdataRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of ticks.
    #[prost(message, repeated, tag = "3")]
    pub tick_data: std::prelude::rust_2015::Vec<TickData>,
    /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
//+------------------------------------------------------------------+
//|                      End quotes section                          |
//+------------------------------------------------------------------+

///* Request for getting details of Trader's profile. Limited due to GDRP requirements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCtidProfileByTokenReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetCtidProfileByTokenReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The Access Token issued for providing access to the Trader's Account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
///* Response to the GetCtidProfileByTokenReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCtidProfileByTokenRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaGetCtidProfileByTokenRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Trader's profile.
    #[prost(message, required, tag = "2")]
    pub profile: CtidProfile,
}
///* Event that is sent when the structure of depth of market is changed. Requires subscription on the depth of markets for the symbol, see SubscribeDepthQuotesReq.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepthEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaDepthEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(uint64, required, tag = "3")]
    pub symbol_id: u64,
    /// The list of changes in the depth of market quotes.
    #[prost(message, repeated, tag = "4")]
    pub new_quotes: std::prelude::rust_2015::Vec<DepthQuote>,
    /// The list of quotes to delete.
    #[prost(uint64, repeated, tag = "5")]
    pub deleted_quotes: std::prelude::rust_2015::Vec<u64>,
}
///* Request for subscribing on depth of market of the specified symbol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDepthQuotesReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSubscribeDepthQuotesReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: std::prelude::rust_2015::Vec<i64>,
}
///* Response to the SubscribeDepthQuotesReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDepthQuotesRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSubscribeDepthQuotesRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Request for unsubscribing from the depth of market of the specified symbol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeDepthQuotesReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaUnsubscribeDepthQuotesReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: std::prelude::rust_2015::Vec<i64>,
}
///* Response to the UnsubscribeDepthQuotesReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeDepthQuotesRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaUnsubscribeDepthQuotesRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Request for a list of symbol categories available for a trading account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolCategoryListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolCategoryReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Response to the ProtoSymbolCategoryListReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolCategoryListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaSymbolCategoryRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of symbol categories.
    #[prost(message, repeated, tag = "3")]
    pub symbol_category: std::prelude::rust_2015::Vec<SymbolCategory>,
}
///* Request for logout of  trading account session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLogoutReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAccountLogoutReq"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Response to the TraderLogoutReq request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLogoutRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAccountLogoutRes"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Event that is sent when the established session for an account is dropped on the server side.
///A new session must be authorized for the account.  
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDisconnectEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaAccountDisconnectEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Request for a list of existing margin call thresholds configured for a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCallListReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginCallListReq"
    )]
    pub payload_type: std::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
///* Response with a list of existing user Margin Calls, usually contains 3 items.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCallListRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginCallListRes"
    )]
    pub payload_type: std::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub margin_call: std::prelude::rust_2015::Vec<MarginCall>,
}
///* Request to modify marginLevelThreshold of specified marginCallType for ctidTraderAccountId.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCallUpdateReq {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginCallUpdateReq"
    )]
    pub payload_type: std::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub margin_call: MarginCall,
}
///* If this response received, it means that margin call was successfully updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCallUpdateRes {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginCallUpdateRes"
    )]
    pub payload_type: std::option::Option<i32>,
}
///* Event that is sent when a Margin Call threshold configuration is updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCallUpdateEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginCallUpdateEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub margin_call: MarginCall,
}
///* Event that is sent when account margin level reaches target marginLevelThreshold. Event is sent no more than once every 10 minutes to avoid spamming.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginCallTriggerEvent {
    #[prost(
        enumeration = "PayloadType",
        optional,
        tag = "1",
        default = "OaMarginCallTriggerEvent"
    )]
    pub payload_type: std::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub margin_call: MarginCall,
}

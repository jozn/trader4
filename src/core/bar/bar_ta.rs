use prost::Message;
use serde::{Deserialize, Serialize};

use super::*;
use crate::base::OHLCV;
use crate::collector::row_data::BTickData;
use crate::{helper, ta};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAMethods {
    pub atr: ta::ATR,
    pub ma1: ta::EMA,
    pub dc_snake: ta::DCSnake,
    pub ma_mom: ta::MAMom,
    pub bb: ta::BB,
    pub sb: ta::SB,
    pub gb: ta::GB,
    pub rpi: ta::RPI,
    pub rpc: ta::RPC,
    pub dc: ta::DC,
    pub macd: ta::MACD,
    pub dmi: ta::DMI,
    pub dmmd: ta::DMMD,
    pub stoch: ta::Stoch,
    pub trend: ta::MATrend,
    pub vel: ta::Vel,
    pub vel_mom: ta::VelMom,
    pub rdc: ta::RDC,
    pub td: ta::TD,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BarTA {
    pub atr: f64,
    pub ma1: f64,
    pub ma_mom: f64,
    pub dc_snake: ta::DCSnakeRes,
    pub bb: ta::BBOut,
    pub sb: ta::SBOut,
    pub gb: ta::GBOut,
    pub rpi: ta::RPIRes,
    pub rpc: ta::RPCRes,
    pub dc: ta::DCRes,
    pub macd: ta::MACDOutput,
    pub dmi: ta::DMIOutput,
    pub dmmd: ta::DMMDOutput,
    pub stoch: ta::StochRes,
    pub trend: ta::MATrendOut,
    pub vel: ta::VelRes,
    pub vel_mom: ta::VelMomRes,
    pub rdc: ta::RDCRes,
    pub td: ta::TDOut,
}

impl TAMethods {
    pub fn new(cfg: &BarConfig) -> Self {
        Self {
            atr: ta::ATR::new(14).unwrap(),
            ma1: ta::EMA::new(25).unwrap(),
            dc_snake: ta::DCSnake::new(20).unwrap(),
            ma_mom: ta::MAMom::new(25, 3).unwrap(),
            bb: ta::BB::new(20, 1.5).unwrap(),
            sb: ta::SB::new(20, 2.0).unwrap(),
            gb: ta::GB::new(20, 2.).unwrap(),
            rpi: ta::RPI::new(10, 5, 0.3).unwrap(),
            rpc: ta::RPC::new(10, 0.5).unwrap(),
            dc: ta::DC::new(12).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            dmi: ta::DMI::new(14, 14).unwrap(),
            dmmd: ta::DMMD::new(14, 14).unwrap(),
            stoch: ta::Stoch::new(14, 3, 5).unwrap(),
            trend: ta::MATrend::new(10).unwrap(),
            vel: ta::Vel::new(15).unwrap(),
            vel_mom: ta::VelMom::new(25, 3).unwrap(),
            rdc: ta::RDC::new(20, 60).unwrap(),
            td: ta::TD::new(14, 14).unwrap(),
        }
    }
}

pub fn cal_indicators(tam: &mut TAMethods, bar: &Bar) -> BarTA {
    let price = bar.hlc3();
    BarTA {
        atr: tam.atr.next(&bar),
        ma1: tam.ma1.next(price),
        dc_snake: tam.dc_snake.next(&bar),
        ma_mom: tam.ma_mom.next(price),
        bb: tam.bb.next(&bar),
        sb: tam.sb.next(&bar),
        gb: tam.gb.next(&bar),
        rpi: tam.rpi.next(&bar),
        rpc: tam.rpc.next(&bar),
        dc: tam.dc.next(&bar),
        macd: tam.macd.next(bar.close),
        dmi: tam.dmi.next(&bar),
        dmmd: tam.dmmd.next(&bar),
        stoch: tam.stoch.next(&bar),
        trend: tam.trend.next(&bar),
        vel: tam.vel.next_ohlc(&bar),
        vel_mom: tam.vel_mom.next(price),
        rdc: tam.rdc.next(&bar),
        td: tam.td.next(&bar),
    }
}

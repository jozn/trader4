// Licence: A modified version of yata library.
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

// use crate::core::{Error, ValueType, OHLCV};
use super::ohlcv::OHLCV;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, PartialOrd)]
pub enum Source {
    Close,
    Open,
    High,
    Low,
    HL2, // (*High*+*Low*)/2 part of a candle
    TP,  // [Typical price](https://en.wikipedia.org/wiki/Typical_price) of a candle
    Volume,
    VolumedPrice,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialOrd)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Candle {
    pub fn from<T: OHLCV + ?Sized>(src: &T) -> Self {
        Self {
            open: src.open(),
            high: src.high(),
            low: src.low(),
            close: src.close(),
            volume: src.volume(),
        }
    }
}

pub type Candlestick = Candle;

impl OHLCV for Candle {
    #[inline]
    fn open(&self) -> f64 {
        self.open
    }

    #[inline]
    fn high(&self) -> f64 {
        self.high
    }

    #[inline]
    fn low(&self) -> f64 {
        self.low
    }

    #[inline]
    fn close(&self) -> f64 {
        self.close
    }

    #[inline]
    fn volume(&self) -> f64 {
        self.volume
    }
}

impl From<&dyn OHLCV> for Candle {
    fn from(src: &dyn OHLCV) -> Self {
        Self::from(src)
    }
}

impl From<(f64, f64, f64, f64)> for Candle {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
            volume: f64::NAN,
        }
    }
}

impl From<(f64, f64, f64, f64, f64)> for Candle {
    fn from(value: (f64, f64, f64, f64, f64)) -> Self {
        Self {
            open: value.0,
            high: value.1,
            low: value.2,
            close: value.3,
            volume: value.4,
        }
    }
}

impl PartialEq for Candle {
    fn eq(&self, other: &Self) -> bool {
        self.open.to_bits() == other.open.to_bits()
            && self.high.to_bits() == other.high.to_bits()
            && self.low.to_bits() == other.low.to_bits()
            && self.close.to_bits() == other.close.to_bits()
            && self.volume.to_bits() == other.volume.to_bits()
    }
}

impl Eq for Candle {}

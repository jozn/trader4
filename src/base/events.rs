use serde::{Deserialize, Serialize};

// Used for signal crossing over some indicators. Bull and Bear range from 1..=100. Higher value
//  shows stronger strength.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SimpleCrossEvent {
    Bull(u8),
    None,
    Bear(u8),
}

impl SimpleCrossEvent {
    pub const BULL_FULL: Self = Self::Bull(100);
    pub const BEAR_FULL: Self = Self::Bear(100);

    pub fn conv(value: i32) -> Self {
        match value {
            0 => Self::None,
            v => {
                if v > 0 {
                    Self::Bull(value as u8)
                } else {
                    Self::Bear(value as u8)
                }
            }
        }
    }

    pub fn conv_bull(value: bool) -> Self {
        match value {
            true => Self::BULL_FULL,
            false => Self::None,
        }
    }

    pub fn conv_bear(value: bool) -> Self {
        match value {
            true => Self::BEAR_FULL,
            false => Self::None,
        }
    }
}

impl Default for SimpleCrossEvent {
    fn default() -> Self {
        Self::None
    }
}

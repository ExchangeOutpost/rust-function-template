use rust_decimal::prelude::*;
use serde::{Deserialize, Deserializer};

/// Represents a single candlestick in financial data, typically used in trading charts.
#[allow(dead_code)]
pub struct Candle<T> {
    /// The timestamp of the candlestick, usually in seconds since the Unix epoch.
    pub timestamp: i64,
    /// The opening price of the asset during the candlestick's time period.
    pub open: T,
    /// The highest price of the asset during the candlestick's time period.
    pub high: T,
    /// The lowest price of the asset during the candlestick's time period.
    pub low: T,
    /// The closing price of the asset at the end of the candlestick's time period.
    pub close: T,
    /// The trading volume of the asset during the candlestick's time period.
    pub volume: T,
}
// Custom deserialization from an array
impl<'de, T> Deserialize<'de> for Candle<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: (i64, T, T, T, T, T) = Deserialize::deserialize(deserializer)?;
        Ok(Candle {
            timestamp: arr.0,
            open: arr.1,
            high: arr.2,
            low: arr.3,
            close: arr.4,
            volume: arr.5,
        })
    }
}

impl Candle<f64> {
    /// Convert candle to a Decimal representation
    pub fn to_decimal(&self, precision: i32) -> Candle<Decimal> {
        Candle {
            timestamp: self.timestamp,
            open: Decimal::from_f64(self.open).unwrap_or(Decimal::ZERO).round_dp(precision as u32),
            high: Decimal::from_f64(self.high).unwrap_or(Decimal::ZERO).round_dp(precision as u32),
            low: Decimal::from_f64(self.low).unwrap_or(Decimal::ZERO).round_dp(precision as u32),
            close: Decimal::from_f64(self.close).unwrap_or(Decimal::ZERO).round_dp(precision as u32),
            volume: Decimal::from_f64(self.volume).unwrap_or(Decimal::ZERO).round_dp(precision as u32),
        }
    }
}

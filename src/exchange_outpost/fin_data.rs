use crate::exchange_outpost::Candle;
use extism_pdk::FromBytesOwned;
use extism_pdk::*;
use rust_decimal::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TickersData {
    pub symbol: String,
    pub exchange: String,
    pub candles: Vec<Candle<f64>>,
    pub precision: i32,
}

#[allow(dead_code)]
impl TickersData {
    pub fn get_candles_iter(&self) -> impl Iterator<Item = &Candle<f64>> {
        self.candles.iter()
    }
    pub fn get_candles(&self) -> &Vec<Candle<f64>> {
        &self.candles
    }
    pub fn get_candles_decimal_iter(&self) -> impl Iterator<Item = Candle<Decimal>> {
        let precision = self.precision;
        self.candles.iter().map(move |candle| candle.to_decimal(precision))
    }
    pub fn get_candles_decimal(&self) -> Vec<Candle<Decimal>> {
        self.get_candles_decimal_iter().collect()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FinData {
    tickers_data: HashMap<String, TickersData>,
    piped_data: HashMap<String, String>,
    call_arguments: HashMap<String, String>,
}

impl FromBytesOwned for FinData {
    fn from_bytes_owned(bytes: &[u8]) -> Result<Self, extism_pdk::Error> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[allow(dead_code)]
impl FinData {
    pub fn get_labels(&self) -> Vec<&String> {
        self.tickers_data.keys().collect()
    }

    pub fn get_candles(&self, label: &str) -> Result<&Vec<Candle<f64>>, WithReturnCode<Error>> {
        self.tickers_data
            .get(label)
            .and_then(|v| Some(&v.candles))
            .ok_or(WithReturnCode::new(
                Error::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Symbol {} not found", label),
                )),
                1,
            ))
    }

    pub fn get_candles_iter(&self, label: &str) -> Result<impl Iterator<Item = &Candle<f64>>, WithReturnCode<Error>> {
        let candles = self.get_candles(label)?;
        Ok(candles.iter())
    }

    pub fn get_pipe_sources(&self) -> Vec<&String> {
        self.piped_data.keys().collect()
    }

    pub fn get_data_from_pipe(&self, source: &str) -> Result<&String, WithReturnCode<Error>> {
        self.piped_data.get(source).ok_or(WithReturnCode::new(
            Error::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Source {} not found", source),
            )),
            2,
        ))
    }

    pub fn get_ticker(&self, label: &str) -> Result<&TickersData, WithReturnCode<Error>> {
        self.tickers_data.get(label).ok_or(WithReturnCode::new(
            Error::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Ticker {} not found", label),
            )),
            3,
        ))
    }
    /// Returns the candles as Decimal, precision is taken from the ticker
    pub fn get_candles_decimal_iter(&self, label: &str) -> Result<impl Iterator<Item = Candle<Decimal>>, WithReturnCode<Error>> {
        let ticker = self.get_ticker(label)?;
        Ok(ticker.get_candles_decimal_iter())
    }
    /// Returns the candles as Decimal, precision is taken from the ticker
    pub fn get_candles_decimal(&self, label: &str) -> Result<Vec<Candle<Decimal>>, WithReturnCode<Error>> {
        Ok(self.get_candles_decimal_iter(label)?.collect())
    }

    // Returns the call arguments as a HashMap
    pub fn get_call_arguments(&self) -> &HashMap<String, String> {
        &self.call_arguments
    }
    pub fn get_call_argument<T: FromStr>(&self, key: &str) -> Result<T, WithReturnCode<Error>> {
        let arg = self.call_arguments.get(key).ok_or(WithReturnCode::new(
            Error::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Call argument {} not found", key),
            )),
            4,
        ))?;
        let res = arg.parse::<T>().map_err(|_| {
            WithReturnCode::new(
                Error::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Call argument {} not found", key),
                )),
                5,
            )
        })?;
        Ok(res)
    }
}

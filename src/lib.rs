mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{FnResult, Json, ToBytes, encoding, plugin_fn};
use serde::Serialize;
use ta::{Next, indicators::BollingerBands};

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
enum Side {
    LONG,
    SHORT,
}
#[derive(Debug, Clone, Copy)]
struct OpenTrade {
    pub open_price: f64,
    pub amount: f64,
    pub side: Side,
}

#[derive(Serialize)]
struct ClosedTrade {
    pub open_price: f64,
    pub close_price: f64,
    pub amount: f64,
    pub side: Side,
}

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
struct BacktestResult {
    pub trades: Vec<ClosedTrade>,
    pub total_profit: f64,
    pub symbol: String,
    pub exchange: String,
}
/// Bollinger Bands Mean Reversion Strategy Backtest
///
/// Strategy Logic:
/// - BUY: When price touches the lower Bollinger Band (oversold condition)
/// - SELL: When price touches the upper Bollinger Band (overbought condition)
/// - Stop Loss: Configurable percentage below/above entry price
/// - Take Profit: Configurable percentage above/below entry price
///
/// Parameters:
/// - period: Number of periods for moving average calculation
/// - multiplier: Standard deviation multiplier for bands
/// - sl: Stop loss percentage (e.g., 0.02 = 2%)
/// - tp: Take profit percentage (e.g., 0.04 = 4%)
/// - usd_balance: Amount in USD to allocate per trade
#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<BacktestResult> {
    let ticker = fin_data.get_ticker("symbol_data")?;
    let bb_period = fin_data.get_call_argument("period")?;
    let multiplier: f64 = fin_data.get_call_argument("multiplier")?;
    let sl: f64 = fin_data.get_call_argument("sl")?;
    let tp: f64 = fin_data.get_call_argument("tp")?;
    let usd_balance: f64 = fin_data.get_call_argument("usd_balance")?;

    // Validate input parameters
    if ticker.candles.len() < bb_period {
        return Ok(BacktestResult {
            trades: vec![],
            total_profit: 0.0,
            symbol: ticker.symbol.clone(),
            exchange: ticker.exchange.clone(),
        });
    }
    let mut trades: Vec<ClosedTrade> = vec![];
    let mut open_trade: Option<OpenTrade> = None;

    let mut bb = BollingerBands::new(bb_period, multiplier).expect("Failed to create Bollinger Bands");

    // Initialize the Bollinger Bands with the first bb_period candles
    let mut candles_iter = ticker.candles.iter();
    candles_iter.by_ref().take(bb_period).for_each(|candle| {
        bb.next(candle.close);
    });

    // Process remaining candles for the backtest
    for candle in candles_iter {
        let v = bb.next(candle.close);

        match open_trade {
            Some(trade) => {
                let sl_price = match trade.side {
                    Side::LONG => trade.open_price * (1.0 - sl),
                    Side::SHORT => trade.open_price * (1.0 + sl),
                };
                let tp_price = match trade.side {
                    Side::LONG => trade.open_price * (1.0 + tp),
                    Side::SHORT => trade.open_price * (1.0 - tp),
                };

                let should_close_long =
                    trade.side == Side::LONG && (candle.close < sl_price || candle.close > tp_price);
                let should_close_short =
                    trade.side == Side::SHORT && (candle.close > sl_price || candle.close < tp_price);

                if should_close_long || should_close_short {
                    trades.push(ClosedTrade {
                        open_price: trade.open_price,
                        close_price: candle.close,
                        amount: trade.amount,
                        side: trade.side,
                    });
                    open_trade = None;
                }
            }
            None => {
                if candle.close > v.upper {
                    // Open a short trade
                    open_trade = Some(OpenTrade {
                        open_price: candle.close,
                        amount: usd_balance / candle.close,
                        side: Side::SHORT,
                    });
                } else if candle.close < v.lower {
                    // Open a long trade
                    open_trade = Some(OpenTrade {
                        open_price: candle.close,
                        amount: usd_balance / candle.close,
                        side: Side::LONG,
                    });
                }
            }
        }
    }
    if let Some(trade) = open_trade {
        trades.push(ClosedTrade {
            open_price: trade.open_price,
            close_price: ticker.candles.last().expect("No candles").close,
            amount: trade.amount,
            side: trade.side,
        });
    }

    Ok(BacktestResult {
        total_profit: trades
            .iter()
            .map(|t| match t.side {
                Side::LONG => (t.close_price - t.open_price) * t.amount,
                Side::SHORT => (t.open_price - t.close_price) * t.amount,
            })
            .sum(),
        trades,
        symbol: ticker.symbol.clone(),
        exchange: ticker.exchange.clone(),
    })
}

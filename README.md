# Bollinger Bands Mean Reversion Backtest

A Rust-based Bollinger Bands mean reversion trading strategy backtest that compiles to WebAssembly for deployment on [ExchangeOutpost](https://www.exchangeoutpost.com/).

## Overview

This project implements a complete Bollinger Bands trading strategy backtest using Rust and the `ta` crate for technical analysis. The strategy follows a mean reversion approach, buying when prices touch the lower band (oversold) and selling when prices touch the upper band (overbought), with configurable stop-loss and take-profit levels.

## Features

- **Bollinger Bands Mean Reversion Strategy**: Classic mean reversion approach using Bollinger Bands
- **Comprehensive Backtesting**: Full trade execution simulation with profit/loss calculation
- **Risk Management**: Configurable stop-loss and take-profit levels
- **WebAssembly Compilation**: Functions compile to WASM for cross-platform execution
- **Technical Analysis Integration**: Uses the `ta` crate for Bollinger Bands calculation
- **Type Safety**: Strongly typed trading structures with serde serialization support

## Strategy Logic

The Bollinger Bands mean reversion strategy operates on the following principles:

### Entry Signals
- **Long Entry**: When price touches or falls below the lower Bollinger Band (oversold condition)
- **Short Entry**: When price touches or rises above the upper Bollinger Band (overbought condition)

### Exit Conditions
- **Stop Loss**: Configurable percentage loss limit (e.g., 2%)
- **Take Profit**: Configurable percentage gain target (e.g., 4%)
- **Opposite Signal**: Position is closed when an opposite entry signal occurs

### Risk Management
- Only one position is held at a time
- Fixed position size of 1.0 unit per trade
- Automatic position closure at the end of the backtest period

## Project Structure

```
src/
├── lib.rs                      # Main strategy implementation and backtest logic
└── exchange_outpost/           # Financial data structures and utility functions
├── manifest.json              # Function configuration and parameter schema
├── STRATEGY.md               # Detailed strategy documentation
└── Cargo.toml               # Dependencies including ta crate
```

## Strategy Parameters

The backtest accepts the following configurable parameters:

| Parameter | Type | Range | Default | Description |
|-----------|------|-------|---------|-------------|
| `period` | integer | 2-200 | 20 | Number of periods for moving average calculation |
| `multiplier` | number | 0.1-5.0 | 2.0 | Standard deviation multiplier for Bollinger Bands |
| `sl` | number | 0.001-0.5 | 0.02 | Stop loss percentage (0.02 = 2%) |
| `tp` | number | 0.001-1.0 | 0.04 | Take profit percentage (0.04 = 4%) |
| `usd_balance` | number | 1.0+ | 1000.0 | Amount in USD to allocate per trade |

### Example Configuration

```json
{
    "period": 20,
    "multiplier": 2.0,
    "sl": 0.02,
    "tp": 0.04,
    "usd_balance": 1000.0
}
```

This configuration creates Bollinger Bands with:
- 20-period moving average
- 2 standard deviations for upper/lower bands
- 2% stop loss
- 4% take profit target
- $1000 USD allocation per trade

## Backtest Output

The backtest returns comprehensive results in the following format:

```rust
BacktestResult {
    trades: Vec<ClosedTrade>,     // Detailed list of all executed trades
    total_profit: f64,           // Total profit/loss from all trades
}
```

### Trade Details
Each trade contains:
- `open_price`: Entry price
- `close_price`: Exit price  
- `amount`: Position size (fixed at 1.0)
- `side`: Trade direction (LONG or SHORT)


## Core Components

### Strategy Implementation
The main strategy logic is implemented in `src/lib.rs` and includes:

- **BollingerBands**: Technical indicator from the `ta` crate
- **OpenTrade**: Struct representing an active position
- **ClosedTrade**: Struct representing a completed trade
- **BacktestResult**: Final output containing all trades and total profit

### Data Structures

```rust
struct OpenTrade {
    pub open_price: f64,    // Entry price
    pub amount: f64,        // Position size
    pub side: Side,         // LONG or SHORT
}

struct ClosedTrade {
    pub open_price: f64,    // Entry price
    pub close_price: f64,   // Exit price
    pub amount: f64,        // Position size
    pub side: Side,         // LONG or SHORT
}

struct BacktestResult {
    pub trades: Vec<ClosedTrade>,  // All executed trades
    pub total_profit: f64,         // Total P&L
}
```

## Getting Started

### Prerequisites

- Rust 1.70+ with 2024 edition support
- `wasm32-unknown-unknown` target installed

### Installation

1. Clone this repository:
```bash
git clone https://github.com/AlessandroRuggiero/bollinger-bands-backtest.git
cd bollinger-bands-backtest
```

2. Install the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Build the project:
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Local Testing

You can test the strategy by modifying the parameters in `manifest.json` and building the project. The backtest will process candlestick data according to the Bollinger Bands strategy logic.

## Strategy Development

### Algorithm Overview

The backtest implements the following algorithm:

1. **Initialize Bollinger Bands** with specified period and multiplier
2. **Process each candle** after the initialization period
3. **Check for entry signals**:
   - Long when price touches lower band
   - Short when price touches upper band
4. **Manage open positions**:
   - Apply stop-loss and take-profit rules
   - Close position if hit
5. **Calculate final results** including total profit and trade details

### Key Implementation Details

```rust
// Initialize Bollinger Bands indicator
let mut bb = BollingerBands::new(bb_period, multiplier)?;

// Entry logic example
if candle.close > v.upper {
    // Open short position (overbought)
    open_trade = Some(OpenTrade {
        open_price: candle.close,
        amount: 1.0,
        side: Side::SHORT,
    });
} else if candle.close < v.lower {
    // Open long position (oversold)
    open_trade = Some(OpenTrade {
        open_price: candle.close,
        amount: 1.0,
        side: Side::LONG,
    });
}
```

### Profit Calculation

Profits are calculated differently based on position side:
- **Long positions**: `(close_price - open_price) * amount`
- **Short positions**: `(open_price - close_price) * amount`

### Customization

You can modify the strategy by:
1. Adjusting entry/exit conditions in the main loop
2. Implementing different position sizing rules
3. Adding additional technical indicators
4. Modifying risk management parameters

## Building and Deployment

### Local Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be located at:
`target/wasm32-unknown-unknown/release/rust-function-template.wasm`

### Automated Releases

This project includes GitHub Actions for automated releases. When you push a tag, it will:
1. Build the WASM binary
2. Create a GitHub release
3. Upload the binary as `finfunc.wasm`

To create a release:
```bash
git tag 1.0.0
git push origin 1.0.0
```
Tags must follow [semantic versioning](https://semver.org/).

### Testing Your Function
When pushing to the `master` branch, the CI will automatically build your function and create a preview release named `master`.
You can use this release to test your function on the ExchangeOutpost platform.

## Dependencies

- **extism-pdk** (1.4.1): Plugin development kit for WebAssembly functions
- **rust_decimal** (1.37.2): High-precision decimal arithmetic for financial calculations
- **serde** (1.0.219): Serialization/deserialization framework
- **serde_json** (1.0.143): JSON support for serde
- **ta** (0.5.0): Technical analysis library for Bollinger Bands calculation

## Performance Considerations

- The backtest processes data sequentially for accurate simulation
- Memory usage is optimized by storing only necessary trade history
- Bollinger Bands calculation uses efficient rolling window algorithms
- WebAssembly compilation ensures fast execution across platforms

## Usage Examples

### Basic Backtest
Deploy the function on ExchangeOutpost with default parameters:
- Period: 20
- Multiplier: 2.0  
- Stop Loss: 2%
- Take Profit: 4%

### Conservative Strategy
For lower risk tolerance:
- Period: 30 (longer moving average)
- Multiplier: 2.5 (wider bands)
- Stop Loss: 1.5%
- Take Profit: 3%

### Aggressive Strategy  
For higher risk tolerance:
- Period: 10 (shorter moving average)
- Multiplier: 1.5 (tighter bands)
- Stop Loss: 3%
- Take Profit: 6%

## Limitations

- Fixed position sizing (1.0 unit per trade)
- No transaction costs or slippage modeling
- Single timeframe analysis
- No portfolio or correlation considerations
- Mean reversion assumption may not hold in trending markets

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/strategy-enhancement`)
3. Commit your changes (`git commit -m 'Add enhanced exit conditions'`)
4. Push to the branch (`git push origin feature/strategy-enhancement`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for more details.

## Related Links

- [ExchangeOutpost Platform](https://www.exchangeoutpost.com/)
- [Bollinger Bands Documentation](https://en.wikipedia.org/wiki/Bollinger_Bands)
- [TA Crate Documentation](https://docs.rs/ta/latest/ta/)
- [Extism Documentation](https://extism.org/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)

# Bollinger Bands Backtest

A Rust-based backtesting engine for trading strategies using Bollinger Bands indicators. This project implements a complete backtesting framework that can be used as a FinFunc on `ExchnageOutpost` for financial analisys.

## Overview

This backtesting engine simulates trading decisions based on Bollinger Bands technical analysis:
- **Long positions** when price touches the lower band
- **Short positions** when price touches the upper band
- Stop-loss and take-profit mechanisms for risk management

## Features

- 📈 **Bollinger Bands Strategy**: Automated trading signals based on price touching upper/lower bands
- 🛡️ **Risk Management**: Configurable stop-loss and take-profit levels
- 📊 **Comprehensive Results**: Detailed trade history and profit calculations
- ⚡ **High Performance**: Written in Rust for optimal execution speed

## Strategy Logic

The backtesting strategy follows these rules:

1. **Entry Signals**:
   - **Short Entry**: When price closes above the upper Bollinger Band
   - **Long Entry**: When price closes below the lower Bollinger Band

2. **Exit Conditions**:
   - **Stop Loss**: Closes position when loss reaches the configured percentage
   - **Take Profit**: Closes position when profit reaches the configured percentage
   - **End of Data**: Any open position is closed at the last available price

## Output Format

The backtest returns results in the following structure:

```json
{
  "profit": 1250.5,
  "trades": [
    {
      "open_price": 47000.0,
      "close_price": 47940.0,
      "amount": 1.0,
      "side": "LONG"
    },
    ...
  ]
}
```

## Running on ExchnageOutpost
Example of arguments to run the backtest on `ExchnageOutpost`:

```json
{
  "source_code": {
    "repository": "ExchangeOutpost/BollingerBandsBacktest",
    "release": "0.1.0"
  },
  "arguments": [
    {
      "financial_data": {
        "symbol_data": {
          "exchange": "binance",
          "symbol": "BTCUSDT",
          "timeframe": "1m",
          "start_date": "2025-03-01",
          "end_date": "2025-05-01"
        }
      },
      "call_arguments": {
        "period": 20,
        "multiplier": 2,
        "sl": 0.02,
        "tp": 0.04
      }
    }
  ],
  "hang": true
}

```


## Risk Disclaimer

This backtesting engine is for educational and research purposes only. Past performance does not guarantee future results. Always conduct thorough testing and risk assessment before using any trading strategy with real capital.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.
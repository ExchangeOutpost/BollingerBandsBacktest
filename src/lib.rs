mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use serde::Serialize;
use ta::{indicators::BollingerBands, Next};

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
enum Side {
    LONG,
    SHORT
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
    pub profit: f64,
}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<BacktestResult> {
    let mut open_trade : Option<OpenTrade> = None;
    let candles = fin_data.get_candles("symbol_data")?;
    let mut bb = BollingerBands::new(20, 2.0).expect("Failed to create Bollinger Bands");
    let mut trades: Vec<ClosedTrade> = vec![];
    let sl = 0.01;
    let tp = 0.01;

    for candle in candles.iter().skip(20) {
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
                if (trade.side == Side::LONG && candle.close < sl_price) || (trade.side == Side::SHORT && candle.close > sl_price) {
                    trades.push(ClosedTrade {
                        open_price: trade.open_price,
                        close_price: candle.close,
                        amount: trade.amount,
                        side: trade.side,
                    });
                    open_trade = None;
                } else if (trade.side == Side::LONG && candle.close > tp_price) || (trade.side == Side::SHORT && candle.close < tp_price) {
                    trades.push(ClosedTrade {
                        open_price: trade.open_price,
                        close_price: candle.close,
                        amount: trade.amount,
                        side: trade.side,
                    });
                    open_trade = None;
                }
            },
            None => {
                if candle.close > v.upper {
                    // Open a short trade
                    open_trade = Some(OpenTrade {
                        open_price: candle.close,
                        amount: 1 as f64,
                        side: Side::SHORT,
                    });
                    
                } else if candle.close < v.lower {
                    // Open a long trade
                    open_trade = Some(OpenTrade {
                        open_price: candle.close,
                        amount: 1 as f64,
                        side: Side::LONG,
                    });
                }
            },
        }

    }
    if let Some(trade) = open_trade {
        trades.push(ClosedTrade {
            open_price: trade.open_price,
            close_price: candles.last().expect("No candles").close,
            amount: trade.amount,
            side: trade.side,
        });
    }

    Ok(BacktestResult {
        profit: trades.iter().map(|trade| {
            if trade.side == Side::LONG {
                (trade.close_price - trade.open_price) * trade.amount
            } else {
                (trade.open_price - trade.close_price) * trade.amount
            }
        }).sum(),
        trades,
    })
}
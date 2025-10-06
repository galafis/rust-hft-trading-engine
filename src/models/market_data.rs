use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub last_price: Decimal,
    pub volume: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub bid_price: Decimal,
    pub bid_size: Decimal,
    pub ask_price: Decimal,
    pub ask_size: Decimal,
    pub timestamp: DateTime<Utc>,
}

impl Quote {
    pub fn spread(&self) -> Decimal {
        self.ask_price - self.bid_price
    }

    pub fn mid_price(&self) -> Decimal {
        (self.bid_price + self.ask_price) / Decimal::TWO
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub ticker: Option<Ticker>,
    pub quote: Option<Quote>,
}

impl MarketData {
    pub fn new() -> Self {
        Self {
            ticker: None,
            quote: None,
        }
    }

    pub fn with_ticker(ticker: Ticker) -> Self {
        Self {
            ticker: Some(ticker),
            quote: None,
        }
    }

    pub fn with_quote(quote: Quote) -> Self {
        Self {
            ticker: None,
            quote: Some(quote),
        }
    }
}

impl Default for MarketData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_quote_calculations() {
        let quote = Quote {
            symbol: "AAPL".to_string(),
            bid_price: dec!(150.00),
            bid_size: dec!(100),
            ask_price: dec!(151.00),
            ask_size: dec!(100),
            timestamp: Utc::now(),
        };

        assert_eq!(quote.spread(), dec!(1.00));
        assert_eq!(quote.mid_price(), dec!(150.50));
    }
}

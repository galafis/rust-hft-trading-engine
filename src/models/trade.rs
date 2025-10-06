use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::OrderSide;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub symbol: String,
    pub buyer_order_id: Uuid,
    pub seller_order_id: Uuid,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
    pub timestamp: DateTime<Utc>,
}

impl Trade {
    pub fn new(
        symbol: String,
        buyer_order_id: Uuid,
        seller_order_id: Uuid,
        price: Decimal,
        quantity: Decimal,
        side: OrderSide,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            symbol,
            buyer_order_id,
            seller_order_id,
            price,
            quantity,
            side,
            timestamp: Utc::now(),
        }
    }

    pub fn notional_value(&self) -> Decimal {
        self.price * self.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_trade_creation() {
        let trade = Trade::new(
            "AAPL".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            dec!(150.50),
            dec!(100),
            OrderSide::Buy,
        );

        assert_eq!(trade.symbol, "AAPL");
        assert_eq!(trade.price, dec!(150.50));
        assert_eq!(trade.quantity, dec!(100));
    }

    #[test]
    fn test_notional_value() {
        let trade = Trade::new(
            "AAPL".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            dec!(150.50),
            dec!(100),
            OrderSide::Buy,
        );

        assert_eq!(trade.notional_value(), dec!(15050.00));
    }
}

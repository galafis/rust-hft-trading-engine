use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub price: Option<Decimal>,
    pub stop_price: Option<Decimal>,
    pub status: OrderStatus,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(
        symbol: String,
        side: OrderSide,
        order_type: OrderType,
        quantity: Decimal,
        price: Option<Decimal>,
        stop_price: Option<Decimal>,
        user_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            symbol,
            side,
            order_type,
            quantity,
            filled_quantity: Decimal::ZERO,
            price,
            stop_price,
            status: OrderStatus::Pending,
            user_id,
            timestamp: now,
            updated_at: now,
        }
    }

    pub fn is_fully_filled(&self) -> bool {
        self.filled_quantity >= self.quantity
    }

    pub fn remaining_quantity(&self) -> Decimal {
        self.quantity - self.filled_quantity
    }

    pub fn fill(&mut self, quantity: Decimal) {
        self.filled_quantity += quantity;
        self.updated_at = Utc::now();
        
        if self.is_fully_filled() {
            self.status = OrderStatus::Filled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }
    }

    pub fn cancel(&mut self) {
        self.status = OrderStatus::Cancelled;
        self.updated_at = Utc::now();
    }

    pub fn reject(&mut self) {
        self.status = OrderStatus::Rejected;
        self.updated_at = Utc::now();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= Decimal::ZERO {
            return Err("Quantity must be positive".to_string());
        }

        match self.order_type {
            OrderType::Limit | OrderType::StopLimit => {
                if self.price.is_none() || self.price.unwrap() <= Decimal::ZERO {
                    return Err("Limit orders must have a positive price".to_string());
                }
            }
            _ => {}
        }

        match self.order_type {
            OrderType::StopLoss | OrderType::StopLimit => {
                if self.stop_price.is_none() || self.stop_price.unwrap() <= Decimal::ZERO {
                    return Err("Stop orders must have a positive stop price".to_string());
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.50)),
            None,
            "user123".to_string(),
        );

        assert_eq!(order.symbol, "AAPL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.quantity, dec!(100));
        assert_eq!(order.status, OrderStatus::Pending);
    }

    #[test]
    fn test_order_fill() {
        let mut order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.50)),
            None,
            "user123".to_string(),
        );

        order.fill(dec!(50));
        assert_eq!(order.filled_quantity, dec!(50));
        assert_eq!(order.status, OrderStatus::PartiallyFilled);
        assert_eq!(order.remaining_quantity(), dec!(50));

        order.fill(dec!(50));
        assert_eq!(order.filled_quantity, dec!(100));
        assert_eq!(order.status, OrderStatus::Filled);
        assert!(order.is_fully_filled());
    }

    #[test]
    fn test_order_validation() {
        let valid_order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.50)),
            None,
            "user123".to_string(),
        );
        assert!(valid_order.validate().is_ok());

        let invalid_order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(-100),
            Some(dec!(150.50)),
            None,
            "user123".to_string(),
        );
        assert!(invalid_order.validate().is_err());
    }
}

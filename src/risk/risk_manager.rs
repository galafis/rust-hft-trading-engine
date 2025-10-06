use dashmap::DashMap;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::models::{Order, Trade};

#[derive(Debug, Clone)]
pub struct RiskLimits {
    pub max_order_size: Decimal,
    pub max_position_size: Decimal,
    pub max_daily_loss: Decimal,
    pub max_order_value: Decimal,
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self {
            max_order_size: Decimal::from(10000),
            max_position_size: Decimal::from(100000),
            max_daily_loss: Decimal::from(50000),
            max_order_value: Decimal::from(1000000),
        }
    }
}

#[derive(Debug)]
pub struct RiskCheck {
    pub passed: bool,
    pub reason: Option<String>,
}

impl RiskCheck {
    pub fn pass() -> Self {
        Self {
            passed: true,
            reason: None,
        }
    }

    pub fn fail(reason: String) -> Self {
        Self {
            passed: false,
            reason: Some(reason),
        }
    }
}

pub struct RiskManager {
    limits: RiskLimits,
    positions: Arc<DashMap<String, Decimal>>,
    daily_pnl: Arc<DashMap<String, Decimal>>,
}

impl RiskManager {
    pub fn new(limits: RiskLimits) -> Self {
        Self {
            limits,
            positions: Arc::new(DashMap::new()),
            daily_pnl: Arc::new(DashMap::new()),
        }
    }

    pub fn check_order(&self, order: &Order) -> RiskCheck {
        // Check order size
        if order.quantity > self.limits.max_order_size {
            return RiskCheck::fail(format!(
                "Order size {} exceeds maximum {}",
                order.quantity, self.limits.max_order_size
            ));
        }

        // Check order value for limit orders
        if let Some(price) = order.price {
            let order_value = price * order.quantity;
            if order_value > self.limits.max_order_value {
                return RiskCheck::fail(format!(
                    "Order value {} exceeds maximum {}",
                    order_value, self.limits.max_order_value
                ));
            }
        }

        // Check position size
        let current_position = self.get_position(&order.user_id);
        let new_position = match order.side {
            crate::models::OrderSide::Buy => current_position + order.quantity,
            crate::models::OrderSide::Sell => current_position - order.quantity,
        };

        if new_position.abs() > self.limits.max_position_size {
            return RiskCheck::fail(format!(
                "New position {} would exceed maximum {}",
                new_position, self.limits.max_position_size
            ));
        }

        // Check daily loss
        let daily_loss = self.get_daily_pnl(&order.user_id);
        if daily_loss.abs() > self.limits.max_daily_loss {
            return RiskCheck::fail(format!(
                "Daily loss {} exceeds maximum {}",
                daily_loss, self.limits.max_daily_loss
            ));
        }

        RiskCheck::pass()
    }

    pub fn update_position(&self, user_id: &str, trade: &Trade) {
        let mut position = self.positions.entry(user_id.to_string()).or_insert(Decimal::ZERO);
        
        match trade.side {
            crate::models::OrderSide::Buy => *position += trade.quantity,
            crate::models::OrderSide::Sell => *position -= trade.quantity,
        }
    }

    pub fn update_pnl(&self, user_id: &str, pnl: Decimal) {
        let mut daily_pnl = self.daily_pnl.entry(user_id.to_string()).or_insert(Decimal::ZERO);
        *daily_pnl += pnl;
    }

    pub fn get_position(&self, user_id: &str) -> Decimal {
        self.positions.get(user_id).map(|p| *p).unwrap_or(Decimal::ZERO)
    }

    pub fn get_daily_pnl(&self, user_id: &str) -> Decimal {
        self.daily_pnl.get(user_id).map(|p| *p).unwrap_or(Decimal::ZERO)
    }

    pub fn reset_daily_pnl(&self) {
        self.daily_pnl.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OrderSide, OrderType};
    use rust_decimal_macros::dec;

    fn create_test_order(quantity: Decimal, price: Decimal) -> Order {
        Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            quantity,
            Some(price),
            None,
            "user123".to_string(),
        )
    }

    #[test]
    fn test_order_size_check() {
        let limits = RiskLimits {
            max_order_size: dec!(1000),
            ..Default::default()
        };
        let risk_manager = RiskManager::new(limits);

        let valid_order = create_test_order(dec!(500), dec!(150.00));
        let check = risk_manager.check_order(&valid_order);
        assert!(check.passed);

        let invalid_order = create_test_order(dec!(2000), dec!(150.00));
        let check = risk_manager.check_order(&invalid_order);
        assert!(!check.passed);
    }

    #[test]
    fn test_order_value_check() {
        let limits = RiskLimits {
            max_order_value: dec!(100000),
            ..Default::default()
        };
        let risk_manager = RiskManager::new(limits);

        let valid_order = create_test_order(dec!(500), dec!(150.00));
        let check = risk_manager.check_order(&valid_order);
        assert!(check.passed);

        let invalid_order = create_test_order(dec!(1000), dec!(1000.00));
        let check = risk_manager.check_order(&invalid_order);
        assert!(!check.passed);
    }

    #[test]
    fn test_position_tracking() {
        let risk_manager = RiskManager::new(RiskLimits::default());
        
        let trade = Trade::new(
            "AAPL".to_string(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            dec!(150.00),
            dec!(100),
            OrderSide::Buy,
        );

        risk_manager.update_position("user123", &trade);
        assert_eq!(risk_manager.get_position("user123"), dec!(100));
    }

    #[test]
    fn test_pnl_tracking() {
        let risk_manager = RiskManager::new(RiskLimits::default());
        
        risk_manager.update_pnl("user123", dec!(1000));
        risk_manager.update_pnl("user123", dec!(-500));
        
        assert_eq!(risk_manager.get_daily_pnl("user123"), dec!(500));
    }
}

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

use super::{Order, OrderSide};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: Decimal,
    pub total_quantity: Decimal,
    pub orders: Vec<Uuid>,
}

impl PriceLevel {
    pub fn new(price: Decimal) -> Self {
        Self {
            price,
            total_quantity: Decimal::ZERO,
            orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order_id: Uuid, quantity: Decimal) {
        self.orders.push(order_id);
        self.total_quantity += quantity;
    }

    pub fn remove_order(&mut self, order_id: Uuid, quantity: Decimal) {
        self.orders.retain(|&id| id != order_id);
        self.total_quantity -= quantity;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: BTreeMap<Decimal, PriceLevel>,
    pub asks: BTreeMap<Decimal, PriceLevel>,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: &Order) {
        let price = order.price.unwrap_or(Decimal::ZERO);
        let quantity = order.remaining_quantity();

        let book = match order.side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        book.entry(price)
            .or_insert_with(|| PriceLevel::new(price))
            .add_order(order.id, quantity);
    }

    pub fn remove_order(&mut self, order: &Order) {
        let price = order.price.unwrap_or(Decimal::ZERO);
        let quantity = order.remaining_quantity();

        let book = match order.side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        if let Some(level) = book.get_mut(&price) {
            level.remove_order(order.id, quantity);
            if level.orders.is_empty() {
                book.remove(&price);
            }
        }
    }

    pub fn best_bid(&self) -> Option<Decimal> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<Decimal> {
        self.asks.keys().next().copied()
    }

    pub fn spread(&self) -> Option<Decimal> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    pub fn mid_price(&self) -> Option<Decimal> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / Decimal::TWO),
            _ => None,
        }
    }

    pub fn depth(&self, side: OrderSide, levels: usize) -> Vec<(Decimal, Decimal)> {
        let book = match side {
            OrderSide::Buy => &self.bids,
            OrderSide::Sell => &self.asks,
        };

        match side {
            OrderSide::Buy => book
                .iter()
                .rev()
                .take(levels)
                .map(|(price, level)| (*price, level.total_quantity))
                .collect(),
            OrderSide::Sell => book
                .iter()
                .take(levels)
                .map(|(price, level)| (*price, level.total_quantity))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OrderType, OrderStatus};
    use rust_decimal_macros::dec;

    fn create_test_order(side: OrderSide, price: Decimal, quantity: Decimal) -> Order {
        Order {
            id: Uuid::new_v4(),
            symbol: "AAPL".to_string(),
            side,
            order_type: OrderType::Limit,
            quantity,
            filled_quantity: Decimal::ZERO,
            price: Some(price),
            stop_price: None,
            status: OrderStatus::Pending,
            user_id: "test_user".to_string(),
            timestamp: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_orderbook_creation() {
        let book = OrderBook::new("AAPL".to_string());
        assert_eq!(book.symbol, "AAPL");
        assert!(book.bids.is_empty());
        assert!(book.asks.is_empty());
    }

    #[test]
    fn test_add_orders() {
        let mut book = OrderBook::new("AAPL".to_string());
        
        let buy_order = create_test_order(OrderSide::Buy, dec!(150.00), dec!(100));
        let sell_order = create_test_order(OrderSide::Sell, dec!(151.00), dec!(100));

        book.add_order(&buy_order);
        book.add_order(&sell_order);

        assert_eq!(book.best_bid(), Some(dec!(150.00)));
        assert_eq!(book.best_ask(), Some(dec!(151.00)));
    }

    #[test]
    fn test_spread_calculation() {
        let mut book = OrderBook::new("AAPL".to_string());
        
        let buy_order = create_test_order(OrderSide::Buy, dec!(150.00), dec!(100));
        let sell_order = create_test_order(OrderSide::Sell, dec!(151.00), dec!(100));

        book.add_order(&buy_order);
        book.add_order(&sell_order);

        assert_eq!(book.spread(), Some(dec!(1.00)));
        assert_eq!(book.mid_price(), Some(dec!(150.50)));
    }

    #[test]
    fn test_depth() {
        let mut book = OrderBook::new("AAPL".to_string());
        
        book.add_order(&create_test_order(OrderSide::Buy, dec!(150.00), dec!(100)));
        book.add_order(&create_test_order(OrderSide::Buy, dec!(149.00), dec!(200)));
        book.add_order(&create_test_order(OrderSide::Sell, dec!(151.00), dec!(150)));
        book.add_order(&create_test_order(OrderSide::Sell, dec!(152.00), dec!(250)));

        let bid_depth = book.depth(OrderSide::Buy, 2);
        assert_eq!(bid_depth.len(), 2);
        assert_eq!(bid_depth[0], (dec!(150.00), dec!(100)));
        assert_eq!(bid_depth[1], (dec!(149.00), dec!(200)));

        let ask_depth = book.depth(OrderSide::Sell, 2);
        assert_eq!(ask_depth.len(), 2);
        assert_eq!(ask_depth[0], (dec!(151.00), dec!(150)));
        assert_eq!(ask_depth[1], (dec!(152.00), dec!(250)));
    }
}

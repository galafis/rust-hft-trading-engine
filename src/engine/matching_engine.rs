use dashmap::DashMap;
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{Order, OrderBook, OrderSide, OrderStatus, OrderType, Trade};

pub struct MatchingEngine {
    orderbooks: Arc<DashMap<String, OrderBook>>,
    orders: Arc<DashMap<Uuid, Order>>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            orderbooks: Arc::new(DashMap::new()),
            orders: Arc::new(DashMap::new()),
        }
    }

    pub fn submit_order(&self, mut order: Order) -> Result<Vec<Trade>, String> {
        order.validate()?;

        let symbol = order.symbol.clone();
        
        // Ensure orderbook exists
        if !self.orderbooks.contains_key(&symbol) {
            self.orderbooks.insert(symbol.clone(), OrderBook::new(symbol.clone()));
        }

        let mut trades = Vec::new();

        // Handle market orders immediately
        if order.order_type == OrderType::Market {
            trades = self.match_market_order(&mut order)?;
        } else {
            // Try to match limit orders
            trades = self.match_limit_order(&mut order)?;
        }

        // If order is not fully filled, add to orderbook
        if !order.is_fully_filled() && order.status != OrderStatus::Cancelled {
            let mut book = self.orderbooks.get_mut(&symbol).unwrap();
            book.add_order(&order);
        }

        // Store order
        self.orders.insert(order.id, order);

        Ok(trades)
    }

    fn match_market_order(&self, order: &mut Order) -> Result<Vec<Trade>, String> {
        let mut trades = Vec::new();
        let symbol = order.symbol.clone();
        
        let mut book = self.orderbooks.get_mut(&symbol).unwrap();
        
        let opposite_side = match order.side {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy,
        };

        let levels: Vec<(Decimal, Vec<Uuid>)> = match opposite_side {
            OrderSide::Buy => book.bids.iter().rev()
                .map(|(price, level)| (*price, level.orders.clone()))
                .collect(),
            OrderSide::Sell => book.asks.iter()
                .map(|(price, level)| (*price, level.orders.clone()))
                .collect(),
        };

        for (price, order_ids) in levels {
            if order.is_fully_filled() {
                break;
            }

            for order_id in order_ids {
                if order.is_fully_filled() {
                    break;
                }

                if let Some(mut matching_order) = self.orders.get_mut(&order_id) {
                    let trade_quantity = order.remaining_quantity().min(matching_order.remaining_quantity());
                    
                    let (buyer_id, seller_id) = match order.side {
                        OrderSide::Buy => (order.id, matching_order.id),
                        OrderSide::Sell => (matching_order.id, order.id),
                    };

                    let trade = Trade::new(
                        symbol.clone(),
                        buyer_id,
                        seller_id,
                        price,
                        trade_quantity,
                        order.side,
                    );

                    order.fill(trade_quantity);
                    matching_order.fill(trade_quantity);

                    trades.push(trade);

                    if matching_order.is_fully_filled() {
                        book.remove_order(&matching_order);
                    }
                }
            }
        }

        if !order.is_fully_filled() {
            order.reject();
            return Err("Market order could not be fully filled".to_string());
        }

        Ok(trades)
    }

    fn match_limit_order(&self, order: &mut Order) -> Result<Vec<Trade>, String> {
        let mut trades = Vec::new();
        let symbol = order.symbol.clone();
        let order_price = order.price.unwrap();
        
        let mut book = self.orderbooks.get_mut(&symbol).unwrap();
        
        let opposite_side = match order.side {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy,
        };

        let levels: Vec<(Decimal, Vec<Uuid>)> = match opposite_side {
            OrderSide::Buy => book.bids.iter().rev()
                .filter(|(price, _)| **price >= order_price)
                .map(|(price, level)| (*price, level.orders.clone()))
                .collect(),
            OrderSide::Sell => book.asks.iter()
                .filter(|(price, _)| **price <= order_price)
                .map(|(price, level)| (*price, level.orders.clone()))
                .collect(),
        };

        for (price, order_ids) in levels {
            if order.is_fully_filled() {
                break;
            }

            for order_id in order_ids {
                if order.is_fully_filled() {
                    break;
                }

                if let Some(mut matching_order) = self.orders.get_mut(&order_id) {
                    let trade_quantity = order.remaining_quantity().min(matching_order.remaining_quantity());
                    
                    let (buyer_id, seller_id) = match order.side {
                        OrderSide::Buy => (order.id, matching_order.id),
                        OrderSide::Sell => (matching_order.id, order.id),
                    };

                    let trade = Trade::new(
                        symbol.clone(),
                        buyer_id,
                        seller_id,
                        price,
                        trade_quantity,
                        order.side,
                    );

                    order.fill(trade_quantity);
                    matching_order.fill(trade_quantity);

                    trades.push(trade);

                    if matching_order.is_fully_filled() {
                        book.remove_order(&matching_order);
                    }
                }
            }
        }

        Ok(trades)
    }

    pub fn cancel_order(&self, order_id: Uuid) -> Result<(), String> {
        if let Some(mut order) = self.orders.get_mut(&order_id) {
            if order.status == OrderStatus::Filled {
                return Err("Cannot cancel filled order".to_string());
            }

            let symbol = order.symbol.clone();
            order.cancel();

            if let Some(mut book) = self.orderbooks.get_mut(&symbol) {
                book.remove_order(&order);
            }

            Ok(())
        } else {
            Err("Order not found".to_string())
        }
    }

    pub fn get_order(&self, order_id: Uuid) -> Option<Order> {
        self.orders.get(&order_id).map(|o| o.clone())
    }

    pub fn get_orderbook(&self, symbol: &str) -> Option<OrderBook> {
        self.orderbooks.get(symbol).map(|b| b.clone())
    }
}

impl Default for MatchingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_limit_order_matching() {
        let engine = MatchingEngine::new();

        let sell_order = Order::new(
            "AAPL".to_string(),
            OrderSide::Sell,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.00)),
            None,
            "seller".to_string(),
        );

        let buy_order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.00)),
            None,
            "buyer".to_string(),
        );

        engine.submit_order(sell_order).unwrap();
        let trades = engine.submit_order(buy_order).unwrap();

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, dec!(100));
        assert_eq!(trades[0].price, dec!(150.00));
    }

    #[test]
    fn test_partial_fill() {
        let engine = MatchingEngine::new();

        let sell_order = Order::new(
            "AAPL".to_string(),
            OrderSide::Sell,
            OrderType::Limit,
            dec!(50),
            Some(dec!(150.00)),
            None,
            "seller".to_string(),
        );

        let buy_order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.00)),
            None,
            "buyer".to_string(),
        );

        engine.submit_order(sell_order).unwrap();
        let trades = engine.submit_order(buy_order.clone()).unwrap();

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, dec!(50));

        let stored_order = engine.get_order(buy_order.id).unwrap();
        assert_eq!(stored_order.filled_quantity, dec!(50));
        assert_eq!(stored_order.status, OrderStatus::PartiallyFilled);
    }

    #[test]
    fn test_order_cancellation() {
        let engine = MatchingEngine::new();

        let order = Order::new(
            "AAPL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            dec!(100),
            Some(dec!(150.00)),
            None,
            "buyer".to_string(),
        );

        let order_id = order.id;
        engine.submit_order(order).unwrap();
        
        assert!(engine.cancel_order(order_id).is_ok());
        
        let cancelled_order = engine.get_order(order_id).unwrap();
        assert_eq!(cancelled_order.status, OrderStatus::Cancelled);
    }
}

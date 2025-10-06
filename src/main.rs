mod engine;
mod models;
mod risk;

use engine::MatchingEngine;
use models::{Order, OrderSide, OrderType};
use risk::{RiskLimits, RiskManager};
use rust_decimal_macros::dec;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting HFT Trading Engine...");

    // Initialize components
    let engine = MatchingEngine::new();
    let risk_manager = RiskManager::new(RiskLimits::default());

    info!("Components initialized successfully");

    // Example: Submit sell order
    let sell_order = Order::new(
        "AAPL".to_string(),
        OrderSide::Sell,
        OrderType::Limit,
        dec!(100),
        Some(dec!(150.50)),
        None,
        "seller_001".to_string(),
    );

    info!("Checking risk for sell order: {:?}", sell_order.id);
    let risk_check = risk_manager.check_order(&sell_order);
    
    if !risk_check.passed {
        info!("Risk check failed: {:?}", risk_check.reason);
        return;
    }

    info!("Submitting sell order...");
    match engine.submit_order(sell_order) {
        Ok(trades) => {
            if trades.is_empty() {
                info!("Sell order added to orderbook (no immediate match)");
            } else {
                info!("Sell order matched! Trades: {}", trades.len());
            }
        }
        Err(e) => {
            info!("Error submitting sell order: {}", e);
            return;
        }
    }

    // Example: Submit buy order
    let buy_order = Order::new(
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        dec!(100),
        Some(dec!(150.50)),
        None,
        "buyer_001".to_string(),
    );

    info!("Checking risk for buy order: {:?}", buy_order.id);
    let risk_check = risk_manager.check_order(&buy_order);
    
    if !risk_check.passed {
        info!("Risk check failed: {:?}", risk_check.reason);
        return;
    }

    info!("Submitting buy order...");
    match engine.submit_order(buy_order.clone()) {
        Ok(trades) => {
            if trades.is_empty() {
                info!("Buy order added to orderbook (no immediate match)");
            } else {
                info!("Buy order matched! Trades executed:");
                for trade in &trades {
                    info!(
                        "  Trade ID: {}, Price: {}, Quantity: {}, Value: {}",
                        trade.id,
                        trade.price,
                        trade.quantity,
                        trade.notional_value()
                    );
                    
                    // Update risk manager
                    risk_manager.update_position(&buy_order.user_id, trade);
                }
            }
        }
        Err(e) => {
            info!("Error submitting buy order: {}", e);
            return;
        }
    }

    // Display orderbook
    if let Some(orderbook) = engine.get_orderbook("AAPL") {
        info!("Orderbook for AAPL:");
        info!("  Best Bid: {:?}", orderbook.best_bid());
        info!("  Best Ask: {:?}", orderbook.best_ask());
        info!("  Spread: {:?}", orderbook.spread());
        info!("  Mid Price: {:?}", orderbook.mid_price());
    }

    // Display positions
    info!("Position for buyer_001: {}", risk_manager.get_position("buyer_001"));
    info!("Position for seller_001: {}", risk_manager.get_position("seller_001"));

    info!("Trading engine demo completed successfully!");
}

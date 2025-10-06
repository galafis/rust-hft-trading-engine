use rust_decimal_macros::dec;
use rust_hft_trading_engine::{
    MatchingEngine, Order, OrderSide, OrderType, RiskLimits, RiskManager,
};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Advanced Trading Example - Simulating Market Making Strategy");

    // Initialize components
    let engine = MatchingEngine::new();
    let risk_manager = RiskManager::new(RiskLimits::default());

    // Simulate market maker placing orders on both sides
    let symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA"];
    
    for symbol in symbols {
        info!("Setting up market making for {}", symbol);
        
        // Place bid orders (buy side)
        for i in 0..5 {
            let offset = dec!(0.10) * rust_decimal::Decimal::from(i);
            let price = dec!(150.00) - offset;
            let quantity = dec!(100);
            
            let bid_order = Order::new(
                symbol.to_string(),
                OrderSide::Buy,
                OrderType::Limit,
                quantity,
                Some(price),
                None,
                format!("market_maker_{}", symbol),
            );
            
            if risk_manager.check_order(&bid_order).passed {
                match engine.submit_order(bid_order) {
                    Ok(_) => info!("Bid order placed: {} @ {}", quantity, price),
                    Err(e) => info!("Failed to place bid: {}", e),
                }
            }
        }
        
        // Place ask orders (sell side)
        for i in 0..5 {
            let offset = dec!(0.10) * rust_decimal::Decimal::from(i);
            let price = dec!(150.10) + offset;
            let quantity = dec!(100);
            
            let ask_order = Order::new(
                symbol.to_string(),
                OrderSide::Sell,
                OrderType::Limit,
                quantity,
                Some(price),
                None,
                format!("market_maker_{}", symbol),
            );
            
            if risk_manager.check_order(&ask_order).passed {
                match engine.submit_order(ask_order) {
                    Ok(_) => info!("Ask order placed: {} @ {}", quantity, price),
                    Err(e) => info!("Failed to place ask: {}", e),
                }
            }
        }
        
        // Display orderbook state
        if let Some(orderbook) = engine.get_orderbook(symbol) {
            info!("Orderbook for {}:", symbol);
            info!("  Best Bid: {:?}", orderbook.best_bid());
            info!("  Best Ask: {:?}", orderbook.best_ask());
            info!("  Spread: {:?}", orderbook.spread());
            info!("  Mid Price: {:?}", orderbook.mid_price());
            
            // Show depth
            let bid_depth = orderbook.depth(OrderSide::Buy, 3);
            let ask_depth = orderbook.depth(OrderSide::Sell, 3);
            
            info!("  Bid Depth (top 3):");
            for (price, qty) in bid_depth {
                info!("    {} @ {}", qty, price);
            }
            
            info!("  Ask Depth (top 3):");
            for (price, qty) in ask_depth {
                info!("    {} @ {}", qty, price);
            }
        }
    }

    // Simulate aggressive trader taking liquidity
    info!("\nSimulating aggressive trader...");
    
    let aggressive_buy = Order::new(
        "AAPL".to_string(),
        OrderSide::Buy,
        OrderType::Market,
        dec!(250),
        None,
        None,
        "aggressive_trader".to_string(),
    );
    
    match engine.submit_order(aggressive_buy) {
        Ok(trades) => {
            info!("Market order executed! {} trades:", trades.len());
            for trade in &trades {
                info!(
                    "  Trade: {} @ {} = {}",
                    trade.quantity,
                    trade.price,
                    trade.notional_value()
                );
            }
        }
        Err(e) => info!("Market order failed: {}", e),
    }

    info!("\nAdvanced trading simulation completed!");
}

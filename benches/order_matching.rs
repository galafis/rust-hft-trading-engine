use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal_macros::dec;
use rust_hft_trading_engine::{MatchingEngine, Order, OrderSide, OrderType};

fn benchmark_order_submission(c: &mut Criterion) {
    c.bench_function("submit_limit_order", |b| {
        let engine = MatchingEngine::new();
        b.iter(|| {
            let order = Order::new(
                "AAPL".to_string(),
                OrderSide::Buy,
                OrderType::Limit,
                dec!(100),
                Some(dec!(150.00)),
                None,
                "user123".to_string(),
            );
            black_box(engine.submit_order(order))
        });
    });
}

fn benchmark_order_matching(c: &mut Criterion) {
    c.bench_function("match_orders", |b| {
        b.iter(|| {
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
            engine.submit_order(sell_order).unwrap();

            let buy_order = Order::new(
                "AAPL".to_string(),
                OrderSide::Buy,
                OrderType::Limit,
                dec!(100),
                Some(dec!(150.00)),
                None,
                "buyer".to_string(),
            );
            black_box(engine.submit_order(buy_order))
        });
    });
}

criterion_group!(benches, benchmark_order_submission, benchmark_order_matching);
criterion_main!(benches);

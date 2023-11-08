use criterion::{criterion_group, criterion_main, Criterion};
use lobster::{OrderBook, OrderType, Side};

fn small_limit_ladder(c: &mut Criterion) {
    c.bench_function("small limit ladder (100 levels)", |b| {
        let mut ob = OrderBook::default();
        b.iter(|| {
            for i in 0..100 {
                ob.execute(OrderType::Limit {
                    id: i as u128,
                    price: 12345 + i as u64,
                    qty: i as u64,
                    side: Side::Bid,
                });
            }
        });
    });
}

fn medium_limit_ladder(c: &mut Criterion) {
    c.bench_function("small limit ladder (5000 levels)", |b| {
        let mut ob = OrderBook::default();
        b.iter(|| {
            for i in 0..5_000 {
                ob.execute(OrderType::Limit {
                    id: i as u128,
                    price: 12345 + i as u64,
                    qty: i as u64,
                    side: Side::Bid,
                });
            }
        });
    });
}

fn big_limit_ladder(c: &mut Criterion) {
    c.bench_function("big limit ladder (10000 levels)", |b| {
        let mut ob = OrderBook::default();
        b.iter(|| {
            for i in 0..10_000 {
                ob.execute(OrderType::Limit {
                    id: i as u128,
                    price: 12345 + i as u64,
                    qty: i as u64,
                    side: Side::Bid,
                });
            }
        });
    });
}

fn extreme_big_limit_ladder(c: &mut Criterion) {
    c.bench_function("extreme big limit ladder (1000000 levels)", |b| {
        let mut ob = OrderBook::default();
        b.iter(|| {
            for i in 0..1_000_000 {
                ob.execute(OrderType::Limit {
                    id: i as u128,
                    price: 12345 + i as u64,
                    qty: i as u64,
                    side: Side::Bid,
                });
            }
        });
    });
}

criterion_group!(
    benches,
    small_limit_ladder,
    medium_limit_ladder,
    big_limit_ladder,
    extreme_big_limit_ladder
);
criterion_main!(benches);

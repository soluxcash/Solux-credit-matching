use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solux::engine::MatchingEngine;
use solux::types::{Order, OrderSide};

fn benchmark_matching(c: &mut Criterion) {
    c.bench_function("simple_match", |b| {
        let mut engine = MatchingEngine::new();
        b.iter(|| {
          
            engine = MatchingEngine::new(); 
            
            let ask = Order::new(OrderSide::Lend, 1000, 500);
            let bid = Order::new(OrderSide::Borrow, 1000, 600);
            
            engine.place_order(black_box(ask));
            engine.place_order(black_box(bid));
        })
    });
}

criterion_group!(benches, benchmark_matching);
criterion_main!(benches);

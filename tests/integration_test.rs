use solux::engine::MatchingEngine;
use solux::types::{Order, OrderSide, Trade};

#[test]
fn test_partial_fill_scenario() {
    let mut engine = MatchingEngine::new();

    
    let lender_order = Order::new(OrderSide::Lend, 10_000, 500);
    engine.place_order(lender_order);

    
    let borrower_a = Order::new(OrderSide::Borrow, 4_000, 600);
    engine.place_order(borrower_a);

    
    let borrower_b = Order::new(OrderSide::Borrow, 6_000, 550);
    engine.place_order(borrower_b);

    
    let events = engine.drain_events();
    
    
    let match_events: Vec<&Trade> = events.iter().filter_map(|e| {
        if let solux::events::EngineEvent::OrderMatched(trade) = e {
            Some(trade)
        } else {
            None
        }
    }).collect();

    
    assert_eq!(match_events.len(), 2);
    assert_eq!(match_events[0].amount, 4000);
    assert_eq!(match_events[1].amount, 6000);

   
    assert!(engine.orderbook.bids.is_empty());
    assert!(engine.orderbook.asks.is_empty());
}

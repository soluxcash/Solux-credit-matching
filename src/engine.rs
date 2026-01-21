use crate::events::EngineEvent;
use crate::orderbook::OrderBook;
use crate::types::Order;

pub struct MatchingEngine {
    pub orderbook: OrderBook,
    pub event_queue: Vec<EngineEvent>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            orderbook: OrderBook::new(),
            event_queue: Vec::new(),
        }
    }

    pub fn place_order(&mut self, order: Order) {
        
        self.event_queue.push(EngineEvent::OrderPlaced(order.clone()));
        
        
        self.orderbook.add_order(order);
        
        
        self.process_matches();
    }

    fn process_matches(&mut self) {
        let trades = self.orderbook.match_orders();
        
        for trade in trades {
            self.event_queue.push(EngineEvent::OrderMatched(trade));
        }
    }

   
    pub fn drain_events(&mut self) -> Vec<EngineEvent> {
        self.event_queue.drain(..).collect()
    }
}

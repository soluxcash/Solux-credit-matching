use crate::types::{Order, OrderSide, Trade};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct OrderBook {
    
    pub bids: Vec<Order>, 
    
    pub asks: Vec<Order>, 
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            OrderSide::Borrow => self.bids.push(order),
            OrderSide::Lend => self.asks.push(order),
        }
        self.sort_book();
    }

    
    fn sort_book(&mut self) {
        
        self.bids.sort_by(|a, b| {
            b.interest_rate.cmp(&a.interest_rate)
                .then_with(|| a.timestamp.cmp(&b.timestamp)) 
        });

       
        self.asks.sort_by(|a, b| {
            a.interest_rate.cmp(&b.interest_rate)
                .then_with(|| a.timestamp.cmp(&b.timestamp)) 
        });
    }

   
    pub fn match_orders(&mut self) -> Vec<Trade> {
        let mut trades = Vec::new();

       
        while self.can_match() {
            let bid = &mut self.bids[0]; 
            let ask = &mut self.asks[0]; 

            
            let match_amount = std::cmp::min(bid.remaining_amount, ask.remaining_amount);

            
            let trade = Trade {
                id: Uuid::new_v4(),
                lend_order_id: ask.id,
                borrow_order_id: bid.id,
                amount: match_amount,
                
                interest_rate: ask.interest_rate, 
                timestamp: chrono::Utc::now().timestamp(),
            };
            trades.push(trade);

            
            bid.remaining_amount -= match_amount;
            ask.remaining_amount -= match_amount;

            
            if bid.remaining_amount == 0 {
                self.bids.remove(0);
            }
            
            if ask.remaining_amount == 0 {
                self.asks.remove(0);
            }
        }

        trades
    }

    
    fn can_match(&self) -> bool {
        if let (Some(bid), Some(ask)) = (self.bids.first(), self.asks.first()) {
            
            return bid.interest_rate >= ask.interest_rate;
        }
        false
    }
}

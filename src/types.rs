use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Lend,   
    Borrow, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub side: OrderSide,
    pub initial_amount: u64,
    pub remaining_amount: u64, 
    pub interest_rate: u64,    
    pub timestamp: i64,
}

impl Order {
    pub fn new(side: OrderSide, amount: u64, interest_rate: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            side,
            initial_amount: amount,
            remaining_amount: amount,
            interest_rate,
            timestamp: chrono::Utc::now().timestamp_micros(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub lend_order_id: Uuid,
    pub borrow_order_id: Uuid,
    pub amount: u64,
    pub interest_rate: u64,
    pub timestamp: i64,
}

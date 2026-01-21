use crate::error::SoluxError;
use crate::orderbook::OrderBook;
use crate::types::Order;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct OrderBookSnapshot {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

pub struct Persistence {
    path: String,
}

impl Persistence {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn save(&self, orderbook: &OrderBook) -> Result<(), SoluxError> {
        let snapshot = OrderBookSnapshot {
            bids: orderbook.bids.clone(),
            asks: orderbook.asks.clone(),
        };
        let data = serde_json::to_string_pretty(&snapshot)?;
        fs::write(&self.path, data)?;
        Ok(())
    }

    pub fn load(&self) -> Result<OrderBook, SoluxError> {
        if !Path::new(&self.path).exists() {
            return Ok(OrderBook::new());
        }

        let data = fs::read_to_string(&self.path)?;
        let snapshot: OrderBookSnapshot = serde_json::from_str(&data)?;

        Ok(OrderBook {
            bids: snapshot.bids,
            asks: snapshot.asks,
        })
    }
}

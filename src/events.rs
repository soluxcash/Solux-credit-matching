use crate::types::{Order, Trade};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineEvent {
    OrderPlaced(Order),
    OrderMatched(Trade),
    OrderFilledComplete(uuid::Uuid), 
    OrderCancelled(uuid::Uuid),
}

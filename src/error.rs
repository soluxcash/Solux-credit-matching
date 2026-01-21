use thiserror::Error;

#[derive(Error, Debug)]
pub enum SoluxError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization Error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Orderbook is locked or inaccessible")]
    LockError,
    
    #[error("Order not found")]
    OrderNotFound,
}

use std::time::{SystemTime, UNIX_EPOCH};
pub struct Block {
    id: u64,
    timestamp: i64,
    hash: String,
}
impl Block {
    pub fn new(id: u64) -> Self {
        Self {
            id,

            timestamp: SystemTime::now().duration_since(UNIX_EPOCH),
            hash: "test".to_string(),
        }
    }
}

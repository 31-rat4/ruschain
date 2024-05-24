use std::time::{SystemTime, UNIX_EPOCH};
pub struct Block {
    id: u64,
    pub timestamp: u64,
    hash: String,
}
impl Block {
    pub fn new(id: u64) -> Self {
        let timestamp: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(e) => {
                panic!("Error: {e:?}");
            }
        };
        Self {
            id,

            timestamp,
            hash: "test".to_string(),
        }
    }
}

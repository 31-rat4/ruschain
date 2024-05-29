use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct Block {
    id: u64,
    pub timestamp: u64,
    hash: String,
}
impl Block {
    pub fn new(id: u64) -> Self {
        sleep(Duration::new(2, 0));
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

use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis();

        let mut block = Self {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();

        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.prev_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn main() {
    println!("Hello, world!");
}

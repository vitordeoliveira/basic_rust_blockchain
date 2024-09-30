use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

#[derive(Debug)]
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

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.add_genesis_block("Genesis Block".to_string());
        blockchain
    }

    fn add_genesis_block(&mut self, data: String) {
        self.chain.push(Block::new(0, data, "genesis".to_string()));
    }

    fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain is empty")
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.get_latest_block();

        let new_block = Block::new(prev_block.index + 1, data, prev_block.hash.clone());

        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain.get(i).unwrap();
            let prev_block = &self.chain.get(i - 1).unwrap();

            if prev_block.hash != current_block.prev_hash {
                println!("prev_hash mismatch");
                return false;
            }

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
        }

        true
    }
}

fn main() {
    // Initialize the blockchain
    let mut blockchain = Blockchain::new();

    // Add some blocks
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());

    // Display the blockchain
    for block in &blockchain.chain {
        dbg!(block);
    }

    // Validate the blockchain
    if blockchain.is_valid() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is invalid!");
    }
}

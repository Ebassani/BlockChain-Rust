use std::time::SystemTime;
extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::Serialize;

#[derive(Serialize)]
pub struct Chain {
    blocks: Vec<Block>
}
impl Chain {
    pub fn new() -> Self {
        let mut blocks:Vec<Block> = Vec::new();

        let genesis_block = Block::new("Genesis Block".to_string(), None);

        blocks.push(genesis_block);

        Chain {
            blocks
        }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    pub fn last_block(&self) -> &Block {
        let block = self.blocks.last().unwrap();
        block
    }

    pub fn add_block(&mut self, data: Data) {
        let last_block_hash = String::from(self.last_block().get_hash());

        let new_block = Block::new(data.to_json_string().unwrap_or_default(), Some(last_block_hash));

        self.blocks.push(new_block);

    }
}

#[derive(Serialize)]
pub struct Block {
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}
impl Block {
    pub fn new(data: String, previous_hash: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs();

        let previous_hash = previous_hash.unwrap_or_default();
        let hash = calculate_hash(&data, &previous_hash, timestamp);

        Block {
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn _to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }
}

#[derive(Serialize)]
pub struct Data {
    amount: f64,
    sender: String,
    receiver: String,
}
impl Data {
    pub fn new(amount: f64, sender: String, receiver: String,) -> Self {
        Data {
            amount,
            sender,
            receiver
        }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn _get_sender_key(&self) -> &str {
        &self.sender
    }
}


/// # calculate_hash
/// This function receives the data as borrowed String(&str) alongside the hash from the
///  previous block and a timestamp of current time
fn calculate_hash(data: &str, previous_hash: &str, timestamp: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(data);
    if let Some(prev_hash) = Some(previous_hash) {
        hasher.input_str(prev_hash);
    }
    hasher.input_str(&timestamp.to_string());
    hasher.result_str()
}
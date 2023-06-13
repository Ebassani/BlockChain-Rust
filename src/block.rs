use std::time::SystemTime;
extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::Serialize;

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

fn calculate_hash(data: &str, previous_hash: &str, timestamp: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(data);
    if let Some(prev_hash) = Some(previous_hash) {
        hasher.input_str(prev_hash);
    }
    hasher.input_str(&timestamp.to_string());
    hasher.result_str()
}
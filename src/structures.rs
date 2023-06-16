extern crate crypto;
use std::{time::SystemTime};
use sha2::{Sha256, Digest};
use serde::Serialize;
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use rand::Rng;
use hex;
use tiny_keccak::keccak256;

#[derive(Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
    difficulty: usize
}
impl Chain {
    pub fn new() -> Self {
        let mut blocks:Vec<Block> = Vec::new();

        let genesis_block = Block::new("Genesis Block".to_string(), None);

        blocks.push(genesis_block);

        Chain {
            blocks,
            difficulty: 4
        }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    pub fn last_block(&self) -> &Block {
        let block = self.blocks.last().unwrap();
        block
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn mine(&mut self, data: Data) -> Block {
        let last_block_hash = String::from(self.last_block().get_hash());

        let mut block = Block::new(data.to_json_string().unwrap_or_default(),Some(last_block_hash));
        let target = std::iter::repeat('0').take(self.difficulty).collect::<String>();
    
        while block.hash[..self.difficulty] != target {
            block.nonce += 1;
            block.hash = calculate_hash(&block.data, &block.hash, block.timestamp);
        }

        let clone_block = block.clone();

        self.add_block(block);
    
        clone_block
    }
}

#[derive(Serialize, Clone)]
pub struct Block {
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
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
            nonce: 0
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
    signature: String
}
impl Data {
    pub fn new(amount: f64, sender: &str, _receiver: &str) -> Self {
        Data {
            amount,
            sender: sender.to_string(),
            receiver: sender.to_string(),
            signature: String::from("")
        }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn _get_sender_key(&self) -> &str {
        &self.sender
    }
    
    pub fn _get_receiver_key(&self) -> &str {
        &self.receiver
    }

    pub fn sign(&mut self, private_key: &str) {
        let secp = Secp256k1::new();
        let bytes = hex::decode(private_key).expect("Invalid private key");

        let secret_key = SecretKey::from_slice(&bytes)
            .expect("Invalid private key");

        let mut hasher = Sha256::new();

        hasher.update(&self.sender);
        hasher.update(&self.receiver);
        hasher.update(&self.amount.to_string());

        let result = hasher.finalize();

        let message = Message::from_slice(&result).expect("Invalid message hash");

        let signature = secp.sign(&message, &secret_key);

        self.signature = hex::encode(signature.serialize_compact().to_vec());
    }
}

#[derive(Serialize)]
pub struct Wallet {
    private_key: String,
    public_key: String,
    adress: String
}
impl Wallet {
    pub fn new() -> Wallet {
        let secp = Secp256k1::new();
        let bytes = rand::thread_rng().gen::<[u8; 32]>();
        
        let private_key = SecretKey::from_slice(&bytes)
            .expect("Invalid");

        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        let public_key = public_key.serialize_uncompressed();

        let adress = gen_adress(&public_key);

        Wallet { 
            private_key: hex::encode(&private_key[..]),
            public_key: hex::encode(public_key),
            adress
        }
    }

    pub fn get_public_key(&self) ->&str {
        &self.public_key
    }

    pub fn get_private_key(&self) ->&str {
        &self.private_key
    }

    pub fn get_adress(&self) ->&str {
        &self.adress
    }

    pub fn _to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}


/// # calculate_hash
///  This function receives the data as borrowed String(&str) alongside the hash from the
///  previous block and a timestamp of current time
fn calculate_hash(data: &str, previous_hash: &str, timestamp: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    if let Some(prev_hash) = Some(previous_hash) {
        hasher.update(prev_hash);
    }
    hasher.update(&timestamp.to_string());

    hex::encode(hasher.finalize())
}

fn gen_adress(public_key: &[u8; 65]) -> String {
    let hash = keccak256(&public_key[1..]);

    hex::encode(&hash[12..])
}
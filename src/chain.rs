use crate::block::Block;
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
    
}
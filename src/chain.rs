use crate::block::Block;
use crate::data::Data;
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
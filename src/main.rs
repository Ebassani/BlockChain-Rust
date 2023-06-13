mod block;
use crate::block::Block;

fn main() {
    let genesis_block = Block::new("Genesis Block".to_string(), None);
    print!("{}", genesis_block.to_json_string().unwrap());
}

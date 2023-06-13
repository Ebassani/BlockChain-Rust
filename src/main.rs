mod block;
use crate::block::Block;
mod data;
use crate::data::Data;

fn main() {
    let genesis_block = Block::new("Genesis Block".to_string(), None);
    println!("{}", genesis_block.to_json_string().unwrap());
    let first_transaction = Data::new(102.34, String::from("2345fwqyw567wqfd"), String::from("2345fwqyw567wqfd"));
    println!("{}", first_transaction.to_json_string().unwrap());
}

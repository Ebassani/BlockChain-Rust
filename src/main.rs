mod block;
mod data;
use crate::data::Data;
mod chain;
use crate::chain::Chain;

fn main() {
    let mut chain = Chain::new();
    
    let first_transaction = Data::new(102.34, String::from("2345fwqyw567wqfd"), String::from("2345fwqyw567wqfd"));
    
    chain.add_block(first_transaction);
    
    println!("{}", chain.to_json_string().unwrap());
}

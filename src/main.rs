mod structures;
use crate::structures::{Chain,Data};

fn main() {
    let mut chain = Chain::new();
    
    let first_transaction = Data::new(102.34, String::from("2345fwqyw567wqfd"), String::from("2345fwqyw567wqfd"));
    
    chain.add_block(first_transaction);
    
    println!("{}", chain.to_json_string().unwrap());
}

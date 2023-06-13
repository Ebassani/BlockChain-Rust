mod block;
mod data;
use crate::data::Data;
mod chain;
use crate::chain::Chain;

fn main() {
    let first_transaction = Data::new(102.34, String::from("2345fwqyw567wqfd"), String::from("2345fwqyw567wqfd"));
    println!("{}", first_transaction.to_json_string().unwrap());
    let chain = Chain::new();
    println!("{}", chain.to_json_string().unwrap());
}

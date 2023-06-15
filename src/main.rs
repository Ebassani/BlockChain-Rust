mod structures;
use crate::structures::{Chain,Data};

fn main() {
    let mut chain = Chain::new();
    
    let data = Data::new(102.34, String::from("2345fwqyw567wqfd"), String::from("2345fwqyw567wqfd"));
    
    chain.mine(data);
    
    println!("{}", chain.to_json_string().unwrap());
}

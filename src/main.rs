mod structures;
use crate::structures::{Chain, Data, Wallet};

fn main() {

    let sender = Wallet::new();
    let receiver = Wallet::new();

    let mut data = Data::new(102.34, sender.get_public_key(), receiver.get_public_key());

    data.sign(sender.get_private_key());
    let mut chain = Chain::new();
    
    chain.mine(data);
    
    println!("{}", chain.to_json_string().unwrap());
}

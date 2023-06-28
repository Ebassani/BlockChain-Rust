mod structures;
mod transaction;
use crate::structures::{Chain, Wallet};

fn main() {
    let mut wallets: Vec<Wallet> = Vec::new();

    wallets.push(Wallet::new());
    wallets.push(Wallet::new());

    let amount = 0.0000065;

    let data_option = transaction::send(wallets[0].get_public_key(), wallets[1].get_adress(), amount, &wallets);
    
    if data_option.is_some(){
        let mut data = data_option.unwrap();
        //data.sign(wallets[0].get_private_key());

        match data.sign(wallets[0].get_private_key()) {
            Ok(()) => println!("Signature generated successfully"),
            Err(err) => println!("Error: {}", err),
        }
        match transaction::verify_signature(&data) {
            Ok(()) => println!("Verified: true"),
            Err(_err) => println!("Verified: false")
        }

       

        let mut chain = Chain::new();
        
        chain.mine(data);
        
        println!("{}", chain.to_json_string().unwrap());
    }
    else {
        println!("Error");
    }
    
}

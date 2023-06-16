use crate::structures::{Data, Wallet};

pub fn send(sender_public_key: &str, receiver_adress: &str, amount: f64, wallets: &Vec<Wallet>) -> Option<Data> {
    let receiver =  wallets.iter().find(|&wallet| wallet.get_adress() == receiver_adress );

    if receiver.is_some() {
        let data = Data::new(amount, sender_public_key, receiver.unwrap().get_public_key());
        return Some(data)
    }
    
    None
}
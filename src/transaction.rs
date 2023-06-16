use sha2::{Sha256, Digest};
use secp256k1::{Secp256k1, Signature, PublicKey, Message};
use hex;
use crate::structures::{Data, Wallet};

pub fn send(sender_public_key: &str, receiver_adress: &str, amount: f64, wallets: &Vec<Wallet>) -> Option<Data> {
    let receiver =  wallets.iter().find(|&wallet| wallet.get_adress() == receiver_adress );

    if receiver.is_some() {
        let data = Data::new(amount, sender_public_key, receiver.unwrap().get_public_key());
        return Some(data)
    }
    
    None
}

pub fn verify_signature(data: &Data) -> bool {
    let secp = Secp256k1::new();

    let signature_bytes = hex::decode(data.get_signature()).expect("Invalid signature");
    let signature = Signature::from_compact(&signature_bytes).expect("Invalid signature");

    let mut hasher = Sha256::new();
    hasher.update(data.get_sender_key());
    hasher.update(data.get_receiver_key());
    hasher.update(data.get_amount().to_string());
    let result = hasher.finalize();
    let message = Message::from_slice(&result).expect("Invalid message hash");

    let public_key_bytes = hex::decode(data.get_sender_key()).expect("Invalid public key");
    let public_key = PublicKey::from_slice(&public_key_bytes).expect("Invalid public key");

    secp.verify(&message, &signature, &public_key).is_ok()
}
use serde::{Deserialize, Serialize};
use ed25519_dalek::{Keypair};
use rand::seq::{SliceRandom};
use rand::rngs::{OsRng};
use hex::{encode,decode};
use rand::Rng;
use std::fs;

pub fn generate_accounts(no_of_account: usize) {
    let mut csprng = OsRng{};
    let mut account_list = vec![];
    let mut count = 0u32;

    loop {
        count += 1;
        let key_pair = Keypair::generate(&mut csprng);
        // let secret = encode(key_pair.secret.to_bytes());
        let public_key = encode(key_pair.public.to_bytes());
        // let wallet = Wallet { private_key: secret, public_key: public };
        account_list.push(public_key);

        if count == no_of_account as u32 {
            break;
        }
    }

    let accounts_string = serde_json::to_string(&account_list).unwrap();
    fs::write("accounts.json",accounts_string).expect("Unable to write accounts");
    println!("{} accounts generated successfully", account_list.len());
}

pub fn get_random_pub_key() -> String {
    let mut csprng = OsRng{};
    let key_pair = Keypair::generate(&mut csprng);
    let public_key = encode(key_pair.public.to_bytes());

    public_key
}
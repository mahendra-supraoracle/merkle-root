// use rs_merkle::algorithms::Sha256;
// use rs_merkle::{Hasher, MerkleProof, MerkleTree};
// extern crate merk;
//extern crate merk;

mod account;

use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;
use merk::*;
use merk::test_utils::{put_entry, seq_key};
use std::ops::Range;
use std::time::Instant;
use crate::account::{generate_accounts, get_random_pub_key};
use rand::Rng;

use memmap2::Mmap;

fn main() {

    const total_account: usize = 1_000_000;
    const update_account_random: usize = 100_000;
    let file_name = "accounts.json";                    // list of account stored

    // generate_accounts(total_account);

    let account_file = File::open(file_name).expect("file should open read only");

    let bytes = unsafe {
        Mmap::map(&account_file).map_err(|e| format!("Error mmaping spec file : {}", e))
    };

    let list_of_accounts_json: serde_json::Value =
        serde_json::from_slice(&bytes.unwrap()).expect("file should be proper JSON");
    let accounts: Vec<String> =
        serde_json::from_str(list_of_accounts_json.to_string().as_str()).unwrap();

    assert_eq!(total_account, accounts.len());
    // println!("ACCOUNT LENGTH :: {}", accounts.len());

    let mut merk = Merk::open("./merk.db").unwrap();
                                                                                     //  println!("Prev Merkle Root Hash :: {:?}", merk.root_hash());
    let mut batch =  make_account_batches(accounts.clone());                  //  make_batch_seq(0..batch_size);


    assert_eq!(accounts.len(), batch.len());


    // START TIME
    let mut start = Instant::now();
    // merk.apply(&batch, &[]).expect("apply failed");

    unsafe { merk.apply_unchecked(&batch, &[]).expect("failed to appy unchecked"); }

    let mut end = start.elapsed().as_millis();

    println!("taken time {} to calculate merkle root for no of A/c {}", end, total_account);
    println!("merkle root hash {:?}", merk.root_hash());


    // let mut apply_batch =  Vec::new();
    // apply_batch.push(prepare_batch_entry(0 as u64, &"cf90feced1666157e1db6e234a97434c965ca34b898fd1daa8d582c04ebc0436".to_string()));           // SAME
    //
    // unsafe { merk.apply_unchecked(&apply_batch, &[]).expect("failed to apply second unchecked"); }


    // preparing random unique keys to update
    let update_account = random_update(update_account_random, total_account);

    start = Instant::now();
    unsafe { merk.apply_unchecked(&update_account, &[]).expect("failed to appy unchecked"); }
    end = start.elapsed().as_millis();

    println!("taken time {} to calculate merkle root for updating no of A/c(random) {}", end, update_account_random);
    println!("merkle root hash {:?}", merk.root_hash());
}

pub fn random_update(updating_account : usize, total_account : usize) -> Vec<BatchEntry> {

    let mut batch = Vec::new();
    let mut rng = rand::thread_rng();
    let mut count : usize = 0;

    let mut random_unique_number = Vec::new();

    loop {
        let position: u32 = rng.gen_range(0, total_account as u32);
        if !random_unique_number.contains(&position) {
            random_unique_number.push(position);
            count += 1;
        }
        if count == updating_account {
            break;
        }
    }

    for account_key in random_unique_number.iter()  {
        let account = get_random_pub_key();
        // println!("at position {} , updating A/c {:?}", account_key, account);

        batch.push(prepare_batch_entry(account_key.clone() as u64, &account));
    }

    batch
}

pub fn make_account_batches(accounts: Vec<String>) -> Vec<BatchEntry> {
    let mut batch = Vec::new();
    for (index, account) in accounts.iter().enumerate() {
        batch.push(prepare_batch_entry(index as u64, account));
    }
    batch
}

pub fn prepare_batch_entry(n: u64, account: &String) -> BatchEntry {
    (seq_key(n), Op::Put(account.clone().into_bytes()))
}

pub fn make_batch_seq(range: Range<u64>) -> Vec<BatchEntry> {
    let mut batch = Vec::new();
    for n in range {
        batch.push(put_entry(n));
    }
    batch
}























//
//
//
// use std::hash::Hash;
// use sparse_merkle_tree::{
//     blake2b::Blake2bHasher, default_store::DefaultStore,
//     error::Error, MerkleProof,
//     SparseMerkleTree, traits::Value, H256
// };
// use blake2b_rs::{Blake2b, Blake2bBuilder};
//
// // define SMT
// type SMT = SparseMerkleTree<Blake2bHasher, Word, DefaultStore<Word>>;
//
// // define SMT value
// #[derive(Default, Clone)]
// pub struct Word(String);
// impl Value for Word {
//     fn to_h256(&self) -> H256 {
//         if self.0.is_empty() {
//             return H256::zero();
//         }
//         let mut buf = [0u8; 32];
//         let mut hasher = new_blake2b();
//         hasher.update(self.0.as_bytes());
//         hasher.finalize(&mut buf);
//         buf.into()
//     }
//     fn zero() -> Self {
//         Default::default()
//     }
// }
//
// // helper function
// fn new_blake2b() -> Blake2b {
//     Blake2bBuilder::new(32).personal(b"SMT").build()
// }
//
// fn main() {
//
//     let mut tree = SMT::default();
//
//     let mut keys = Vec::new();
//     let mut values = Vec::new();
//
//     for (i, word) in "The quick brown fox jumps over the lazy dog"
//         .split_whitespace()
//         .enumerate()
//     {
//         let key: H256 = {
//             let mut buf = [0u8; 32];
//             let mut hasher = new_blake2b();
//             hasher.update(&(i as u32).to_le_bytes());
//             hasher.finalize(&mut buf);
//
//             buf.into()
//         };
//
//         keys.push(key);
//
//         println!("i : {} == word : {:?}", i, word);
//
//         let value = Word(word.to_string());
//
//         values.push(value.clone());
//
//         // insert key value into tree
//         tree.update( key, value).expect("update");
//
//     }
//
//     let mut tree_root = tree.root();
//
//     println!("SMT root is {:?} ", tree_root);
//
//     // Update leave value(keep same) at position 0
//     let mut new_value = Word("The".to_string());
//     tree.update(keys[0], new_value).expect("update");
//
//     let mut new_tree_root =  tree.root();
//
//


    //
    //
    // assert_eq!(tree_root.clone(), new_tree_root.clone());
    //
    // let mut latest_value = Word("They".to_string());
    //
    // tree.update(keys[0], latest_value).expect("update");
    //
    // let mut latest_tree_root =  tree.root();
    //
    // assert_ne!(tree_root, latest_tree_root);



    //
    // let leaf_values = ["a", "b", "c", "d", "e", "f"];
    // let leaves: Vec<[u8; 32]> = leaf_values
    //     .iter()
    //     .map(|x| Sha256::hash(x.as_bytes()))
    //     .collect();
    //
    // let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    // let indices_to_prove = vec![3, 4];
    // let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove")?;
    // let merkle_proof = merkle_tree.proof(&indices_to_prove);
    // let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root")?;
    // // Serialize proof to pass it to the client
    // let proof_bytes = merkle_proof.to_bytes();
    //
    // // Parse proof back on the client
    // let proof = MerkleProof::<Sha256>::try_from(proof_bytes);
    //
    // assert!(proof.verify(merkle_root, &indices_to_prove, leaves_to_prove, leaves.len()));


//     let elements = ["a", "b", "c", "d", "e", "f"];
//     let mut leaves: Vec<[u8; 32]> = elements
//         .iter()
//         .map(|x| Sha256::hash(x.as_bytes()))
//         .collect();
//
//     println!("LEAVE :: {:?}", leaves[0]);       //
//
//     let mut merkle_tree: MerkleTree<Sha256> = MerkleTree::new();
//
//     // Appending leaves to the tree without committing
//     merkle_tree.append(&mut leaves);
//
//     // Without committing changes we can get the root for the uncommitted data, but committed
//     // tree still doesn't have any elements
//     assert_eq!(merkle_tree.root(), None);
//     assert_eq!(
//         merkle_tree.uncommitted_root_hex(),
//         Some("1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2".to_string())
//     );
//
//     // Committing the changes
//     merkle_tree.commit();
//
//     // Changes applied to the tree after the commit, and there's no uncommitted changes anymore
//     assert_eq!(
//         merkle_tree.root_hex(),
//         Some("1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2".to_string())
//     );
//     assert_eq!(merkle_tree.uncommitted_root_hex(), None);
//
//     // Adding a new leaf
//     merkle_tree.insert(Sha256::hash("g".as_bytes())).commit();
//
//     // Root was updated after insertion
//     assert_eq!(
//         merkle_tree.root_hex(),
//         Some("e2a80e0e872a6c6eaed37b4c1f220e1935004805585b5f99617e48e9c8fe4034".to_string())
//     );
//
// // Adding some more leaves
//     merkle_tree.append(vec![
//         Sha256::hash("h".as_bytes()),
//         Sha256::hash("k".as_bytes()),
//     ].as_mut()).commit();
//     assert_eq!(
//         merkle_tree.root_hex(),
//         Some("09b6890b23e32e607f0e5f670ab224e36af8f6599cbe88b468f4b0f761802dd6".to_string())
//     );
//
// // Rolling back to the previous state
//     merkle_tree.rollback();
//     assert_eq!(
//         merkle_tree.root_hex(),
//         Some("e2a80e0e872a6c6eaed37b4c1f220e1935004805585b5f99617e48e9c8fe4034".to_string())
//     );
//
// // We can rollback multiple times as well
//     merkle_tree.rollback();
//     assert_eq!(
//         merkle_tree.root_hex(),
//         Some("1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2".to_string())
//     );
//
//     merkle_tree.
// }

// use std::env;
#![allow(dead_code, unused)]

pub mod tree;

use crate::transactions::tx::{self, Tx, TxInput, TxOutput};
use crate::mempool::mempool::Mempool;
use serde_json;
use sha2::digest::core_api::TruncSide;
use sha2::{Sha256, Sha256VarCore};
use sha256::{digest, try_digest};
use std::path::Path;
use std::io::{self, Read};
use std::fs::{self, File};
use std::str::FromStr;
use std::thread::sleep;
use std::time;
use ripemd::{Ripemd160, Digest};
use hex;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey, ecdsa};
// use libsecp256k1::{Message, PublicKey, Signature, PublicKeyFormat};


pub fn get_merkle_root(tx_vec: &Vec<Tx>) -> String {
    let merkle_root: String = String::new();

    for tx in tx_vec{

    }

    String::new()
}

pub fn validate_transaction_inputs(tx: &Tx) -> Vec<String> {
    let mut script: Vec<String> = vec![];
    for vin in &tx.tx_input {
        let mut scriptsig: Vec<String> = vec![];
        scriptsig.extend(vin.scriptsig_asm.clone().split_whitespace().map(String::from));

        let signature: String = scriptsig[1].clone();
        let pubkey: String = scriptsig[3].clone();

        script.push(signature);
        script.push(pubkey);
        script.extend(vin.prevout.scriptpubkey_asm.clone().split_whitespace().map(String::from));
    }
    
    let mut stack: Vec<String> = vec![];

    println!("Starting the Script Stack");

    for instruction in &script{
        match instruction.as_str() {
            "OP_DUP" => {
                println!("OP_DUP de {}", stack.last().unwrap());
                stack.push(stack.last().unwrap().clone());
                println!("{:?}\n", stack);
            }
            "OP_HASH160" => {
                let value: String = stack.pop().unwrap();

                let mut value_hash = hash_160(&value);

                println!("OP_HASH160 de {} virando {}", value, value_hash);

                stack.push(value_hash);
                println!("{:?}\n", stack);
            }
            "OP_EQUALVERIFY" => {
                let last_stack_value = stack.pop().unwrap();
                let pubkey = stack.pop().unwrap();
                if last_stack_value != pubkey {
                    stack.push("OP_0".to_string());
                }
            }
            "OP_CHECKSIG" => {
                println!("{:?}", stack);
                
                let secp = Secp256k1::new();

                let pubkey_pop = stack.pop().unwrap();

                // let pubkey_test = PublicKey::parse_slice(&hex::decode(&pubkey_pop).expect("Error while decoding pubkey to hex"), Some(PublicKeyFormat::Compressed)).expect("Error while parsing pubkey");
                
                let pubkey_hex = hex::decode(pubkey_pop).expect("Error while decoding pubkey to hex");

                let pubkey = PublicKey::from_slice(&pubkey_hex).expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");

                println!("{:?}", stack);

                let signature = stack.pop().unwrap();

                // println!("Signature {}", signature);
                // println!("Signature len {}", &hex::decode(&signature).expect("Error while decoding signature to hex").len());

                let mut signature_hex =  hex::decode(&signature).expect("Error while decoding signature to hex");

                signature_hex.remove(signature_hex.len() - 1);

                // let sig_test = Signature::parse_der(&signature_hex).expect("Error while parsing signature");

                // println!("sig_test {:?}", sig_test);

                let signature = ecdsa::Signature::from_der(&signature_hex).expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");

                println!("signature {}", signature);

                let message_pop = tx.tx_input[0].txid.clone();
                
                // println!("message_pop len {}", message_pop.len());

                let mut message_hex = hex::decode(&message_pop).expect("Error while decoding message to hex");
                // println!("message hex {:?}", message_hex);
                // message_hex.reverse();
                // println!("message hex {:?}", message_hex);

                // println!("Address {}", hash_160(&pubkey_pop));

                // let message_test = Message::parse_slice(&message_hex).expect("Error while parsing message");

                let message = Message::from_digest_slice(&message_hex).expect("messages must be 32 bytes and are expected to be hashes");

                
                // println!("tx op_checksig {}", libsecp256k1::verify(&message_test, &sig_test, &pubkey_test));
                println!("{}", secp.verify_ecdsa(&message, &signature, &pubkey).unwrap_err());
                // assert!();
            }
            instruction if instruction.starts_with("OP_PUSHBYTES_") => {
                println!("Pushing bytes");
            }
            _ => {
                println!("Inserindo item na lista");
                stack.push(instruction.clone());
                println!("{:?}\n", stack);
            } 
        }
        // sleep(time::Duration::from_secs(1));
    }
    
    script
}

pub fn hash_160(value: &String) -> String {

    let hex_value = hex::decode(value).expect("Error while parsing hex from string");

    let mut hasher256 = Sha256::new();

    hasher256.update(hex_value);

    let result_256 = hasher256.finalize();

    let mut hasher_ripemd = Ripemd160::new();

    hasher_ripemd.update(result_256);

    let result_ripemd = hasher_ripemd.finalize();

    let hash160_string: String = result_ripemd.clone().iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    println!("\nHash RIPEMD160 de {} => {}\n", value, hash160_string);

    hash160_string
}

pub fn read_mempool(path: &str) -> Mempool {

    // let files = get_files_in_directory(path)
    //     .expect("Error while reading Dir");
    
    // let tx = read_tx_from_file("/home/gabriel/projects/bitcoin-mining-challenge/mempool/0.json");

    let mut mempool: Mempool = Mempool::get_mempool_from_dir(path);

    // println!("{:?}", mempool);

    // println!("==================== Before sorting ====================");
    // for i in 0..100{
    //     println!("{}", mempool.txs[i]);
    // }

    // println!("==================== After sorting ====================");
    // mempool.sort_mempool_by_tx();
    // for i in 0..100{
    //     println!("{}", mempool.txs[i]);
    // }
    mempool
}

pub fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;
    
    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
    .filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_file() {
            path.file_name()?.to_str().map(|s| s.to_owned())
        } else {
            None
        }
    })
    .collect();

    Ok(file_names)
}

pub fn read_tx_from_file(file_path: &str) -> Tx {
    let mut file_content: String = String::new();
    let path = Path::new(&file_path);
    
    let mut file = File::open(path).expect("Error while loading file");
    file.read_to_string(&mut file_content).expect("File can not be read");
    
    let error_message_in_file:String = String::from("Error while parsing json to Tx in file ") + file_path;
    
    let tx_in_json: Tx = serde_json::from_str(&file_content).expect(&error_message_in_file);
    
    return tx_in_json;
}
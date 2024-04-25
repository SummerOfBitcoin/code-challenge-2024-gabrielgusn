// use std::env;
#![allow(dead_code, unused)]

mod transactions;
// use core::panicking::panic;
use std::io::{self, Read};
use std::fs::{self, File};
use std::path::Path;
use serde_json;
use transactions::tx::{Tx, TxInput, TxOutput};

pub fn read_mempool(path: &str){

    let files = get_files_in_directory(path)
        .expect("Error while reading Dir");
    
    let tx = read_tx_from_file("/home/gabriel/projects/bitcoin-mining-challenge/mempool/0.json");

    let mut mempool: Vec<Tx> = vec![];

    for file in files{
        let file_path = path.to_string() + &file;

        let transaction: Tx = read_tx_from_file(&file_path);

        mempool.push(transaction);
    }

    let mut teste: String = String::from("teste");
    teste.push('a');

    println!("teste eh {}", teste);

    println!("Before sorting");
    println!("Fee:{}", mempool[0].get_tx_fee());
    println!("Fee:{}", mempool[1].get_tx_fee());
    println!("Fee:{}", mempool[2].get_tx_fee());

    mempool.sort_by_key(|tx| tx.get_tx_fee());
    mempool.reverse();
    println!("After sorting");
    println!("Fee:{}", mempool[0].get_tx_fee());
    println!("Fee:{}", mempool[1].get_tx_fee());
    println!("Fee:{}", mempool[2].get_tx_fee());
    // is_coinbase(tx_in_json);
}

pub fn read_tx_from_file(file_path: &str) -> Tx {
    let mut file_content: String = String::new();
    let path = Path::new(&file_path);

    let mut file = File::open(path).expect("Error while loading file");
    file.read_to_string(&mut file_content).expect("File can not be read");

    let error_in_file_message:String = String::from("Error while parsing json to Tx in file ") + file_path;
    
    let tx_in_json: Tx = serde_json::from_str(&file_content).expect(&error_in_file_message);

    return tx_in_json;
}

fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
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
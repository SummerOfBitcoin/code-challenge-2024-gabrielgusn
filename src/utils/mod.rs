// use std::env;
#![allow(dead_code, unused)]

// use core::panicking::panic;

use crate::transactions::tx::{Tx, TxInput, TxOutput};
use crate::mempool::mempool::Mempool;
use serde_json;
use std::path::Path;
use std::io::{self, Read};
use std::fs::{self, File};

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
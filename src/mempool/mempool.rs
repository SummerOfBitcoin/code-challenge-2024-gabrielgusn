
#![allow(dead_code, unused)]

use std::fmt::Formatter;
use core::fmt;
use crate::transactions::tx::Tx;
use crate::utils::{read_tx_from_file, get_files_in_directory};
// use crate::get_files_in_directory;

#[derive(Debug)]
pub struct Mempool {
    pub txs: Vec<Tx>,    
}

impl Mempool{
    pub fn new() -> Self {
        let mempool_txs: Vec<Tx> = vec![];
        Self {
            txs: mempool_txs,
        }
    }
    
    pub fn get_mempool_from_dir(dir_path: &str) -> Mempool {
        let files: Vec<String> = get_files_in_directory(&dir_path).expect("Error while reading mempool directory");
        
        let mut mempool: Mempool = Mempool::new();
        
        for file  in files{

            let file_path: String = dir_path.to_string() + &file;
            let tx: Tx = read_tx_from_file(&file_path);

            if tx.pre_validate_tx() {
                mempool.txs.push(tx);
            }
        }
        
        return mempool;
    }

    pub fn sort_mempool_by_tx(&mut self) {
        self.txs.sort_by(|a, b| a.get_tx_fee_per_bit().partial_cmp(&b.get_tx_fee_per_bit()).unwrap());
    }
    
}
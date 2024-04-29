
#![allow(dead_code, unused)]

use std::fmt::Formatter;
use core::fmt;
use crate::transactions::tx::Tx;
use crate::utils::{read_tx_from_file, get_files_in_directory};
// use crate::get_files_in_directory;

const MAX_BLOCK_SIZE: u32 = 8000000;
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
        println!("{}", dir_path);
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
    
    pub fn get_transactions_vec(&self) -> Vec<Tx> {
        let mut tx_vec: Vec<Tx> = vec![];
        let mut vec_size: u32 = 0;
        for tx in &self.txs{
            if tx.get_tx_size_in_bits() < MAX_BLOCK_SIZE - vec_size {
                tx_vec.push(tx.clone());
                vec_size += tx.get_tx_size_in_bits();
                println!("{}", tx);
            }
        }

        return tx_vec;
    }
}

#![allow(dead_code, unused)]

use crate::transactions::tx::Tx;
use mining_challenge::{get_files_in_directory};

pub struct Mempool {
    mempool_txs: Vec<MempoolTx>,    
}

impl Mempool{
    pub fn new(mempool_txs: Vec<MempoolTx>) -> Self {
        Self {
            mempool_txs,
        }
    }
    
}

pub fn get_mempool_from_dir(dir_path: &str) -> Mempool {
    let files: Vec<String> = get_files_in_directory(dir_path).expect("Error while reading mempool directory");

    let mut mempool: Vec<MempoolTx> = vec![];

    for file in files{
        let file_path = path.to_string() + &file;

        let transaction: Tx = read_tx_from_file(&file_path);

        let mempool_tx: MempoolTx = MempoolTx::new(transaction);

        mempool.push(mempool_tx);
    }

    return mempool;
}

struct MempoolTx {
    default_tx: Tx,
    tx_size: u64,
    tx_fee: u64,
}

impl MempoolTx{
    pub fn new(tx: Tx) -> Self {
        Self {
            default_tx,
            tx_size: tx.get_tx_size_in_bits(),
            tx_fee: tx.get_tx_fee(),
        }
    }
}
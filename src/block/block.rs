#![allow(dead_code, unused)]

use super::block_header::BlockHeader;
use crate::transactions::tx::Tx;
use core::fmt;
#[derive(Debug)]
pub struct Block{
    pub block_header: BlockHeader,
    // TO DO: still need to define how we will represent the transactions in the block
    pub transactions: Vec<Tx>,
}

impl Block {

    pub fn new(block_header: BlockHeader, transactions: Vec<Tx>) -> Self {
        Self{
            block_header,
            transactions,
        }
    }

    fn get_block_header_hash(&self) -> String {
        self.block_header.get_block_header_sha256sum()
    }

    pub fn get_block_size(&self) -> u64 {
    let mut block_size: u64 = 0;
    for tx in &self.transactions{
        block_size += tx.get_tx_size_in_bits();
    }

    return block_size;
    }

    pub fn proof_of_work(&mut self){
    loop {
        if self.block_header.get_block_header_sha256sum() < self.block_header.target_hash {
            println!("\tFound the nonce for the target Hash! It is: {} and you can attach this block to the blockchain\n\t Block Hash is: {}\n", self.block_header.nonce, self.block_header.get_block_header_sha256sum());
            break
        }
        // println!("Not yet! ({})", self.block_header.get_block_header_sha256sum());
        self.block_header.nonce += 1;
    }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        write!(
            f,
            "Block ID: {}\nMerkle Root: {}\nTimestamp: {}\nNonce: {}\n\tTransactions: {:?}",
            self.block_header.block_id,
            self.block_header.txs_merkle_root,
            self.block_header.timestamp,
            self.block_header.nonce,
            self.transactions,
        )
    }
}
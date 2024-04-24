#![allow(dead_code, unused)]

use super::block_header::BlockHeader;
use crate::transactions::tx::{Tx};

#[derive(Debug)]
struct Block{
    block_header: BlockHeader,
    // TO DO: still need to define how we will represent the transactions in the block
    transactions: Vec<Tx>,
}

impl Block {
     fn get_block_header_hash() -> String {
        String::from("Hello World!")
     }

     pub fn get_block_size(&self) -> u64 {
        let mut block_size: u64 = 0;
        for tx in &self.transactions{
            block_size += tx.get_tx_size_in_bits();
        }

        return block_size;
     }
}
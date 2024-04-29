#![allow(dead_code, unused)]

use super::block_header::BlockHeader;
use crate::{mempool::mempool::Mempool, transactions::{self, tx::Tx}};
use core::fmt;

const BLOCK_MAX_SIZE: u32 = 8000000;

#[derive(Debug, Clone)]
pub struct Block{
    pub block_header: BlockHeader,
    pub transactions: Vec<Tx>,
    pub block_size: u32,
}

impl Block {

    pub fn new(block_header: BlockHeader, transactions: Vec<Tx>) -> Self {
        Self{
            block_header: block_header.clone(),
            transactions,
            block_size: block_header.get_block_header_size() + 32,
        }
    }

    pub fn insert_transactions_from_mempool(&mut self, mempool: &mut Mempool) {
        loop {
            if mempool.txs.len() > 0{
                if mempool.txs.last().unwrap().get_tx_size_in_bits() < self.get_block_size_left(){
                    self.push_transaction(mempool.txs.pop().unwrap());
                    println!("Txs Left on Mempool: {}", mempool.txs.len());
                }
                else {
                    break;
                }        
            }
        }
    }

    pub fn calculate_total_block_fee(&self) -> u64 {
        let mut total_fee: u64 = 0;

        for tx in &self.transactions{
            total_fee += tx.get_tx_fee();
        }

        return total_fee;
    }

    pub fn get_block_size_left(&self) -> u32 {
        BLOCK_MAX_SIZE - self.block_size
    }

    pub fn push_transaction(&mut self, tx: Tx) {
        if self.block_size + tx.get_tx_size_in_bits() <= BLOCK_MAX_SIZE {
            self.block_size += tx.get_tx_size_in_bits();
            self.transactions.push(tx);
        }
        else {
            println!("It is not possible to attach this tx to the block\n\tThe Tx Size is {} and the remaining space in the block is {}", tx.get_tx_size_in_bits(), self.block_size);
        }
    }

    fn get_block_header_hash(&self) -> String {
        self.block_header.get_block_header_sha256sum()
    }

    pub fn get_block_size(&self) -> u32 {
        let mut actual_block_size: u32 = self.block_size;

        for tx in &self.transactions{
            actual_block_size += tx.get_tx_size_in_bits();
        }

        return actual_block_size;
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
            "Block ID: {}\nMerkle Root: {}\nTimestamp: {}\nNonce: {}\n\tAmount of Transactions: {:?}\n\tTotal Fee: {}\n\tBlock Size in KB: {} KBs\n\tBlock Size in Bits: {} Bits",
            self.block_header.block_id,
            self.block_header.txs_merkle_root,
            self.block_header.timestamp,
            self.block_header.nonce,
            self.transactions.len(),
            self.calculate_total_block_fee(),
            (self.get_block_size() / 8 / 1000),
            self.get_block_size()
        )
    }
}
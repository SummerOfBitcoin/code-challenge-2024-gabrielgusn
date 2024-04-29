#![allow(dead_code, unused)]
mod block;
mod transactions;
mod mempool;
mod utils;
use block::block_header::BlockHeader;
use block::block::Block;
use chrono::Utc;
use utils::{read_mempool, read_tx_from_file};
use std::fs::File;
use std::io::prelude::*;
use mempool::mempool::Mempool;
use std::fs;

use crate::utils::validate_transaction_inputs;

fn main() {
    // let file_size = fs::metadata("/home/gabriel/projects/bitcoin-mining-challenge/mempool/ff73248e38bfcdac87261f5a51f3143478937fda6b0d1def46e794b8f250e6dd.json").expect("Falha ao ler o arquivo");
    // println!("Size: {} ", file_size.len());
    let mut mempool: Mempool = read_mempool("/home/gabriel/projects/bitcoin-mining-challenge/mempool/");
    mempool.sort_mempool_by_tx();

    let genesis_block_header: BlockHeader = BlockHeader::new(String::from("00000000000000000000000000000000"), String::from("00000000000000000000000000000000"), Utc::now(), 128);
    let mut genesis_block = Block::new(genesis_block_header.clone(), vec![]);

    println!("{}", genesis_block.get_block_size());

    let mut block_chain: Vec<Block> = vec![];
    
    println!("Mempool Size Before: {}", mempool.txs.len());

    genesis_block.insert_transactions_from_mempool(&mut mempool);
    block_chain.push(genesis_block.clone());
    println!("Mempool Size After: {}", mempool.txs.len());
    let tx = read_tx_from_file("/home/gabriel/projects/bitcoin-mining-challenge/mempool/0a8b21af1cfcc26774df1f513a72cd362a14f5a598ec39d915323078efb5a240.json");
    // let tx = read_tx_from_file("/home/gabriel/projects/bitcoin-mining-challenge/mempool/0ac528562a1626863c0cb912eb725530c54e786e6485380c16633e4b9bce1720.json");
    
    validate_transaction_inputs(&tx);

    let mut file: File = File::create("output.txt").expect("Error while creating file");

    file.write_all(genesis_block.block_header.get_block_header_sha256sum().as_bytes());
    file.write_all(b"\n");
    
    // while mempool.txs.len() > 6000 {
    //         let mut new_block_header: BlockHeader = BlockHeader::new(block_chain.last().unwrap().block_header.get_block_header_sha256sum(), String::from("00000000000000000000000000000000"), Utc::now(), 0);
    //         let mut new_block: Block = Block::new(new_block_header, vec![]);
    //         // if mempool.txs.last().unwrap().get_tx_size_in_bits() < new_block.get_block_size_left(){
    //         //     new_block.push_transaction(mempool.txs.pop().unwrap());
    //         //     println!("Txs Left on Mempool: {}", mempool.txs.len());
    //         // }
    //         // else {
    //         //     break;
    //         // }      
    //         println!("Inserting Txs on Block {}", new_block.block_header.block_id);
    //         new_block.insert_transactions_from_mempool(&mut mempool);
    //         block_chain.push(new_block);
    // }

    // println!("{}", block_chain.len());

    // for block in block_chain{
    //     println!("{}", block);
    // }
    
    // println!("=================================");
    // println!("{}", first_block_header);
    // println!("=================================");
    // println!("1st Block Hash: {}", first_block_header.get_block_header_sha256sum());
    // println!("Size of Block Header: {}", std::mem::size_of::<BlockHeader>());

    // let mut block_vec: Vec<Block> = vec![];

    // block_vec.push(block);

    // loop {

    // }

    // for i in 0..10{

    //     println!("Generating Block {}", i+1);

    //     let block_header = BlockHeader::new(block_vec[i].block_header.get_block_header_sha256sum(), String::from("00000000000000000000000000000000"), Utc::now(), 0);

    //     let new_block = Block::new(block_header, vec![]);
    //     block_vec.push(new_block);
    // }


    // for mut block in block_vec {
    //     println!("{}", block);
    //     block.proof_of_work()
    // }

    // for mut block in block_veu ec{
        
    // }

    // loop{

    //     let mut context: Context = Context::new(&SHA256);
        
    //     // let target_hash: String = String::from("0000ffff00000000000000000000000000000000000000000000000000000000");
    //     let target_hash: String = String::from("00000cff00000000000000000000000000000000000000000000000000000000");

    //     context.update(data.as_bytes());
    //     context.update(&nonce.to_be_bytes());
        
    //     let digest: Digest = context.finish();
    //     let mut actual_hex: String = String::new();
        
    //     for &byte in digest.as_ref() {
    //         write!(&mut actual_hex, "{:02x}", byte).expect("Failed to write hex");
    //     }
        
    //     println!("Nonce: {}, Hash: {}", nonce, actual_hex);

    //     if actual_hex <= target_hash {
    //         println!("Found the nonce for the target Hash! It is: {} and you can attach this block to the blockchain", nonce);
    //         break
    //     }
    //     nonce += 1;
    // }
}

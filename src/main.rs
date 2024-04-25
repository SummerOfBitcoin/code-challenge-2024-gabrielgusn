#![allow(dead_code, unused)]
mod block;
mod transactions;
use block::block_header::BlockHeader;
use block::block::Block;
use chrono::Utc;
use mining_challenge::{read_mempool, read_tx_from_file};
use ring::digest::{Context, Digest, SHA256};
use std::fmt::Write;
// use hex_literal::hex;
use std::fs;
// use transactions::tx;

fn main() {
    // let file_size = fs::metadata("/home/gabriel/projects/bitcoin-mining-challenge/mempool/ff73248e38bfcdac87261f5a51f3143478937fda6b0d1def46e794b8f250e6dd.json").expect("Falha ao ler o arquivo");
    // println!("Size: {} ", file_size.len());
    read_mempool("/home/gabriel/projects/bitcoin-mining-challenge/mempool/");

    // let mut hasher = Sha256::new();
    let first_block_header: BlockHeader = BlockHeader::new(String::from("00000000000000000000000000000000"), String::from("00000000000000000000000000000000"), Utc::now(), 128);
    let block = Block::new(first_block_header, vec![]);

    // println!("=================================");
    // println!("{}", first_block_header);
    // println!("=================================");
    // println!("1st Block Hash: {}", first_block_header.get_block_header_sha256sum());
    // println!("Size of Block Header: {}", std::mem::size_of::<BlockHeader>());

    let mut block_vec: Vec<Block> = vec![];

    block_vec.push(block);

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



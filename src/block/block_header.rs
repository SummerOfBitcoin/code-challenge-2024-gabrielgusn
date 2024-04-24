//This file is responsible for implementing the basic structure for the Block's Header

/*  Directive used to stop rust compiles pointing unused code as warnings, otherwise, 
    it would place a warning in the lines 14-17 because we are not actually using the value of 
    theses items.
*/
#![allow(dead_code, unused)]

// Rust's dependencies used to implement the structure
// core::fmt for implementing the custom Display for this struct, customizing the way it is printed in stdout
use core::fmt;
// chrono::... is being used for the `timestamp` value in the header
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/*  As we use ::fmt, we need the Debug trait in the struct and it can be automatically 
    implemented using derive()
*/
#[derive(Debug)]
pub struct BlockHeader {
    // block_id is intended to store the sha256 hash of the previous block's header, as in bitcoin
    pub block_id: String, //[u8; 32],
    // the block's transactions merkle root, intended also to store a sha256 hash
    // both the block_id and txs_merkle_root content is garanteed to store a 32 bytes value
    // in the "new()" method
    pub txs_merkle_root: String, //[u8; 32],
    // intended to include information about the time the block was mined
    pub timestamp: DateTime<Utc>,
    pub target_hash: String, //[u8; 32],
    // value to be adjusted in order to find the pre image for the sha256 hash below the target 
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new(block_id: String, txs_merkle_root: String, timestamp: DateTime<Utc>, nonce: u32) -> Self {
        Self {
            block_id, 
            txs_merkle_root,
            target_hash: String::from("0000ffff00000000000000000000000000000000000000000000000000000000"),
            timestamp,
            nonce
        }
    }

    pub fn get_block_header_sha256sum(&self) -> String{
        let serialized_header = format!("{:?}", self);
        let mut hasher = Sha256::new();

        hasher.update(serialized_header.as_bytes());

        let result = hasher.finalize();

        let hash_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        return hash_string;
    }

}

impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block ID: {}\nMerkle Root: {}\nTimestamp: {}\nNonce: {}",
            self.block_id,
            self.txs_merkle_root,
            self.timestamp,
            self.nonce
        )
    }
}
use sha2::{Digest, Sha256, Sha256VarCore};

pub fn hash_sha256(value: String) -> String {
    
    let bytes = value.as_bytes();

    let mut hasher = Sha256::new();

    hasher.update(bytes);

    let result = hasher.finalize();

    println!("{:?}", result);
}
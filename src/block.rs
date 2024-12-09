use sha2::{Sha256, Digest};

pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new(index: u32, previous_hash: &str, data: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", index, previous_hash, data).as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp: 0,
            previous_hash: previous_hash.to_string(),
            hash,
            data: data.to_string(),
        }
    }
}

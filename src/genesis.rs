use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct GenesisBlock {
    pub index: u32,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

pub fn create_genesis_block(config: &super::CoinConfig) -> GenesisBlock {
    let data = format!("{} - {}", config.genesis_message, config.name);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    GenesisBlock {
        index: 0,
        timestamp: 0,
        previous_hash: String::from("0"),
        hash,
        data,
    }
}

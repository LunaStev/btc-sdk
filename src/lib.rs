pub mod genesis;
pub mod transaction;
pub mod block;
pub mod network;
pub mod db;

pub struct CoinConfig {
    pub name: String,
    pub total_supply: u64,
    pub block_time: u64,
    pub genesis_message: String,
}

impl CoinConfig {
    pub fn new(name: &str, total_supply: u64, block_time: u64, genesis_message: &str) -> Self {
        Self {
            name: name.to_string(),
            total_supply,
            block_time,
            genesis_message: genesis_message.to_string(),
        }
    }

    pub fn generate_genesis_block(&self) {
        let genesis_block = genesis::create_genesis_block(&self);
        println!("🚀 Genesis Block Created: {:?}", genesis_block);
    }
}

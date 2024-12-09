# btc-sdk: Bitcoin-like Blockchain Library
This project is a Rust library that allows you to build a Bitcoin-like blockchain system. The library provides the basic structure of a blockchain and includes the ability to create a genesis block. It provides the foundational framework for implementing basic blockchain networks and transaction systems.

## Project Overview
With this library, you can create your own blockchain system and define the basic features of your own coin, similar to Bitcoin. It allows you to create the genesis block, store blockchain data, and define the basic configuration of the coin.

## Implemented Features
The major features currently implemented are as follows:

* Genesis Block Creation: The CoinConfig struct allows you to define the basic coin configuration, and the generate_genesis_block method generates the genesis block.
* Blockchain Database Management: The BlockchainDB struct is used to store and load the blockchain data.
* Basic Block Linking: You can add new blocks to the blockchain, and the blocks are connected through hash-based links.
* Node Connection Setup: Basic P2P network initialization code is included, preparing the system for connecting with other nodes.

## Usage
To use this library, you need to first add it as a dependency in your `Cargo.toml` file.

### 1. Adding the Dependency
In your `Cargo.toml`, add the following:

```toml
btc-sdk = "0.1.0"
```

### 2. Genesis Block Creation
You can create the genesis block by setting up the `CoinConfig` and `BlockchainDB`, then calling the `generate_genesis_block` method.
```rust
use btc_sdk::{CoinConfig, BlockchainDB};

fn main() {
    // Define coin configuration
    let config = CoinConfig {
        name: "MyCoin".to_string(),
        symbol: "MC".to_string(),
        supply: 1000000,
        initial_reward: 50,
        initial_difficulty: 1,
    };

    // Generate the genesis block
    let genesis_block = config.generate_genesis_block();
    
    // Initialize the blockchain database
    let mut db = BlockchainDB::new();
    
    // Add the genesis block to the database
    db.add_block(&genesis_block);
    
    println!("Genesis Block: {:?}", genesis_block);
}
```

### 3. Blockchain Database Storage and Loading
You can store blocks in the blockchain database and load them later.

```rust
// Initialize the blockchain database
let mut db = BlockchainDB::new();

// Create a new block
let new_block = Block::new(1, "previous_block_hash", "data", "block_hash");

// Add the new block to the database
db.add_block(&new_block);

// Load all blocks from the blockchain database
let blocks = db.get_blocks();
println!("{:?}", blocks);
```

## Implemented Features
### 1. Genesis Block Creation
* The CoinConfig struct is used to define the basic configurations of the coin.
* The generate_genesis_block method creates the first genesis block.
* The genesis block includes fundamental information such as supply, name, symbol, initial reward, and difficulty.
### 2. Blockchain Database
* The BlockchainDB struct is used for storing blockchain data.
* Blocks are linked via hashes, ensuring the integrity of the blockchain.
* Blocks can be added to and loaded from the blockchain database.
### 3. Basic Block Linking
* Blocks are connected via hash values, linking each new block to the previous one.
* When new blocks are added, they are stored in the blockchain and linked together.
### 4. P2P Network Preparation
* Basic code for initializing the P2P network is included.
* This allows the preparation of the node to connect with other nodes. Currently, the network connection setup is provided, but the actual data exchange and synchronization have not yet been implemented.

## What Can You Build with This?
With this library, you can build a basic blockchain network and, by extending it, create your own cryptocurrency. For example:

* Create Your Own Cryptocurrency: Generate your own coin with the genesis block and store blockchain data to manage your coin's lifecycle.
* Implement P2P Network: Extend the library to synchronize blocks across nodes and build a distributed blockchain network.
* Transaction System: Implement transaction creation, signing, and verification, and add transactions to blocks to create a working transaction system.
* Mining System: Add mining functionality to create new blocks and reward miners with coins.
* Smart Contracts (optional): Implement smart contracts that run on the blockchain and enable user-defined logic.

## Contributing
This project is open-source! If you'd like to contribute, please submit a Pull Request or open an Issue to suggest improvements.
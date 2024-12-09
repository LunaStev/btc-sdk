use secp256k1::{Secp256k1, SecretKey, Message};
use sha2::{Sha256, Digest};

pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
}

pub struct TransactionOutput {
    pub value: u64,
    pub address: String,
}

pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }

    pub fn sign(&self, private_key: &SecretKey) -> Vec<u8> {
        let secp = Secp256k1::new();

        let message = b"transaction_data";
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash = hasher.finalize();

        let msg = Message::from_slice(&hash).expect("Failed to create message");

        let sig = secp.sign_ecdsa_low_r(&msg, private_key);

        sig.serialize_compact().to_vec()
    }
}

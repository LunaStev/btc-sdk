use secp256k1::{Secp256k1, SecretKey, PublicKey};

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
        let sig = secp.sign_ecdsa_recoverable(&message.into(), private_key);
        sig.serialize_compact().to_vec()
    }
}

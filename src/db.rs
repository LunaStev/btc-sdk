use rocksdb::{DB, Options};

pub struct BlockchainDB {
    pub db: DB,
}

impl BlockchainDB {
    pub fn new(path: &str) -> Self {
        let db = DB::open_default(path).unwrap();
        BlockchainDB { db }
    }

    pub fn write_block(&self, key: &str, value: &str) {
        self.db.put(key, value).unwrap();
    }

    pub fn read_block(&self, key: &str) -> Option<String> {
        match self.db.get(key) {
            Ok(Some(value)) => Some(String::from_utf8(value).unwrap()),
            _ => None,
        }
    }
}

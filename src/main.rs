use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: chrono::Utc::now().timestamp(),
            data: String::from("Genesis Block"),
            previous_hash: String::from("0"),
            hash: String::new(),
            nonce: 0,
        };
        let hash = self.hash_block(&genesis_block);
        let block_with_hash = Block { hash, ..genesis_block };
        self.chain.push(block_with_hash);
    }

    fn hash_block(&self, block: &Block) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}{}",
            block.index, block.timestamp, &block.data, &block.previous_hash, block.nonce
        );
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block {
            index: previous_block.index + 1,
            timestamp: chrono::Utc::now().timestamp(),
            data,
            previous_hash: previous_block.hash.clone(),
            hash: String::new(),
            nonce: 0,
        };
        self.proof_of_work(&mut new_block);
        let hash = self.hash_block(&new_block);
        new_block.hash = hash;
        self.chain.push(new_block);
    }

    fn proof_of_work(&self, block: &mut Block) {
        while !self.is_valid_proof(block) {
            block.nonce += 1;
        }
    }

    fn is_valid_proof(&self, block: &Block) -> bool {
        let hash = self.hash_block(block);
        hash.starts_with("0000") // Example of a simple proof of work validation (difficulty)
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != self.hash_block(current_block) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.mine_block("Transaction 1".to_string());
    blockchain.mine_block("Transaction 2".to_string());
    blockchain.mine_block("Transaction 3".to_string());
    blockchain.mine_block("Transaction 4".to_string());
    blockchain.mine_block("Transaction 5".to_string());
    blockchain.mine_block("Transaction 6".to_string());

    for block in blockchain.chain.iter() {
        println!("{:#?}", block);
    }

    println!("Is chain valid? {}", blockchain.is_chain_valid());
}
